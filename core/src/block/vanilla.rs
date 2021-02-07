use crate::util::{Direction, Axis};
use crate::{blocks, properties};


properties! {

    pub PROP_AGE_26: int("age", 26);
    pub PROP_AGE_16: int("age", 16);
    pub PROP_AGE_8: int("age", 8);
    pub PROP_AGE_6: int("age", 6);
    pub PROP_AGE_4: int("age", 4);
    pub PROP_AGE_3: int("age", 3);
    pub PROP_BAMBOO_AGE: int("age", 2);
    pub PROP_BAMBOO_STAGE: int("stage", 2);
    pub PROP_HAS_BOTTLE_0: bool("has_bottle_0");
    pub PROP_HAS_BOTTLE_1: bool("has_bottle_1");
    pub PROP_HAS_BOTTLE_2: bool("has_bottle_2");
    pub PROP_HONEY_LEVEL: int("honey_level", 6);
    pub PROP_ROTATION: int("rotation", 16);
    pub PROP_OPEN: bool("open");
    pub PROP_OCCUPIED: bool("occupied");
    pub PROP_POWERED: bool("powered");
    pub PROP_TRIGGERED: bool("triggered");
    pub PROP_ENABLED: bool("enabled");
    pub PROP_LIT: bool("lit");
    pub PROP_INVERTED: bool("inverted");
    pub PROP_SIGNAL_FIRE: bool("signal_fire");
    pub PROP_WATERLOGGED: bool("waterlogged");
    pub PROP_CAKE_BITES: int("bites", 7);
    pub PROP_CAULDRON_LEVEL: int("level", 4);
    pub PROP_COMPOSTER_LEVEL: int("level", 9);
    pub PROP_REDSTONE_POWER: int("power", 16);
    pub PROP_END_PORTAL_EYE: bool("eye");
    pub PROP_FARMLAND_MOISTURE: int("moisture", 8);
    pub PROP_SNOWY: bool("snowy");
    pub PROP_JUKEBOX_HAS_RECORD: bool("has_record");
    pub PROP_HANGING: bool("hanging");

    pub PROP_DOWN: bool("down");
    pub PROP_EAST: bool("east");
    pub PROP_NORTH: bool("north");
    pub PROP_SOUTH: bool("south");
    pub PROP_UP: bool("up");
    pub PROP_WEST: bool("west");

    pub PROP_FACING: enum<Direction>("facing", FACING, [
        Direction::East, Direction::North, Direction::South, Direction::West
    ]);

    pub PROP_FACING_ALL: enum<Direction>("facing", FACING_ALL, [
        Direction::Down, Direction::East, Direction::North, Direction::South, Direction::Up, Direction::West
    ]);

    pub PROP_AXIS: enum<Axis>("axis", AXIS, [Axis::X, Axis::Y, Axis::Z]);

    pub PROP_BAMBOO_LEAVES: enum<BambooLeaves>("leaves", BAMBOO_LEAVES, [BambooLeaves::Large, BambooLeaves::None, BambooLeaves::Small]);
    pub PROP_BED_PART: enum<BedPart>("part", BED_PART, [BedPart::Foot, BedPart::Head]);

    pub PROP_BELL_ATTACHMENT: enum<Face>("attachment", BELL_ATTACHMENT, [
        Face::Ceiling, Face::DoubleWall, Face::Floor, Face::SingleWall
    ]);

    pub PROP_FACE: enum<Face>("face", FACE, [
        Face::Ceiling, Face::Floor, Face::Wall
    ]);

}


// TODO:
//  - Banners
//  - Bubble Column
//  - Buttons
//  - Chests
//  - Command Blocks
//  - Doors
//  - Fences
//  - Fence Gates
//  - Glass Panes
//  - Glazed Terracotta
//  - Jigsaw Block
//  - Large Flowers
//  - Fluids


