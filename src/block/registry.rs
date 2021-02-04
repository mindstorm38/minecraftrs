use crate::block::Block;
use crate::version::Version;
use crate::res::Registry;
use derive_more::{Deref, DerefMut};


/// A block registry, used to store all available blocks for a specific
/// versions and get them by their identifier or legacy ids.
#[derive(Deref, DerefMut)]
pub struct BlockRegistry(pub Registry<u16, Block>);


impl From<Version> for BlockRegistry {

    fn from(_: Version) -> Self {

        let mut reg = Registry::new();

        reg.register(Block::new("stone", 1));
        reg.register(Block::new("grass", 2));
        reg.register(Block::new("dirt", 3));
        reg.register(Block::new("cobblestone", 4));
        reg.register(Block::new("bedrock", 7));
        reg.register(Block::new("water", 9));
        reg.register(Block::new("lava", 11));
        reg.register(Block::new("sand", 12));
        reg.register(Block::new("sand_stone", 24));
        reg.register(Block::new("ice", 79));
        reg.register(Block::new("mycelium", 110));

        BlockRegistry(reg)

    }

}


macro_rules! block {
    ($name:ident, $id:expr) => {
        #[allow(unused)]
        const $name: u16 = $id;
    };
}

block!(STONE, 1);
block!(GRASS, 2);
block!(DIRT, 3);
block!(COBBLESTONE, 4);
block!(PLANKS, 5);
block!(SAPLING, 6);
block!(BEDROCK, 7);
block!(WATER_MOVING, 8);
block!(WATER_STILL, 9);
block!(LAVA_MOVING, 10);
block!(LAVA_STILL, 11);
block!(SAND, 12);
block!(GRAVEL, 13);
block!(GOLD_ORE, 14);
block!(IRON_ORE, 15);
block!(COAL_ORE, 16);
block!(LOG, 17);
block!(LEAVES, 18);
block!(SPONGE, 19);
block!(GLASS, 20);
block!(LAPIS_ORE, 21);
block!(LAPIS_BLOCK, 21);
