use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::cell::{RefCell, RefMut};
use std::sync::Arc;

use crossbeam_channel::{Sender, Receiver, unbounded, bounded};

use mc_core::world::source::{LevelSource, ChunkLoadRequest, LevelSourceError, ProtoChunk};
use mc_core::world::chunk::{ChunkResult, ChunkError};
use mc_core::block::BlockState;
use crate::feature::LevelView;


/// Trait for terrain generators.
pub trait TerrainGenerator {
    fn generate(&mut self, chunk: &mut ProtoChunk);
}

/// Trait for feature generators.
pub trait FeatureGenerator {
    fn decorate(&mut self, level: QuadLevelView, x: i32, y: i32, z: i32);
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


/// A common threaded generator level source that generate legacy terrain.
/// This source is a generator that uses multiple terrain workers and a single
/// feature worker. Proto chunk are created by terrain workers and then
/// decorate by feature worker and then queued waiting for poll.
///
/// The following diagram explain how workers are connected through channels:
/// ```text
/// ┌────────────┐       ┌───────────────────┐
/// │   Source   ├─┬─────► Terrain Worker #0 ├─┐
/// └─▲──────────┘ │     └───────────────────┘ │
///   │            │ load request              │
///   │            │     ┌───────────────────┐ │
///   │            └─────► Terrain Worker #n ├─┤
///   │                  └───────────────────┘ │
///   │ full                                   │
///   │ chunk ┌────────────────┐ terrain chunk │
///   └───────┤ Feature Worker ◄───────────────┘
///           └────────────────┘
/// ```
pub struct LegacyGeneratorLevelSource {
    request_sender: Sender<ChunkLoadRequest>,
    chunk_receiver: Receiver<ProtoChunk>,
    loading_chunks: HashSet<(i32, i32)>
}

impl LegacyGeneratorLevelSource {

    /// Construct a new legacy generator with the given number of terrain workers (threads).
    /// For now there is only a single worker for features generation, this might change in
    /// the future.
    pub fn new<P>(provider: P, terrain_workers: u16) -> Self
    where
        P: GeneratorProvider + Send + Sync + 'static
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

impl LevelSource for LegacyGeneratorLevelSource {

    fn request_chunk_load(&mut self, req: ChunkLoadRequest) -> Result<(), (LevelSourceError, ChunkLoadRequest)> {
        // We ensure that all surrounding chunk are also loaded.
        for cx in (req.cx - 1)..=(req.cx + 1) {
            for cz in (req.cz - 1)..=(req.cz + 1) {
                if !self.loading_chunks.contains(&(cx, cz)) {
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
        self.chunk_receiver.recv().ok().map(|c| Ok(c))
    }

}


/// Internal thread worker for terrain generation, this is the first step in the generation
/// process. Another thread is responsible of the features generation.
struct TerrainWorker<G: TerrainGenerator> {
    request_receiver: Receiver<ChunkLoadRequest>,
    terrain_sender: Sender<ProtoChunk>,
    generator: G
}

impl<G: TerrainGenerator> TerrainWorker<G> {

    fn run(mut self) {
        loop {
            match self.request_receiver.recv() {
                Err(_) => break,
                Ok(req) => {
                    let mut proto_chunk = req.build_proto_chunk();
                    self.generator.generate(&mut proto_chunk);
                    self.terrain_sender.send(proto_chunk).unwrap();
                },
            }
        }
    }

}


/// Internal thread worker
struct FeatureWorker<G: FeatureGenerator> {
    chunks: HashMap<(i32, i32), RefCell<ProtoChunk>>,
    chunks_counters: HashMap<(i32, i32), u8>,
    terrain_receiver: Receiver<ProtoChunk>,
    chunk_sender: Sender<ProtoChunk>,
    generator: G
}

impl<G: FeatureGenerator> FeatureWorker<G> {

    fn run(mut self) {
        loop {
            match self.terrain_receiver.recv() {
                Err(_) => break,
                Ok(chunk) => {

                    let (cx, cz) = chunk.get_position();

                    // Here we increment counters for each surrounding chunks.
                    for dcx in (cx - 1)..=(cx + 1) {
                        for dcz in (cz - 1)..=(cz + 1) {

                            let count = match self.chunks_counters.entry((dcx, dcz)) {
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

                                let order = match (cx & 1, cz & 1) {
                                    (0, 0) => [(1, 1), (1, 0), (0, 0), (0, 1)],
                                    (0, 1) => [(1, 0), (1, 1), (0, 1), (0, 0)],
                                    (1, 0) => [(0, 1), (0, 0), (1, 0), (1, 1)],
                                    (1, 1) => [(0, 0), (0, 1), (1, 1), (1, 0)],
                                    _ => unreachable!()
                                };

                                for (dx, dz) in order {

                                    // Chunk coordinates of the chunk with lowest x/z of the 4
                                    // chunks for the QuadLevelView.
                                    let (ocx, ocz) = (cx + dx - 1, cz + dz - 1);
                                    // Block coordinates of the feature chunk.
                                    let (center_x, center_z) = ((cx + dx) << 4, (cz + dz) << 4);
                                    let (block_x, block_z) = (center_x - 8, center_z - 8);

                                    let view = QuadLevelView {
                                        chunks: [
                                            self.chunks.get(&(ocx + 0, ocz + 0)).unwrap().borrow_mut(),
                                            self.chunks.get(&(ocx + 1, ocz + 0)).unwrap().borrow_mut(),
                                            self.chunks.get(&(ocx + 0, ocz + 1)).unwrap().borrow_mut(),
                                            self.chunks.get(&(ocx + 1, ocz + 1)).unwrap().borrow_mut()
                                        ],
                                        x_start: ocx << 4,
                                        z_start: ocz << 4
                                    };

                                    self.generator.decorate(view, block_x, 0, block_z);

                                };

                                // When a chunk is decorated, remove it from maps, it should never
                                // be queried again because all surrounding chunks have received
                                // its decoration.
                                self.chunks_counters.remove(&(cx, cz));
                                let chunk = self.chunks.remove(&(cx, cz)).unwrap().into_inner();
                                self.chunk_sender.send(chunk).unwrap();

                            }

                        }
                    }

                }
            }
        }
    }

}


pub struct QuadLevelView<'a> {
    /// Ordering is 0/0 1/0 0/1 1/1 (X then Z)
    chunks: [RefMut<'a, ProtoChunk>; 4],
    x_start: i32,
    z_start: i32
}

impl<'a> QuadLevelView<'a> {

    #[inline(always)]
    fn get_chunk_index(&self, x: i32, z: i32) -> ChunkResult<usize> {
        let dx = (x - self.x_start) >> 4;
        let dz = (z - self.z_start) >> 4;
        if dx >= 0 && dz >= 0 && dx < 2 && dz < 2 {
            Ok(dx as usize + dz as usize * 2)
        } else {
            Err(ChunkError::ChunkUnloaded)
        }
    }

}

impl<'a> LevelView for QuadLevelView<'a> {

    fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()> {
        self.chunks[self.get_chunk_index(x, z)?].set_block_at(x, y, z, state)
    }

    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState> {
        self.chunks[self.get_chunk_index(x, z)?].get_block_at(x, y, z)
    }

}
