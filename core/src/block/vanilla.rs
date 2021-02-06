use super::{IntProperty, BoolProperty};
use crate::blocks;


pub static PROP_LIQUID_LEVEL: IntProperty = IntProperty("level", 16);
pub static PROP_CACTUS_AGE: IntProperty = IntProperty("age", 16);
pub static PROP_CROP_AGE: IntProperty = IntProperty("age", 8);
pub static PROP_BAMBOO_AGE: IntProperty = IntProperty("age", 2);
pub static PROP_BAMBOO_STAGE: IntProperty = IntProperty("stage", 2);

pub static PROP_HAS_BOTTLE_0: BoolProperty = BoolProperty("has_bottle_0");
pub static PROP_HAS_BOTTLE_1: BoolProperty = BoolProperty("has_bottle_1");
pub static PROP_HAS_BOTTLE_2: BoolProperty = BoolProperty("has_bottle_2");


blocks!(VanillaBlocksStruct VanillaBlocks [
    STONE "stone",
    GRASS "grass",
    DIRT "dirt",
    COBBLESTONE "cobblestone",
    BEDROCK "bedrock",
    CACTUS "cactus" [PROP_CACTUS_AGE],
    BAMBOO "bamboo" [PROP_BAMBOO_AGE, /* PROP_BAMBOO_LEAVES */PROP_BAMBOO_STAGE],
    WHEAT "wheat" [PROP_CROP_AGE],
    BREWING_STAND "brewing_stand" [PROP_HAS_BOTTLE_0, PROP_HAS_BOTTLE_1, PROP_HAS_BOTTLE_2]
]);
