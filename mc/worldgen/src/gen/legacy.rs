use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::cell::{RefCell, RefMut};
use std::sync::Arc;

use crossbeam_channel::{Sender, Receiver, unbounded, bounded};

use mc_core::world::source::{LevelSource, ChunkLoadRequest, LevelSourceError, ProtoChunk};
use mc_core::world::chunk::{ChunkResult, ChunkError, ChunkStatus, Chunk};
use mc_core::world::level::LevelEnv;
use mc_core::heightmap::HeightmapType;
use mc_core::block::BlockState;
use mc_core::biome::Biome;
use mc_core::util::Rect;
use mc_core::perf;

use crate::view::{LevelView, ProtoChunkView};


/// Trait for terrain generators.
pub trait TerrainGenerator {
    type Chunk: ProtoChunkView;
    fn generate(&mut self, chunk: ProtoChunk) -> Self::Chunk;
}

/// Trait for feature generators.
pub trait FeatureGenerator {
    type Chunk: ProtoChunkView;
    fn decorate(&mut self, level: QuadLevelView<Self::Chunk>, cx: i32, cz: i32, x: i32, z: i32);
}

/// Base trait for a temporary provider of terrain and feature generators. Structures
/// implementing this trait should also be `Sync` and `Send` because they will be shared
/// between threads in order to call its methods inside the thread. This allows terrain
/// and feature generators not to be Send and/or Sync, which is not required because these
/// generators will not be moved outside the thread.
pub trait GeneratorProvider {
    type Terrain: TerrainGenerator;
    type Feature: FeatureGenerator;
    fn build_terrain(&self) -> Self::Terrain;
    fn build_feature(&self) -> Self::Feature;
}


/// A common threaded generator level source that generate terrain and features,
/// this generator works in two major steps, terrain and decoration generation.
///
/// This source is a generator that uses multiple terrain workers and a single
/// feature worker. Proto chunk are created by terrain workers and then
/// decorated by feature worker and then queued waiting for polling.
///
/// The following diagram explain how workers are connected through channels:
/// ```text
/// ┌────────────┐       ┌───────────────────┐
/// │   Source   ├─┬─────► Terrain Worker #0 ├─┐
/// └─▲──────────┘ │     └───────────────────┘ │
///   │            │ load request              │
///   │            │     ┌───────────────────┐ │
///   │            └─────► Terrain Worker #N ├─┤
///   │                  └───────────────────┘ │
///   │ full                                   │
///   │ chunk ┌────────────────┐ terrain chunk │
///   └───────┤ Feature Worker ◄───────────────┘
///           └────────────────┘
/// ```
pub struct LegacyGenLevelSource {
    request_sender: Sender<ChunkLoadRequest>,
    chunk_receiver: Receiver<ProtoChunk>,
    loading_chunks: HashSet<(i32, i32)>
}

impl LegacyGenLevelSource {

    /// Construct a new legacy generator with the given number of terrain workers (threads).
    /// For now there is only a single worker for features generation, this might change in
    /// the future.
    pub fn new<P, C>(provider: P, terrain_workers: u16) -> Self
    where
        P: GeneratorProvider + Send + Sync + 'static,
        C: ProtoChunkView + Send + 'static,
        P::Terrain: TerrainGenerator<Chunk = C>,
        P::Feature: FeatureGenerator<Chunk = C>,
    {

        let (
            request_sender,
            request_receiver
        ) = unbounded();

        let (
            terrain_sender,
            terrain_receiver
        ) = bounded(256);

        let (
            chunk_sender,
            chunk_receiver
        ) = bounded(256);

        let provider = Arc::new(provider);

        for i in 0..terrain_workers {
            let request_receiver = request_receiver.clone();
            let terrain_sender = terrain_sender.clone();
            let provider = Arc::clone(&provider);
            std::thread::Builder::new()
                .name(format!("Legacy Generator Terrain Worker #{}", i))
                .spawn(move || {
                    TerrainWorker {
                        request_receiver,
                        terrain_sender,
                        generator: provider.build_terrain()
                    }.run()
                }).unwrap();
        }

        std::thread::Builder::new()
            .name("Legacy Generator Feature Worker".to_string())
            .spawn(move || {
                FeatureWorker {
                    chunks: HashMap::new(),
                    chunks_counters: HashMap::new(),
                    terrain_receiver,
                    chunk_sender,
                    generator: provider.build_feature(),
                }.run()
            }).unwrap();

        Self {
            request_sender,
            chunk_receiver,
            loading_chunks: HashSet::new()
        }

    }

}

impl LevelSource for LegacyGenLevelSource {

