//!
//! Naming convention for generators modules (check the `version` module before) :
//!
//! "gen" + <major_version> + <("0" if minor_version < 10)minor_version>
//!

use crate::world::WorldInfo;
use crate::world::provider::ChunkLoader;
use crate::version::{Version, VersionType::*};
use std::rc::Rc;

pub mod layer;
pub mod gen102;


pub fn for_world(world_info: Rc<WorldInfo>) -> Box<dyn ChunkLoader> {

    match world_info.version {
        Version(Release, 1, 2, _) => Box::new(gen102::ChunkGenerator102::new(world_info)),
        _ => panic!("Version {} has no generator supported !", world_info.version)
    }

}
