use super::vanilla::*;
use super::BlockState;


/// A block extension to set for legacy blocks.
pub enum LegacyId {
    Simple(u8, u8),
    Dynamic(fn(&BlockState) -> (u8, u8))
}


macro_rules! legacy {
    ($block_id:ident = $legacy_id:expr; $($t:tt)*) => {
        VanillaBlocks.$block_id.add_ext(LegacyId::Simple($legacy_id, 0));
        legacy!($($t)*);
    };
    ($block_id:ident = $legacy_id:expr, $legacy_meta:expr; $($t:tt)*) => {
        VanillaBlocks.$block_id.add_ext(LegacyId::Simple($legacy_id, $legacy_meta));
        legacy!($($t)*);
    };
    ($block_id:ident = func $legacy_supplier:ident; $($t:tt)*) => {
        VanillaBlocks.$block_id.add_ext(LegacyId::Dynamic($legacy_supplier));
        legacy!($($t)*);
    };
    () => {}
}


pub fn setup_legacy_ids() {
    legacy! {

        STONE               = 1, 0;
        GRANITE             = 1, 1;
        POLISHED_GRANITE    = 1, 2;
        DIORITE             = 1, 3;
        POLISHED_DIORITE    = 1, 4;
        ANDESITE            = 1, 5;
        POLISHED_ANDESITE   = 1, 6;

        GRASS_BLOCK = 2;

        DIRT        = 3, 0;
        COARSE_DIRT = 3, 1;
        PODZOL      = 3, 2;

        COBBLESTONE = 4;
        PLANKS      = 5; // TODO
        SAPLING     = 6; // TODO
        BEDROCK     = 7;

        SAND     = 12, 0;
        RED_SAND = 12, 1;
        GRAVEL   = 13;
        COAL_ORE = 16;
        LOG      = 17; // TODO
        LEAVES   = 18;

        GOLD_ORE    = 14;
        GOLD_BLOCK  = 41;
        IRON_ORE    = 15;
        IRON_BLOCK  = 42;
        LAPIS_ORE   = 21;
        LAPIS_BLOCK = 22;
        DIAMOND_ORE = 56;
        DIAMOND_BLOCK = 57;

        SANDSTONE = 24, 0;
        CHISELED_SANDSTONE = 24, 1;
        CUT_SANDSTONE = 24, 2;
        GLASS = 20;
        BRICKS = 45;
        MOSSY_COBBLESTONE = 48;
        OBSIDIAN = 49;
        TORCH = 50;
        FIRE = 51;
        SPAWNER = 52;
        WOODEN_STAIRS = 53;
        CHEST = 54;

        SPONGE = 19, 0;
        WET_SPONGE = 19, 1;
        DISPENSER  = 23;
        NOTE_BLOCK = 25;
        BED = 26;
        COBWEB = 30;
        TNT = 46;
        BOOKSHELF = 47;
        CRAFTING_TABLE = 58;
        WHEAT = 59;
        FARMLAND = 60; // TODO: Last ID done today

        POWERED_RAIL = 27;
        DETECTOR_RAIL = 28;

        REDSTONE_WIRE = 55;
        PISTON = func piston;
        PISTON_HEAD = 34;

        // The variant (31, 0) is a "fake dead-bush", not represented in modern versions.
        GRASS     = 31, 1;
        FERN      = 31, 2;
        DEAD_BUSH = 32;
        WOOL = func wool;

        DANDELION    = 37;
        POPPY        = 38, 0;
        BLUE_ORCHID  = 38, 1;
        ALLIUM       = 38, 2;
        AZURE_BLUET  = 38, 3;
        RED_TULIP    = 38, 4;
        ORANGE_TULIP = 38, 5;
        WHITE_TULIP  = 38, 6;
        PINK_TULIP   = 38, 7;
        OXEYE_DAISY  = 38, 8;

        BROWN_MUSHROOM = 39;
        RED_MUSHROOM   = 40;

        // TODO:
        //  - Slabs
        //  - Water / Lava

    }
}


pub fn get_legacy_id(b: &BlockState) -> Option<(u8, u8)> {
    let legacy_id = b.get_block().get_ext::<LegacyId>()?;
    match *legacy_id {
        LegacyId::Simple(id, meta) => Some((id, meta)),
        LegacyId::Dynamic(func) => Some(func(b))
    }
}


fn piston(b: &BlockState) -> (u8, u8) {
    match b.get(&PROP_STICKY).unwrap() {
        true => (29, 0),
        false => (33, 0)
    }
}

fn wool(b: &BlockState) -> (u8, u8) {
    (35, b.get(&PROP_COLOR).unwrap().get_index())
}
