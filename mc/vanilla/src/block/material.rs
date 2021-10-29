//! Base module for block's materials. This module defines multiple tags that are included in the
//! vanilla module.

use mc_core::block::{Block, GlobalBlocks};
use mc_core::tag::TagType;
use crate::block::*;


pub static TAG_NON_SOLID: TagType = TagType::new_logical("minecraft:rust:non_solid");
pub static TAG_NON_BLOCKING: TagType = TagType::new_logical("minecraft:rust:non_blocking");
pub static TAG_LIQUID: TagType = TagType::new_logical("minecraft:rust:liquid");
pub static TAG_LEAVES: TagType = TagType::new_logical("minecraft:rust:leaves");
pub static TAG_LOG: TagType = TagType::new_logical("minecraft:rust:log");


pub(crate) fn register_tags(blocks: &mut GlobalBlocks) {

    blocks.register_tag_type(&TAG_NON_SOLID);
    blocks.set_blocks_tag(&TAG_NON_SOLID, true, NON_SOLID_BLOCKS.iter().copied()).unwrap();

    blocks.register_tag_type(&TAG_NON_BLOCKING);
    blocks.set_blocks_tag(&TAG_NON_BLOCKING, true, NON_SOLID_BLOCKS.iter().chain(NON_BLOCKING_BLOCKS.iter()).copied()).unwrap();

    blocks.register_tag_type(&TAG_LIQUID);
    blocks.set_blocks_tag(&TAG_LIQUID, true, [
        &WATER,
        &LAVA
    ]).unwrap();

    blocks.register_tag_type(&TAG_LEAVES);
    blocks.set_blocks_tag(&TAG_LEAVES, true, [
        &OAK_LEAVES,
        &SPRUCE_LEAVES,
        &BIRCH_LEAVES,
        &JUNGLE_LEAVES,
        &ACACIA_LEAVES,
        &DARK_OAK_LEAVES,
        &AZALEA_LEAVES,
        &FLOWERING_AZALEA_LEAVES
    ]).unwrap();

    blocks.register_tag_type(&TAG_LOG);
    blocks.set_blocks_tag(&TAG_LOG, true, [
        &OAK_LOG,
        &SPRUCE_LOG,
        &BIRCH_LOG,
        &JUNGLE_LOG,
        &ACACIA_LOG,
        &DARK_OAK_LOG,
    ]).unwrap();

}


