use std::thread::Builder as ThreadBuilder;
use std::time::{Instant, Duration};
use std::path::{PathBuf, Path};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crossbeam_channel::{Sender, Receiver, RecvTimeoutError, unbounded, bounded};

use crate::world::source::{LevelSource, LevelSourceError, ChunkLoadRequest, ProtoChunk, ChunkSaveRequest};
use crate::util::TimedCache;
use crate::debug;

use super::region::{RegionFile, RegionResult, RegionError, calc_region_pos};
use super::decode::{decode_chunk_from_reader};
use super::encode::{encode_chunk_to_writer};


enum Request {
    Load(ChunkLoadRequest),
    Save(ChunkSaveRequest)
}


/// A level source that load chunks from anvil region files. This source internally use
/// a threaded worker to avoid disk access durations overhead. Each opened region file
/// remains opened for `REGIONS_CACHE_TIME` duration.
pub struct AnvilLevelSource {
    request_sender: Sender<Request>,
    result_receiver: Receiver<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>>
}

impl AnvilLevelSource {

    pub fn new<P: AsRef<Path>>(dir: P) -> Self {

        let (
            request_sender,
            request_receiver
        ) = unbounded();

        let result_receiver = Worker::new(
            dir.as_ref().join("region"),
            request_receiver
        );

        Self {
            request_sender,
            result_receiver
        }

    }

}

impl LevelSource for AnvilLevelSource {

    fn request_chunk_load(&mut self, req: ChunkLoadRequest) -> Result<(), (LevelSourceError, ChunkLoadRequest)> {
        // SAFETY: Unwrap should be safe because the channel is unbounded.
        self.request_sender.send(Request::Load(req)).unwrap();
        Ok(())
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>> {
        self.result_receiver.try_recv().ok()
    }

    fn request_chunk_save(&mut self, req: ChunkSaveRequest) -> Result<(), LevelSourceError> {
        self.request_sender.send(Request::Save(req)).unwrap();
        Ok(())
    }

}


const REGIONS_CACHE_TIME: Duration = Duration::from_secs(60);
const REGIONS_REQUEST_RECV_TIMEOUT: Duration = Duration::from_secs(30);

struct Worker {
    regions_dir: PathBuf,
    request_receiver: Receiver<Request>,
    result_sender: Sender<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>>,
    regions: HashMap<(i32, i32), TimedCache<RegionFile>>,
    last_cache_check: Instant
}

impl Worker {

    /// Internal constructor for worker, you must give the regions directory, not level directory.
    fn new(
        regions_dir: PathBuf,
        request_receiver: Receiver<Request>
    ) -> Receiver<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>> {

        let (
            result_sender,
            result_receiver
        ) = bounded(128);

        let worker = Self {
            regions_dir,
            request_receiver,
            result_sender,
            regions: HashMap::new(),
            last_cache_check: Instant::now()
        };

        ThreadBuilder::new()
            .name("Anvil level source worker".into())
            .spawn(move || worker.run())
            .expect("Failed to create anvil level source worker thread.");

        result_receiver

    }

    fn run(mut self) {

        loop {

            match self.request_receiver.recv_timeout(REGIONS_REQUEST_RECV_TIMEOUT) {
                Ok(Request::Load(req)) => {
                    // debug!("Received chunk load request for {}/{}", req.cx, req.cz);
                    let chunk = self.load_chunk(req);
                    if let Err(_) = self.result_sender.send(chunk) {
                        break
                    }
                }
                Ok(Request::Save(req)) => {
                    debug!("Received chunk save request for {}/{}", req.cx, req.cz);
                    self.save_chunk(req);
                }
                Err(RecvTimeoutError::Timeout) => {},
                Err(RecvTimeoutError::Disconnected) => break
            }

            self.check_cache();

        }

    }

    fn access_region(&mut self, rx: i32, rz: i32, create: bool) -> RegionResult<&mut TimedCache<RegionFile>> {
        match self.regions.entry((rx, rz)) {
            Entry::Occupied(o) => Ok(o.into_mut().cache_update()),
            Entry::Vacant(v) => {
                let region = RegionFile::new(self.regions_dir.clone(), rx, rz, create)?;
                debug!("Region file opened at {}/{}", rx, rz);
                Ok(v.insert(TimedCache::new(region, REGIONS_CACHE_TIME)))
            }
        }
    }

    fn load_chunk(&mut self, req: ChunkLoadRequest) -> Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)> {

        let (rx, rz) = calc_region_pos(req.cx, req.cz);
        let region = match self.access_region(rx, rz, false) {
            Ok(region) => region,
            Err(RegionError::FileNotFound(_)) => return Err((LevelSourceError::UnsupportedChunkPosition, req)),
            Err(e) => return Err((LevelSourceError::new_custom(e), req))
        };

        let mut reader = match region.get_chunk_reader(req.cx, req.cz) {
            Ok(reader) => reader,
            // If the chunk is empty, just return an unsupported chunk pos error, this is used to
            // delegate to the generator in case of LoadOrGen source.
            Err(RegionError::EmptyChunk) => return Err((LevelSourceError::UnsupportedChunkPosition, req)),
            Err(err) => return Err((LevelSourceError::new_custom(err), req))
        };

        let mut chunk = req.build_proto_chunk();

        match decode_chunk_from_reader(&mut reader, &mut chunk) {
            Ok(_) => Ok(chunk),
            Err(err) => Err((LevelSourceError::new_custom(err), req))
        }

    }

    fn save_chunk(&mut self, req: ChunkSaveRequest) {

        let chunk = req.chunk.read().unwrap();
        let (cx, cz) = chunk.get_position();
        let (rx, rz) = calc_region_pos(cx, cz);

        let region = match self.access_region(rx, rz, true) {
            Ok(region) => region,
            Err(_) => return
        };

        let mut writer = region.get_chunk_writer(cx, cz, Default::default());
        encode_chunk_to_writer(&mut writer, &*chunk);
        writer.write_chunk();
        // debug!("Chunk at {}/{} saved", cx, cz);

    }

    fn check_cache(&mut self) {
        if self.last_cache_check.elapsed() >= REGIONS_CACHE_TIME {
            self.regions.retain(|(rx, rz), region| {
                if region.is_cache_timed_out() {
                    debug!("Region file timed out at {}/{}", rx, rz);
                    false
                } else {
                    true
                }
            });
            self.last_cache_check = Instant::now();
        }
    }

}