    fn request_chunk_load(&mut self, req: ChunkLoadRequest) -> Result<(), (LevelSourceError, ChunkLoadRequest)> {
        // We ensure that all surrounding chunk are also loaded.
        for cx in (req.cx - 1)..=(req.cx + 1) {
            for cz in (req.cz - 1)..=(req.cz + 1) {
                if !self.loading_chunks.contains(&(cx, cz)) {
                    self.loading_chunks.insert((cx, cz));
                    let mut req = req.clone();
                    req.cx = cx;
                    req.cz = cz;
                    self.request_sender.send(req).unwrap();
                }
            }
        }
        Ok(())
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>> {
        match self.chunk_receiver.try_recv() {
            Ok(chunk) => {
                self.loading_chunks.remove(&chunk.get_position());
                Some(Ok(chunk))
            },
            _ => None
        }
    }

}


/// Internal thread worker for terrain generation, this is the first step in the generation
/// process. Another thread is responsible of the features generation.
struct TerrainWorker<G: TerrainGenerator> {
    request_receiver: Receiver<ChunkLoadRequest>,
    terrain_sender: Sender<G::Chunk>,
    generator: G,
}

impl<G: TerrainGenerator> TerrainWorker<G> {

    fn run(mut self) {
        loop {
            match self.request_receiver.recv() {
                Err(_) => break,
                Ok(req) => {
                    perf::push("gen_terrain");
                    let proto_chunk = self.generator.generate(req.build_proto_chunk());
                    self.terrain_sender.send(proto_chunk).unwrap();
                    perf::pop();
                    // perf::debug();
                },
            }
        }
    }

}


/// Internal thread worker
struct FeatureWorker<G: FeatureGenerator> {
    chunks: HashMap<(i32, i32), RefCell<G::Chunk>>,
    chunks_counters: HashMap<(i32, i32), u8>,
    terrain_receiver: Receiver<G::Chunk>,
    chunk_sender: Sender<ProtoChunk>,
    generator: G
}

impl<G: FeatureGenerator> FeatureWorker<G> {

    fn run(mut self) {
        loop {
            match self.terrain_receiver.recv() {
                Err(_) => break,
                Ok(chunk) => {

                    perf::push("gen_feature");

                    let (cx, cz) = chunk.as_chunk_ref().get_position();
                    // println!("[{}] Decorating {}/{}", std::thread::current().name().unwrap(), cx, cz);

                    self.chunks.insert((cx, cz), RefCell::new(chunk));

                    // Here we increment counters for each surrounding chunks ('n' for neighbor).
                    for ncx in (cx - 1)..=(cx + 1) {
                        for ncz in (cz - 1)..=(cz + 1) {

                            let count = match self.chunks_counters.entry((ncx, ncz)) {
                                Entry::Occupied(o) => {
                                    let count = o.into_mut();
                                    *count += 1;
                                    *count
                                },
                                Entry::Vacant(v) => *v.insert(1)
                            };

                            // When all surrounding (including itself) have a loaded terrain,
                            // let's generate features for this chunk.
                            if count == 9 {

                                perf::push("generating");

                                let order = match (ncx & 1, ncz & 1) {
                                    (0, 0) => [(1, 1), (1, 0), (0, 0), (0, 1)],
                                    (0, 1) => [(1, 0), (1, 1), (0, 1), (0, 0)],
                                    (1, 0) => [(0, 1), (0, 0), (1, 0), (1, 1)],
                                    (1, 1) => [(0, 0), (0, 1), (1, 1), (1, 0)],
                                    _ => unreachable!()
                                };

                                // println!(" => Ready to be decorated... {}/{} {:?}", ncx, ncz, (ncx & 1, ncz & 1));

                                for (dx, dz) in order {

                                    perf::push("generating_feature");

                                    // Chunk coordinates of the chunk with lowest x/z of the 4
                                    // chunks for the QuadLevelView.
                                    let (ocx, ocz) = (ncx + dx - 1, ncz + dz - 1);
                                    // Block coordinates of the feature chunk.
                                    let (block_x, block_z) = ((ocx << 4) + 8, (ocz << 4) + 8);

                                    if let (
                                        Some(c00),
                                        Some(c10),
                                        Some(c01),
                                        Some(c11)
                                    ) = (
                                        self.chunks.get(&(ocx + 0, ocz + 0)),
                                        self.chunks.get(&(ocx + 1, ocz + 0)),
                                        self.chunks.get(&(ocx + 0, ocz + 1)),
                                        self.chunks.get(&(ocx + 1, ocz + 1))
                                    ) {

                                        // println!(" => Decorating feature chunk {}/{}", ocx, ocz);

                                        let view = QuadLevelView {
                                            chunks: [
                                                c00.borrow_mut(), c10.borrow_mut(),
                                                c01.borrow_mut(), c11.borrow_mut()
                                            ],
                                            ocx,
                                            ocz
                                        };

                                        self.generator.decorate(view, ocx, ocz, block_x, block_z);

                                    }

                                    perf::pop();

                                };

                                // When a chunk is decorated, remove it from maps, it should never
                                // be queried again because all surrounding chunks have received
                                // its decoration.
                                self.chunks_counters.remove(&(ncx, ncz));
                                let chunk = self.chunks.remove(&(ncx, ncz)).unwrap().into_inner();
                                let mut proto_chunk = chunk.into_inner();
                                proto_chunk.set_status(ChunkStatus::Full);
                                self.chunk_sender.send(proto_chunk).unwrap();

                                perf::pop();

                            }

                        }
                    }

                    perf::pop();

                }
            }
        }
    }

}





/// An implementation of `LevelView` (from feature module) that refers to a quad of 4 chunks,
/// used to generate one feature chunk.
pub struct QuadLevelView<'a, C: ProtoChunkView> {
    /// Ordering is 0/0 1/0 0/1 1/1 (X then Z)
    chunks: [RefMut<'a, C>; 4],
    ocx: i32,
    ocz: i32
}

impl<'a, C: ProtoChunkView> QuadLevelView<'a, C> {

