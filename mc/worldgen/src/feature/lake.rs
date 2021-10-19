use mc_core::block::BlockState;
use mc_core::world::chunk::ChunkGuard;
use mc_core::rand::JavaRandom;

use super::Feature;


pub struct LakeFeature {
    block: &'static BlockState
}

impl Feature for LakeFeature {

    fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {

        let x = x - 8;
        let mut y = y;
        let z = z - 8;

        /*while y > 5 && chunk.get_block_at() {

        }*/

    }

}
