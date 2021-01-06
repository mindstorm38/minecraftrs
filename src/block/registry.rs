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

        reg.register(Block::new("air", 0));
        reg.register(Block::new("stone", 1).set_hardness(1.5).set_resistance(10.0));
        reg.register(Block::new("grass", 2).set_hardness(0.6));
        reg.register(Block::new("dirt", 3).set_hardness(0.5));
        reg.register(Block::new("cobblestone", 4).set_hardness(2.0).set_resistance(10.0));
        reg.register(Block::new("bedrock", 7).set_hardness(-1.0).set_resistance(6000000.0));
        reg.register(Block::new("water", 9).set_hardness(100.0));
        reg.register(Block::new("sand", 12).set_hardness(0.5));
        reg.register(Block::new("sand_stone", 24).set_hardness(0.8));
        reg.register(Block::new("ice", 79).set_hardness(0.5));
        reg.register(Block::new("mycelium", 110).set_hardness(0.6));

        BlockRegistry(reg)

    }

}
