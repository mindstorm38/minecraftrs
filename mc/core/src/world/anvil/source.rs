use std::thread::Builder as ThreadBuilder;
use std::collections::hash_map::Entry;
use std::io::{Result as IoResult};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Instant, Duration};
use std::fs::File;

use crossbeam_channel::{Sender, Receiver, RecvTimeoutError, unbounded, bounded};
use nbt::{CompoundTag, decode::read_gzip_compound_tag};

use crate::world::chunk::{Chunk};
use crate::util::TimedCache;
use crate::world::source::{
    LevelSource, LevelSourceError, LevelSourceResult, LevelSourcePollResult,
    LevelSourceBuilder, ChunkBuilder
};
use crate::debug;

use super::region::{RegionFile, calc_region_pos};
use super::decode::{decode_chunk_from_reader};


pub struct AnvilLevelSourceBuilder {
    dir: PathBuf,
    metadata: CompoundTag
}

impl AnvilLevelSourceBuilder {

    /// Build the builder for `AnvilLevelSource` from the level directory.
    pub fn new<P: Into<PathBuf>>(dir: P) -> IoResult<Self> {

        let dir = dir.into();
        let mut file = File::open(dir.join("level.dat"))?;
        let metadata = read_gzip_compound_tag(&mut file)
            .expect("Invalid level.dat");  // TODO: Replace except with a new result type

        Ok(Self {
            dir,
            metadata
        })

    }

}

impl LevelSourceBuilder<AnvilLevelSource> for AnvilLevelSourceBuilder {

    fn build(self, chunk_builder: ChunkBuilder) -> AnvilLevelSource {
        AnvilLevelSource::new(self.dir, chunk_builder)
    }

}


/// A level source that load chunks from anvil region files. This source internally use
/// a threaded worker to avoid disk access durations overhead. Each opened region file
/// remains opened for `REGIONS_CACHE_TIME` duration.
pub struct AnvilLevelSource {
    request_sender: Sender<(i32, i32)>,
    result_receiver: Receiver<LevelSourcePollResult>
}

impl AnvilLevelSource {

    fn new(dir: PathBuf, chunk_builder: ChunkBuilder) -> Self {

        let (
            request_sender,
            request_receiver
        ) = unbounded();

        let result_receiver = Worker::new(
            dir.join("region"),
            chunk_builder,
            request_receiver
        );

        Self {
            request_sender,
            result_receiver
        }

    }

}

impl LevelSource for AnvilLevelSource {

    fn request_chunk_load(&mut self, cx: i32, cz: i32) -> LevelSourceResult<()> {
        // SAFETY: Unwrap should be safe because the channel is unbounded.
        self.request_sender.send((cx, cz)).unwrap();
        Ok(())
    }

    fn poll_chunk(&mut self) -> Option<LevelSourcePollResult> {
        self.result_receiver.try_recv().ok()
    }

}


pub const REGIONS_CACHE_TIME: Duration = Duration::from_secs(60);

struct Worker {
    regions_dir: PathBuf,
    chunk_builder: ChunkBuilder,
    request_receiver: Receiver<(i32, i32)>,
    result_sender: Sender<LevelSourcePollResult>,
    regions: HashMap<(i32, i32), TimedCache<RegionFile>>,
    last_cache_check: Instant
}

impl Worker {

    /// Internal constructor for worker, you must give the regions directory, not level directory.
    fn new(
        regions_dir: PathBuf,
        chunk_builder: ChunkBuilder,
        request_receiver: Receiver<(i32, i32)>
    ) -> Receiver<LevelSourcePollResult> {

        let (
            result_sender,
            result_receiver
        ) = bounded(128);

        let worker = Self {
            regions_dir,
            chunk_builder,
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

        static REQUEST_RECV_TIMEOUT: Duration = REGIONS_CACHE_TIME / 2;

        loop {

            match self.request_receiver.recv_timeout(REQUEST_RECV_TIMEOUT) {
                Ok(pos) => {
                    debug!("Received chunk load request for {}/{}", pos.0, pos.1);
                    let chunk = self.load_chunk(pos.0, pos.1);
                    if let Err(_) = self.result_sender.send((pos, chunk)) {
                        break
                    }
                },
                Err(RecvTimeoutError::Timeout) => {},
                Err(RecvTimeoutError::Disconnected) => break
            }

            self.check_cache();

        }

    }

    fn load_chunk(&mut self, cx: i32, cz: i32) -> LevelSourceResult<Chunk> {

        let region_pos = calc_region_pos(cx, cz);

        let region = match self.regions.entry(region_pos) {
            Entry::Occupied(o) => o.into_mut().cache_update(),
            Entry::Vacant(v) => {
                match RegionFile::new(self.regions_dir.clone(), region_pos.0, region_pos.1) {
                    Ok(region) => {
                        debug!("Region file opened at {}/{}", region_pos.0, region_pos.1);
                        v.insert(TimedCache::new(region, REGIONS_CACHE_TIME))
                    },
                    Err(err) => return Err(LevelSourceError::new_custom(err))
                }
            }
        };

        let mut reader = region.get_chunk_reader(cx, cz)
            .map_err(|err| LevelSourceError::new_custom(err))?;

        let mut chunk = self.chunk_builder.build(cx, cz);

        decode_chunk_from_reader(&mut reader, &mut chunk)
            .map_err(|err| LevelSourceError::new_custom(err))?;

        Ok(chunk)

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