    #[inline]
    fn get_chunk_index(&self, cx: i32, cz: i32) -> ChunkResult<usize> {
        let dx = cx - self.ocx;
        let dz = cz - self.ocz;
        if dx >= 0 && dz >= 0 && dx < 2 && dz < 2 {
            Ok(dx as usize + dz as usize * 2)
        } else {
            Err(ChunkError::ChunkUnloaded)
        }
    }

    #[inline]
    fn get_chunk_at_index(&self, x: i32, z: i32) -> ChunkResult<usize> {
        self.get_chunk_index(x >> 4, z >> 4)
    }

}

impl<'a, C: ProtoChunkView> LevelView for QuadLevelView<'a, C> {

    fn get_env(&self) -> &Arc<LevelEnv> {
        self.chunks[0].as_chunk_ref().get_env()
    }

    fn get_chunk(&self, cx: i32, cz: i32) -> Option<&Chunk> {
        let idx = self.get_chunk_index(cx, cz).ok()?;
        self.chunks.get(idx).map(|proto| proto.as_chunk_ref())
    }

    fn get_chunk_mut(&mut self, cx: i32, cz: i32) -> Option<&mut Chunk> {
        let idx = self.get_chunk_index(cx, cz).ok()?;
        self.chunks.get_mut(idx).map(|proto| proto.as_chunk_mut())
    }

    fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()> {
        self.chunks[self.get_chunk_at_index(x, z)?].set_block_at(x, y, z, state)
    }

    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState> {
        self.chunks[self.get_chunk_at_index(x, z)?].get_block_at(x, y, z)
    }

    fn get_biome_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static Biome> {
        self.chunks[self.get_chunk_at_index(x, z)?].get_biome_at(x, y, z)
    }

    fn get_heightmap_column_at(&self, heightmap_type: &'static HeightmapType, x: i32, z: i32) -> ChunkResult<i32> {
        self.chunks[self.get_chunk_at_index(x, z)?].get_heightmap_column_at(heightmap_type, x, z)
    }

}


/// This is an implementation of the trait `ProtoChunkView`, this implementation does not
/// use the chunk's biome methods but uses the `legacy_biomes` field of this structure which
/// is actually a rectangle of biomes of 16x16 (minimum required size).
pub struct LegacyProtoChunk {
    /// The real (modern) proto chunk.
    pub inner: ProtoChunk,
    /// The legacy 16x16 biomes rectangle.
    pub legacy_biomes: Rect<&'static Biome>
}

impl LegacyProtoChunk {

    pub fn get_legacy_biome(&self, x: u8, z: u8) -> &'static Biome {
        *self.legacy_biomes.get(x as usize, z as usize)
    }

}

impl ProtoChunkView for LegacyProtoChunk {

    fn into_inner(self) -> ProtoChunk {
        self.inner
    }

    #[inline]
    fn as_chunk_ref(&self) -> &Chunk {
        &*self.inner
    }

    #[inline]
    fn as_chunk_mut(&mut self) -> &mut Chunk {
        &mut *self.inner
    }

    #[inline]
    fn get_position(&self) -> (i32, i32) {
        self.inner.get_position()
    }

    fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()> {
        self.inner.set_block_at(x, y, z, state)
    }

    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState> {
        self.inner.get_block_at(x, y, z)
    }

    fn get_biome_at(&self, x: i32, _y: i32, z: i32) -> ChunkResult<&'static Biome> {
        Ok(*self.legacy_biomes.get((x & 15) as usize, (z & 15) as usize))
    }

    fn get_heightmap_column_at(&self, heightmap_type: &'static HeightmapType, x: i32, z: i32) -> ChunkResult<i32> {
        self.inner.get_heightmap_column_at(heightmap_type, x, z)
    }

}
