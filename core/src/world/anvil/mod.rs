use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

pub mod region;

use region::{RegionFile, RegionResult, RegionError};
use std::io::Read;


#[inline]
fn calc_region_pos(cx: i32, cz: i32) -> (i32, i32) {
    (cx >> 5, cz >> 5)
}

pub struct AnvilManager {
    dir: PathBuf,
    cache: Mutex<HashMap<(i32, i32), Mutex<RegionFile>>>
}

impl AnvilManager {

    pub fn get_chunk_reader(&mut self, cx: i32, cz: i32) -> RegionResult<Box<dyn Read>> {

        let mut cache = self.cache.lock().unwrap();
        let pos = calc_region_pos(cx, cz);

        let mut region = match cache.entry(pos) {
            Entry::Occupied(o) => {
                o.into_mut().lock().unwrap()
            },
            Entry::Vacant(v) => {
                let region = RegionFile::new(self.dir.clone(), pos.0, pos.1)?;
                v.insert(Mutex::new(region)).lock().unwrap()
            }
        };

        region.get_chunk_reader(cx, cz)

    }

}