blocks!(VanillaBlocksStruct VanillaBlocks [
    STONE "stone",
    GRASS_BLOCK "grass_block"       [PROP_SNOWY],
    PODZOL "podzol"                 [PROP_SNOWY],
    MYCELIUM "mycelium"             [PROP_SNOWY],
    DIRT "dirt",
    COBBLESTONE "cobblestone",
    BEDROCK "bedrock",
    CACTUS "cactus"                 [PROP_AGE_16],
    BAMBOO "bamboo"                 [PROP_BAMBOO_AGE, PROP_BAMBOO_LEAVES, PROP_BAMBOO_STAGE],
    WHEAT "wheat"                   [PROP_AGE_8],
    CARROTS "carrots"               [PROP_AGE_8],
    BEETROOTS "beetroots"           [PROP_AGE_4],
    ANVIL "anvil"                   [PROP_FACING],
    BARREL "barrel"                 [PROP_FACING_ALL, PROP_OPEN],
    BED "bed"                       [PROP_FACING, PROP_OCCUPIED, PROP_BED_PART],
    BEEHIVE "beehive"               [PROP_FACING, PROP_HONEY_LEVEL],
    BEE_NEST "bee_nest"             [PROP_FACING, PROP_HONEY_LEVEL],
    BELL "bell"                     [PROP_BELL_ATTACHMENT, PROP_FACING, PROP_POWERED],
    BLAST_FURNACE "blast_furnace"   [PROP_FACING, PROP_LIT],
    BONE_BLOCK "bone_block"         [PROP_AXIS],
    BREWING_STAND "brewing_stand"   [PROP_HAS_BOTTLE_0, PROP_HAS_BOTTLE_1, PROP_HAS_BOTTLE_2],
    CAMPFIRE "campfire"             [PROP_FACING, PROP_LIT, PROP_SIGNAL_FIRE, PROP_WATERLOGGED],
    CAKE "cake"                     [PROP_CAKE_BITES],
    CARVED_PUMPKIN "carved_pumpkin" [PROP_FACING],
    CAULDRON "cauldron"             [PROP_CAULDRON_LEVEL],
    CHAIN "chain"                   [PROP_WATERLOGGED, PROP_AXIS],
    ENDER_CHEST "ender_chest"       [PROP_FACING, PROP_WATERLOGGED],
    CHORUS_FLOWER "chorus_flower"   [PROP_AGE_6],
    CHORUS_PLANT "chorus_plant"     [PROP_DOWN, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_UP, PROP_WEST],
    COCOA "cocoa"                   [PROP_AGE_3, PROP_FACING],
    COMPOSTER "composter"           [PROP_COMPOSTER_LEVEL],
    CONDUIT "conduit"               [PROP_WATERLOGGED],
    DAYLIGHT_DETECTOR "daylight_detector" [PROP_INVERTED, PROP_REDSTONE_POWER],
    DISPENSER "dispenser"           [PROP_FACING_ALL, PROP_TRIGGERED],
    DROPPER "dropper"               [PROP_FACING_ALL, PROP_TRIGGERED],
    END_PORTAL_FRAME "end_portal_frame" [PROP_END_PORTAL_EYE, PROP_FACING],
    END_ROD "end_rod"               [PROP_FACING_ALL],
    FARMLAND "farmland"             [PROP_FARMLAND_MOISTURE],
    FIRE "fire"                     [PROP_AGE_16, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_UP, PROP_WEST],
    FROSTED_ICE "frosted_ice"       [PROP_AGE_4],
    FURNACE "furnace"               [PROP_FACING, PROP_LIT],
    GRINDSTONE "grindstone"         [PROP_FACE, PROP_FACING],
    HAY_BLOCK "hay_block"           [PROP_AXIS],
    HOPPER "hopper"                 [PROP_ENABLED, PROP_FACING],
    IRON_BARS "iron_bars"           [PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_WEST, PROP_WATERLOGGED],
    JACK_O_LANTERN "jack_o_lantern" [PROP_FACING],
    JUKEBOX "jukebox"               [PROP_JUKEBOX_HAS_RECORD],
    KELP "kelp"                     [PROP_AGE_26],
    LADDER "ladder"                 [PROP_FACING, PROP_WATERLOGGED],
    LANTERN "lantern"               [PROP_HANGING]
]); // TODO: https://minecraft.gamepedia.com/Block_states#Lectern


#[derive(strum::ToString, Debug, Copy, Clone, Eq, PartialEq)]
pub enum BambooLeaves {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "large")]
    Large,
    #[strum(serialize = "small")]
    Small
}


#[derive(strum::ToString, Debug, Copy, Clone, Eq, PartialEq)]
pub enum BedPart {
    #[strum(serialize = "foot")]
    Foot,
    #[strum(serialize = "head")]
    Head
}


#[derive(strum::ToString, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Face {
    #[strum(serialize = "ceiling")]
    Ceiling,
    #[strum(serialize = "floor")]
    Floor,
    #[strum(serialize = "wall")]
    Wall,
    #[strum(serialize = "double_wall")]
    DoubleWall,
    #[strum(serialize = "single_wall")]
    SingleWall
}
