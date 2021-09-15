//!
//! Naming convention for generators modules (check the `version` module before):
//!
//! `"gen" + <major_version> + <minor_version>` with `minor_version` being padded to 2 digits.
//!

pub mod noise;
// pub mod carver;
// pub mod feature;
// pub mod layer;
pub mod gen;

pub mod layer_new;

/*use crate::world::WorldInfo;
use crate::world::loader::ChunkLoader;
use crate::version::{Version, VersionType::*};
use std::rc::Rc;

pub mod layer;
pub mod feature;
pub mod carver;

#[cfg(feature = "release-1-2")]
pub mod gen102;


pub fn for_world(world_info: Rc<WorldInfo>) -> Box<dyn ChunkLoader> {

    match world_info.version {
        #[cfg(feature = "release-1-2")]
        Version(Release, 1, 2, _) => Box::new(gen102::ChunkGenerator102::new(world_info)),
        _ => panic!("Version {} has no generator supported !", world_info.version)
    }

}
*/
