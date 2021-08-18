use mc_core::world::level::LevelEnv;
use mc_core::block::GlobalBlocks;
use mc_core::biome::GlobalBiomes;
use crate::block::VANILLA_BLOCKS;
use crate::biome::VANILLA_BIOMES;


/// A trait to implement on registers or group of registers to provide
/// a default constructor for vanilla contents.
pub trait FromVanilla {
    fn from_vanilla() -> Self;
}


impl FromVanilla for GlobalBlocks {
    fn from_vanilla() -> Self {
        // SAFETY: It's safe to unwrap because the states count for vanilla
        //         (around 20k states) can't overflow 32 bits save ID.
        Self::from_static(&VANILLA_BLOCKS).unwrap()
    }
}


impl FromVanilla for GlobalBiomes {
    fn from_vanilla() -> Self {
        // SAFETY: Check safety comment for vanilla blocks.
        Self::from_static(&VANILLA_BIOMES).unwrap()
    }
}


impl FromVanilla for LevelEnv {
    fn from_vanilla() -> Self {
        Self::new(GlobalBlocks::from_vanilla(), GlobalBiomes::from_vanilla())
    }
}
