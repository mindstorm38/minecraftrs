use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, RwLock, Arc};

use crossbeam_channel::{Sender, Receiver, unbounded, bounded};

use mc_core::world::source::{LevelSource, ChunkLoadRequest, LevelSourceError, ProtoChunk};


/// A common threaded generator level source that generate legacy terrain.
///
/// The following diagram explain how workers are connected through channels:
/// ```text
/// ┌────────────┐       ┌───────────────────┐
/// │   Source   ├─┬─────► Terrain Worker #0 ├─┐
/// └─▲──────────┘ │     └───────────────────┘ │
///   │            │ load request              │
///   │            │     ┌───────────────────┐ │
///   │            └─────► Terrain Worker #1 ├─┤
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
    pub fn new(terrain_workers: u16) -> Self {

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

        for i in 0..terrain_workers {

            let worker = TerrainWorker {
                request_receiver: request_receiver.clone(),
                terrain_sender: terrain_sender.clone()
            };

            std::thread::Builder::new()
                .name(format!("Legacy Generator Terrain Worker #{}", i))
                .spawn(move || worker.run());

        }

        let feature_worker = FeatureWorker {
            chunks: HashMap::new(),
            chunks_counters: HashMap::new(),
            terrain_receiver,
            chunk_sender,
        };

        std::thread::Builder::new()
            .name("Legacy Generator Feature Worker".to_string())
            .spawn(move || feature_worker.run());

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
        /*match self.answer_receiver.try_recv() {
            Ok(Answer::Terrain(chunk)) => {
                let (cx, cz) = chunk.get_position();
                let mut chunks = self.chunks.write().unwrap();
                chunks.insert((cx, cz), Mutex::new(chunk));
                self.load_request_sender.send(Request::Feature(cx, cz)).unwrap();
            },
            Ok(Answer::Feature(chunk)) => {

            },
            Err(_) => None
        }*/
        todo!()
    }

}


/// Internal thread worker for terrain generation, this is the first step in the generation
/// process. Another thread is responsible of the features generation.
struct TerrainWorker {
    request_receiver: Receiver<ChunkLoadRequest>,
    terrain_sender: Sender<Box<ProtoChunk>>
}

impl TerrainWorker {

    fn run(mut self) {
        loop {
            match self.request_receiver.recv() {
                Err(_) => break,
                Ok(req) => {
                    self.terrain_sender.send(Box::new(self.generate_terrain(req))).unwrap();
                },
            }
        }
    }

    fn generate_terrain(&self, req: ChunkLoadRequest) -> ProtoChunk {
        todo!()
    }

}


/// Internal thread worker
struct FeatureWorker {
    chunks: HashMap<(i32, i32), ProtoChunk>,
    chunks_counters: HashMap<(i32, i32), u8>,
    terrain_receiver: Receiver<Box<ProtoChunk>>,
    chunk_sender: Sender<ProtoChunk>
}

impl FeatureWorker {

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



                            }

                        }
                    }

                }
            }
        }
    }

    fn generate_features(&self, cx: i32, cz: i32) {

    }

}