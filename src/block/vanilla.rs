use crate::def_blocks;
use super::property::*;

pub static PROP_LIQUID_LEVEL: IntProperty = IntProperty("level", 16);
pub static PROP_CACTUS_AGE: IntProperty = IntProperty("age", 16);

def_blocks!(VanillaBlocks [
    STONE "stone",
    GRASS "grass",
    DIRT "dirt",
    COBBLESTONE "cobblestone",
    BEDROCK "bedrock",
    CACTUS "cactus" [PROP_CACTUS_AGE]
]);