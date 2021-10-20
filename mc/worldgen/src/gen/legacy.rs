use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, RwLock, Arc};

use crossbeam_channel::{Sender, Receiver, unbounded, bounded};

use mc_core::world::source::{LevelSource, ChunkLoadRequest, LevelSourceError, ProtoChunk};


/// A common threaded generator level source that generate legacy terrain.
pub struct LegacyGeneratorLevelSource {
    load_request_sender: Sender<ChunkLoadRequest>,
    loading_chunks: HashSet<(i32, i32)>
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
                    self.load_request_sender.send(req).unwrap();
                }
            }
        }
        Ok(())
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>> {
        match self.answer_receiver.try_recv() {
            Ok(Answer::Terrain(chunk)) => {
                let (cx, cz) = chunk.get_position();
                let mut chunks = self.chunks.write().unwrap();
                chunks.insert((cx, cz), Mutex::new(chunk));
                self.load_request_sender.send(Request::Feature(cx, cz)).unwrap();
            },
            Ok(Answer::Feature(chunk)) => {

            },
            Err(_) => None
        }
        todo!()
    }

}


/// Internal thread worker for terrain generation, this is the first step in the generation
/// process. Another thread is responsible of the features generation.
struct TerrainWorker {
    request_receiver: Receiver<ChunkLoadRequest>,
    terrain_sender: Sender<ProtoChunk>
}

impl TerrainWorker {

    fn run(mut self) {
        loop {
            match self.request_receiver.recv() {
                Err(_) => break,
                Ok(req) => {
                    self.terrain_sender.send(self.generate_terrain(req)).unwrap();
                },
            }
        }
    }

    fn generate_terrain(&self, req: ChunkLoadRequest) -> ProtoChunk {
        todo!()
    }

}


struct FeatureWorker {
    chunks: HashMap<(i32, i32), ProtoChunk>,
    chunks_counters: HashMap<(i32, i32), u8>,
    terrain_receiver: Receiver<ProtoChunk>
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