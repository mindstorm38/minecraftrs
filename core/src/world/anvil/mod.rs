use std::sync::{Mutex, Arc};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod region;
pub mod loader;

use region::{RegionFile, RegionResult};


#[inline]
fn calc_region_pos(cx: i32, cz: i32) -> (i32, i32) {
    (cx >> 5, cz >> 5)
}


pub struct AnvilManager {
    dir: PathBuf,
    cache: Mutex<HashMap<(i32, i32), Arc<Mutex<RegionFile>>>>
}


impl AnvilManager {

    pub fn ensure_region(&self, cx: i32, cz: i32) -> RegionResult<Arc<Mutex<RegionFile>>> {
        let pos = calc_region_pos(cx, cz);
        // Unwrapping the LockResult because the calling thread should not panic in this method.
        match self.cache.lock().unwrap().entry(pos) {
            Entry::Occupied(o) => Ok(Arc::clone(o.into_mut())),
            Entry::Vacant(v) => {
                let region = RegionFile::new(self.dir.clone(), pos.0, pos.1)?;
                let region = Arc::new(Mutex::new(region));
                Ok(Arc::clone(v.insert(region)))
            }
        }
    }

}
