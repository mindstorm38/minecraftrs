use std::thread::Builder as ThreadBuilder;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::time::{Instant, Duration};

use crossbeam_channel::{Sender, Receiver, RecvTimeoutError, unbounded, bounded};

use crate::world::source::{LevelSource, LevelSourceError, ChunkInfo, ProtoChunk};
use crate::util::TimedCache;
use crate::debug;

use super::region::{RegionFile, RegionError, calc_region_pos};
use super::decode::{decode_chunk_from_reader};


/// A level source that load chunks from anvil region files. This source internally use
/// a threaded worker to avoid disk access durations overhead. Each opened region file
/// remains opened for `REGIONS_CACHE_TIME` duration.
pub struct AnvilLevelSource {
    request_sender: Sender<ChunkInfo>,
    result_receiver: Receiver<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>>
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

    fn request_chunk_load(&mut self, info: ChunkInfo) -> Result<(), (LevelSourceError, ChunkInfo)> {
        // SAFETY: Unwrap should be safe because the channel is unbounded.
        self.request_sender.send(info).unwrap();
        Ok(())
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>> {
        self.result_receiver.try_recv().ok()
    }

}


const REGIONS_CACHE_TIME: Duration = Duration::from_secs(60);
const REGIONS_REQUEST_RECV_TIMEOUT: Duration = Duration::from_secs(30);

struct Worker {
    regions_dir: PathBuf,
    request_receiver: Receiver<ChunkInfo>,
    result_sender: Sender<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>>,
    regions: HashMap<(i32, i32), TimedCache<RegionFile>>,
    last_cache_check: Instant
}

impl Worker {

    /// Internal constructor for worker, you must give the regions directory, not level directory.
    fn new(
        regions_dir: PathBuf,
        request_receiver: Receiver<ChunkInfo>
    ) -> Receiver<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>> {

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
                Ok(chunk_info) => {
                    debug!("Received chunk load request for {}/{}", chunk_info.cx, chunk_info.cz);
                    let chunk = self.load_chunk(chunk_info);
                    if let Err(_) = self.result_sender.send(chunk) {
                        break
                    }
                },
                Err(RecvTimeoutError::Timeout) => {},
                Err(RecvTimeoutError::Disconnected) => break
            }

            self.check_cache();

        }

    }

    fn load_chunk(&mut self, chunk_info: ChunkInfo) -> Result<ProtoChunk, (LevelSourceError, ChunkInfo)> {

        let region_pos = calc_region_pos(chunk_info.cx, chunk_info.cz);

        let region = match self.regions.entry(region_pos) {
            Entry::Occupied(o) => o.into_mut().cache_update(),
            Entry::Vacant(v) => {
                match RegionFile::new(self.regions_dir.clone(), region_pos.0, region_pos.1) {
                    Ok(region) => {
                        debug!("Region file opened at {}/{}", region_pos.0, region_pos.1);
                        v.insert(TimedCache::new(region, REGIONS_CACHE_TIME))
                    },
                    Err(RegionError::FileNotFound(_)) => return Err((LevelSourceError::UnsupportedChunkPosition, chunk_info)),
                    Err(err) => return Err((LevelSourceError::new_custom(err), chunk_info))
                }
            }
        };

        let mut reader = match region.get_chunk_reader(chunk_info.cx, chunk_info.cz) {
            Ok(reader) => reader,
            // If the chunk is empty, just return an unsupported chunk pos error, this is used to
            // delegate to the generator in case of LoadOrGen source.
            Err(RegionError::EmptyChunk) => return Err((LevelSourceError::UnsupportedChunkPosition, chunk_info)),
            Err(err) => return Err((LevelSourceError::new_custom(err), chunk_info))
        };

        let mut chunk = chunk_info.build_proto_chunk();

        match decode_chunk_from_reader(&mut reader, &mut chunk) {
            Ok(_) => Ok(chunk),
            Err(err) => Err((LevelSourceError::new_custom(err), chunk_info))
        }

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
