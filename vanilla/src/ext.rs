use mc_core::world::level::LevelEnv;
use mc_core::block::GlobalBlocks;
use mc_core::biome::GlobalBiomes;
use crate::block::VANILLA_BLOCKS;
use crate::biome::VANILLA_BIOMES;


/// A trait to implement on registers or group of registers to provide
/// a default constructor for vanilla contents.
pub trait WithVanilla {
    fn with_vanilla() -> Self;
}


impl WithVanilla for GlobalBlocks {
    fn with_vanilla() -> Self {
        // SAFETY: It's safe to unwrap because the states count for vanilla
        //         (around 20k states) can't overflow 32 bits save ID.
        Self::with_static(&VANILLA_BLOCKS).unwrap()
    }
}


impl WithVanilla for GlobalBiomes {
    fn with_vanilla() -> Self {
        // SAFETY: Check safety comment for vanilla blocks.
        Self::with_static(&VANILLA_BIOMES).unwrap()
    }
}


impl WithVanilla for LevelEnv {
    fn with_vanilla() -> Self {
        Self::new(GlobalBlocks::with_vanilla(), GlobalBiomes::with_vanilla())
    }
}