static NON_SOLID_BLOCKS: &[&'static Block] = &[

    // Air
    &AIR,
    &STRUCTURE_VOID,

    // Portal
    &NETHER_PORTAL,
    &END_PORTAL,
    &END_GATEWAY,

    // Cloth
    &CARPET,

    // Plants
    &OAK_SAPLING,
    &SPRUCE_SAPLING,
    &BIRCH_SAPLING,
    &JUNGLE_SAPLING,
    &ACACIA_SAPLING,
    &DARK_OAK_SAPLING,

    &DANDELION,
    &POPPY,
    &BLUE_ORCHID,
    &ALLIUM,
    &AZURE_BLUET,
    &RED_TULIP,
    &ORANGE_TULIP,
    &WHITE_TULIP,
    &PINK_TULIP,
    &OXEYE_DAISY,
    &CORNFLOWER,
    &WITHER_ROSE,
    &LILY_OF_THE_VALLEY,

    &BROWN_MUSHROOM,
    &RED_MUSHROOM,

    &WHEAT,
    &SUGAR_CANE,
    &ATTACHED_PUMPKIN_STEM,
    &ATTACHED_MELON_STEM,
    &PUMPKIN_STEM,
    &MELON_STEM,
    &LILY_PAD,
    &NETHER_WART,
    &COCOA,
    &CARROTS,
    &POTATOES,
    &CHORUS_PLANT,
    &CHORUS_FLOWER,
    &BEETROOTS,
    &SWEET_BERRY_BUSH,

    &WARPED_FUNGUS,
    &CRIMSON_FUNGUS,
    &WEEPING_VINES,
    &WEEPING_VINES_PLANT,
    &TWISTING_VINES,
    &TWISTING_VINES_PLANT,

    &CAVE_VINES,
    &CAVE_VINES_PLANT,
    &SPORE_BLOSSOM,
    &AZALEA,
    &FLOWERING_AZALEA,
    &MOSS_CARPET,
    &BIG_DRIPLEAF,
    &BIG_DRIPLEAF_STEM,
    &SMALL_DRIPLEAF,

    // Water plants
    &KELP,
    &KELP_PLANT,
    &CORAL,
    &CORAL_FAN,
    &CORAL_WALL_FAN,
    &SEA_PICKLE,

    // Replaceable plants
    &GRASS,
    &FERN,
    &DEAD_BUSH,
    &VINE,
    &GLOW_LICHEN,
    &SUNFLOWER,
    &LILAC,
    &ROSE_BUSH,
    &PEONY,
    &TALL_GRASS,
    &LARGE_FERN,
    &HANGING_ROOTS,

    &WARPED_ROOTS,
    &NETHER_SPROUTS,
    &CRIMSON_ROOTS,

    // Replaceable water plants
    &SEAGRASS,
    &TALL_SEAGRASS,

    // Liquids
    &WATER,
    &BUBBLE_COLUMN,
    &LAVA,

    // Snow
    &SNOW,

    // Fire
    &FIRE,
    &SOUL_FIRE,

    // Decoration
    &POWERED_RAIL,
    &DETECTOR_RAIL,
    &TORCH,
    &WALL_TORCH,
    &REDSTONE_WIRE,
    &LADDER,
    &RAIL,
    &LEVER,
    &REDSTONE_TORCH,
    &REDSTONE_WALL_TORCH,
    &STONE_BUTTON,
    &SOUL_TORCH,
    &SOUL_WALL_TORCH,
    &REPEATER,
    &TRIPWIRE_HOOK,
    &TRIPWIRE,

    &FLOWER_POT,
    &POTTED_OAK_SAPLING,
    &POTTED_SPRUCE_SAPLING,
    &POTTED_BIRCH_SAPLING,
    &POTTED_JUNGLE_SAPLING,
    &POTTED_ACACIA_SAPLING,
    &POTTED_DARK_OAK_SAPLING,
    &POTTED_FERN,
    &POTTED_DANDELION,
    &POTTED_POPPY,
    &POTTED_BLUE_ORCHID,
    &POTTED_ALLIUM,
    &POTTED_AZURE_BLUET,
    &POTTED_RED_TULIP,
    &POTTED_ORANGE_TULIP,
    &POTTED_WHITE_TULIP,
    &POTTED_PINK_TULIP,
    &POTTED_OXEYE_DAISY,
    &POTTED_CORNFLOWER,
    &POTTED_LILY_OF_THE_VALLEY,
    &POTTED_WITHER_ROSE,
    &POTTED_RED_MUSHROOM,
    &POTTED_BROWN_MUSHROOM,
    &POTTED_DEAD_BUSH,
    &POTTED_CACTUS,
    &POTTED_BAMBOO,
    &POTTED_CRIMSON_FUNGUS,
    &POTTED_WARPED_FUNGUS,
    &POTTED_CRIMSON_ROOTS,
    &POTTED_WARPED_ROOTS,
    &POTTED_AZALEA,
    &POTTED_FLOWERING_AZALEA,

    &OAK_BUTTON,
    &SPRUCE_BUTTON,
    &BIRCH_BUTTON,
    &JUNGLE_BUTTON,
    &ACACIA_BUTTON,
    &DARK_OAK_BUTTON,

    &SKELETON_SKULL,
    &SKELETON_WALL_SKULL,
    &WITHER_SKELETON_SKULL,
    &WITHER_SKELETON_WALL_SKULL,
    &ZOMBIE_HEAD,
    &ZOMBIE_WALL_HEAD,
    &PLAYER_HEAD,
    &PLAYER_WALL_HEAD,
    &CREEPER_HEAD,
    &CREEPER_WALL_HEAD,
    &DRAGON_HEAD,
    &DRAGON_WALL_HEAD,

    &COMPARATOR,
    &ACTIVATOR_RAIL,
    &END_ROD,
    &SCAFFOLDING,

    &CRIMSON_BUTTON,
    &WARPED_BUTTON,
    &POLISHED_BLACKSTONE_BUTTON,
    &CANDLE,
    &COLORED_CANDLE,

    // Powder snow
    &POWDER_SNOW,

];

// Non blocking list is an extension to
static NON_BLOCKING_BLOCKS: &[&'static Block] = &[
    &COBWEB,
    &BAMBOO_SAPLING,
];
