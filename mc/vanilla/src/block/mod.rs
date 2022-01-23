use mc_core::{blocks, blocks_properties, blocks_specs, def_enum_serializable, impl_enum_serializable};
use mc_core::pos::{Axis, Direction};

use crate::util::DyeColor;

pub mod material;


impl_enum_serializable!(DyeColor {
    White: "white",
    Orange: "orange",
    Magenta: "magenta",
    LightBlue: "light_blue",
    Yellow: "yellow",
    Lime: "lime",
    Pink: "pink",
    Gray: "gray",
    LightGray: "light_gray",
    Cyan: "cyan",
    Purple: "purple",
    Blue: "blue",
    Brown: "brown",
    Green: "green",
    Red: "red",
    Black: "black"
});


static REDSTONE_MODE: [RedstoneWireMode; 3] = [RedstoneWireMode::None, RedstoneWireMode::Side, RedstoneWireMode::Up];
static WALL_SIDE: [WallSide; 3] = [WallSide::None, WallSide::Low, WallSide::Tall];


blocks_properties! {

    pub PROP_AGE_26: int("age", 26);
    pub PROP_AGE_16: int("age", 16);
    pub PROP_AGE_8: int("age", 8);
    pub PROP_AGE_6: int("age", 6);
    pub PROP_AGE_4: int("age", 4);
    pub PROP_AGE_3: int("age", 3);
    pub PROP_SAPLING_STAGE: int("stage", 2);
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
    pub PROP_CAULDRON_LEVEL: range("level", 1, 3);
    pub PROP_COMPOSTER_LEVEL: int("level", 9);
    pub PROP_REDSTONE_POWER: int("power", 16);
    pub PROP_END_PORTAL_EYE: bool("eye");
    pub PROP_FARMLAND_MOISTURE: int("moisture", 8);
    pub PROP_SNOWY: bool("snowy");
    pub PROP_HAS_RECORD: bool("has_record");
    pub PROP_HANGING: bool("hanging");
    pub PROP_HAS_BOOK: bool("has_book");
    pub PROP_NOTE: int("note", 25);
    pub PROP_STICKY: bool("sticky");
    pub PROP_EXTENDED: bool("extended");
    pub PROP_SHORT: bool("short");
    pub PROP_LOCKED: bool("locked");
    pub PROP_REPEATER_DELAY: range("delay", 1, 4); // Real is 1 to 4
    pub PROP_CHARGES: int("charges", 5);
    pub PROP_SCAFFOLDING_DISTANCE: int("distance", 8);
    pub PROP_PICKLES: range("pickles", 1, 4);  // Real is 1 to 4
    pub PROP_SNOW_LAYERS: range("layers", 1, 8);  // Real is 1 to 8
    pub PROP_UNSTABLE: bool("unstable");
    pub PROP_ATTACHED: bool("attached");
    pub PROP_DISARMED: bool("disarmed");
    pub PROP_EGGS: range("eggs", 1, 4);  // Real is 1 to 4
    pub PROP_HATCH: int("hatch", 3);
    pub PROP_LIQUID_LEVEL: int("level", 16);  // Internally split by minecraft, static/moving.
    // pub PROP_LIQUID_FALLING: bool("falling");
    pub PROP_IN_WALL: bool("in_wall");
    pub PROP_CONDITIONAL: bool("conditional");
    pub PROP_DRAG: bool("drag");
    pub PROP_PERSISTENT: bool("persistent");
    pub PROP_LEAVES_DISTANCE: range("distance", 1, 7); // Real is 1 to 7
    pub PROP_CANDLES: range("candles", 1, 4);  // Real is 1 to 4
    pub PROP_BERRIES: bool("berries");
    pub PROP_LIGHT_LEVEL: int("level", 16);

    pub PROP_DOWN: bool("down");
    pub PROP_EAST: bool("east");
    pub PROP_NORTH: bool("north");
    pub PROP_SOUTH: bool("south");
    pub PROP_UP: bool("up");
    pub PROP_WEST: bool("west");
    pub PROP_BOTTOM: bool("bottom");

    pub PROP_HORIZONTAL_FACING: enum("facing", Direction, [East, North, South, West]);
    pub PROP_HOPPER_FACING: enum("facing", Direction, [Down, East, North, South, West]);
    pub PROP_VERTICAL_DIRECTION: enum("vertical_direction", Direction, [Up, Down]);
    pub PROP_FACING: enum("facing", Direction, [Down, East, North, South, Up, West]);

    pub PROP_AXIS: enum("axis", Axis, [X, Y, Z]);
    pub PROP_HORIZONTAL_AXIS: enum("axis", Axis, [X, Z]);

    pub PROP_BAMBOO_LEAVES: enum("leaves", BambooLeaves, [Large, None, Small]);
    pub PROP_BED_PART: enum("part", BedPart, [Foot, Head]);

    pub PROP_BELL_ATTACHMENT: enum("attachment", Face, [
        Ceiling, DoubleWall, Floor, SingleWall
    ]);

    pub PROP_FACE: enum("face", Face, [Ceiling, Floor, Wall]);
    pub PROP_DOUBLE_BLOCK_HALF: enum("half", DoubleBlockHalf, [Lower, Upper]);
    pub PROP_DOOR_HINGE: enum("hinge", DoorHingeSide, [Left, Right]);
    pub PROP_HALF: enum("half", Half, [Top, Bottom]);

    pub PROP_COLOR: enum("color", DyeColor, [
        White,
        Orange,
        Magenta,
        LightBlue,
        Yellow,
        Lime,
        Pink,
        Gray,
        LightGray,
        Cyan,
        Purple,
        Blue,
        Brown,
        Green,
        Red,
        Black
    ]);

    pub PROP_INSTRUMENT: enum("instrument", Instrument, [
        Banjo,
        BassDrum,
        Bass,
        Bell,
        Bit,
        Chime,
        CowBell,
        Didjeridoo,
        Flute,
        Guitar,
        Harp,
        Hat,
        IronXylophone,
        Pling,
        Snare,
        Xylophone
    ]);

    pub PROP_RAIL_SHAPE: enum("shape", RailShape, [
        EastWest,
        NorthEast,
        NorthSouth,
        NorthWest,
        SouthEast,
        SouthWest,
        AscendingEast,
        AscendingNorth,
        AscendingSouth,
        AscendingWest
    ]);

    pub PROP_RAIL_SHAPE_SPECIAL: enum("shape", RailShape, [
        EastWest, NorthSouth, AscendingEast, AscendingNorth, AscendingSouth, AscendingWest
    ]);

    pub PROP_COMPARATOR_MODE: enum("mode", ComparatorMode, [
        Compare, Subtract
    ]);

    pub PROP_REDSTONE_EAST: enum("east", RedstoneWireMode, REDSTONE_MODE);
    pub PROP_REDSTONE_NORTH: enum("north", RedstoneWireMode, REDSTONE_MODE);
    pub PROP_REDSTONE_SOUTH: enum("south", RedstoneWireMode, REDSTONE_MODE);
    pub PROP_REDSTONE_WEST: enum("west", RedstoneWireMode, REDSTONE_MODE);

    pub PROP_WALL_EAST: enum("east", WallSide, WALL_SIDE);
    pub PROP_WALL_NORTH: enum("north", WallSide, WALL_SIDE);
    pub PROP_WALL_SOUTH: enum("south", WallSide, WALL_SIDE);
    pub PROP_WALL_WEST: enum("west", WallSide, WALL_SIDE);

    pub PROP_CHEST_TYPE: enum("type", ChestType, [Single, Left, Right]);
    pub PROP_SLAB_TYPE: enum("type", SlabType, [Top, Bottom, Double]);

    pub PROP_PISTON_TYPE: enum("type", PistonType, [Normal, Sticky]);

    /*pub PROP_COMMAND_BLOCK_TYPE: enum("type", CommandBlockType, [
        Impulse, Repeating, Chain
    ]);*/

    pub PROP_STRUCTURE_MODE: enum("mode", StructureMode, [
        Save, Load, Corner, Data
    ]);

    pub PROP_CORAL_TYPE: enum("type", CoralType, [
        Tube, Brain, Bubble, Fire, Horn
    ]);

    pub PROP_STAIRS_SHAPE: enum("shape", StairsShape, [
        Straight, InnerLeft, InnerRight, OuterLeft, OuterRight
    ]);

    pub PROP_JIGSAW_ORIENTATION: enum("orientation", FrontAndTop, [
        DownEast,
        DownNorth,
        DownSouth,
        DownWest,
        UpEast,
        UpNorth,
        UpSouth,
        UpWest,
        WestUp,
        EastUp,
        NorthUp,
        SouthUp
    ]);

    pub PROP_SCULK_SENSOR_PHASE: enum("sculk_sensor_phase", SculkSensorPhase, [
        Inactive,
        Active,
        Cooldown
    ]);

    /*pub PROP_OXYDATION_STATE: enum("oxydation_state", OxydationState, [
        Unaffected,
        Exposed,
        Weathered,
        Oxidized
    ]);*/

    pub PROP_DRIPSTONE_THICKNESS: enum("thickness", DripstoneThickness, [
        TipMerge,
        Tip,
        Frustum,
        Middle,
        Base
    ]);

    pub PROP_DRIPLEAF_TILT: enum("tilt", DripleafTilt, [
        None,
        Unstable,
        Partial,
        Full
    ]);

}


blocks_specs! {

    pub SPEC_GRASS: [PROP_SNOWY];
    pub SPEC_LEAVES: [PROP_LEAVES_DISTANCE, PROP_PERSISTENT];
    pub SPEC_FARMLAND: [PROP_FARMLAND_MOISTURE];
    pub SPEC_SNOW: [PROP_SNOW_LAYERS];
    pub SPEC_SAPLING: [PROP_SAPLING_STAGE];
    pub SPEC_VINE: [PROP_UP, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST];
    // pub SPEC_CORAL_BLOCK: [PROP_CORAL_TYPE];
    // pub SPEC_CORAL: [PROP_WATERLOGGED, PROP_CORAL_TYPE];
    // pub SPEC_CORAL_WALL: [PROP_WATERLOGGED, PROP_FACING, PROP_CORAL_TYPE];
    pub SPEC_SEA_PICKLE: [PROP_PICKLES, PROP_WATERLOGGED];
    pub SPEC_BAMBOO: [PROP_BAMBOO_AGE, PROP_BAMBOO_LEAVES, PROP_BAMBOO_STAGE];
    pub SPEC_BIG_DRIPLEAF: [PROP_WATERLOGGED, PROP_HORIZONTAL_FACING, PROP_DRIPLEAF_TILT];
    pub SPEC_GLOW_LICHEN: [PROP_WATERLOGGED, PROP_UP, PROP_DOWN, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST];

    pub SPEC_CROP: [PROP_AGE_8];
    pub SPEC_BEETROOTS: [PROP_AGE_4];
    pub SPEC_CACTUS: [PROP_AGE_16];
    pub SPEC_SUGAR_CANE: [PROP_AGE_16];
    pub SPEC_NETHER_WART: [PROP_AGE_4];
    pub SPEC_COCOA: [PROP_AGE_3, PROP_HORIZONTAL_FACING];
    pub SPEC_DOUBLE_PLANT: [PROP_DOUBLE_BLOCK_HALF];
    pub SPEC_CHORUS_FLOWER: [PROP_AGE_6];
    pub SPEC_KELP: [PROP_AGE_26];
    pub SPEC_NETHER_VINE: [PROP_AGE_26];
    pub SPEC_SWEET_BERRY_BUSH: [PROP_AGE_4];
    pub SPEC_CAVE_VINES: [PROP_BERRIES, PROP_AGE_26];
    pub SPEC_CAVE_VINES_PLANT: [PROP_BERRIES];
    pub SPEC_SMALL_DRIPLEAF: [PROP_DOUBLE_BLOCK_HALF, PROP_WATERLOGGED, PROP_HORIZONTAL_FACING];

    pub SPEC_LIQUID: [PROP_LIQUID_LEVEL];

    pub SPEC_DISPENSER: [PROP_FACING, PROP_TRIGGERED];
    pub SPEC_DROPPER: [PROP_FACING, PROP_TRIGGERED];
    pub SPEC_OBSERVER: [PROP_FACING, PROP_POWERED];
    pub SPEC_FURNACE_LIKE: [PROP_HORIZONTAL_FACING, PROP_LIT];
    pub SPEC_BARREL: [PROP_FACING, PROP_OPEN];
    pub SPEC_NOTE_BLOCK: [PROP_INSTRUMENT, PROP_NOTE, PROP_POWERED];
    pub SPEC_BED: [/*PROP_COLOR,*/ PROP_HORIZONTAL_FACING, PROP_BED_PART, PROP_OCCUPIED];
    pub SPEC_BREWING_STAND: [PROP_HAS_BOTTLE_0, PROP_HAS_BOTTLE_1, PROP_HAS_BOTTLE_2];
    pub SPEC_BUTTON: [PROP_HORIZONTAL_FACING, PROP_POWERED, PROP_FACE];
    pub SPEC_CHEST: [PROP_HORIZONTAL_FACING, PROP_CHEST_TYPE, PROP_WATERLOGGED];
    pub SPEC_ENDER_CHEST: [PROP_HORIZONTAL_FACING, PROP_WATERLOGGED];
    pub SPEC_REDSTONE_WIRE: [PROP_REDSTONE_POWER, PROP_REDSTONE_EAST, PROP_REDSTONE_NORTH, PROP_REDSTONE_SOUTH, PROP_REDSTONE_WEST];
    pub SPEC_LEVER: [PROP_FACE, PROP_HORIZONTAL_FACING, PROP_POWERED];
    pub SPEC_PRESSURE_PLATE: [PROP_POWERED];
    pub SPEC_DOOR: [PROP_DOUBLE_BLOCK_HALF, PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_DOOR_HINGE, PROP_POWERED];
    pub SPEC_WALL_REDSTONE_TORCH: [PROP_HORIZONTAL_FACING, PROP_LIT];
    pub SPEC_JUKEBOX: [PROP_HAS_RECORD];
    pub SPEC_REPEATER: [PROP_REPEATER_DELAY, PROP_HORIZONTAL_FACING, PROP_LOCKED, PROP_POWERED];
    pub SPEC_TRAPDOOR: [PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_HALF, PROP_POWERED, PROP_WATERLOGGED];
    pub SPEC_CAULDRON_LEVEL: [PROP_CAULDRON_LEVEL];
    pub SPEC_TRIPWIRE_HOOK: [PROP_ATTACHED, PROP_HORIZONTAL_FACING, PROP_POWERED];
    pub SPEC_TRIPWIRE: [PROP_ATTACHED, PROP_DISARMED, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_WEST, PROP_POWERED];
    pub SPEC_COMMAND_BLOCK: [PROP_FACING, PROP_CONDITIONAL];
    pub SPEC_COMPARATOR: [PROP_HORIZONTAL_FACING, PROP_COMPARATOR_MODE, PROP_POWERED];
    pub SPEC_DAYLIGHT_DETECTOR: [PROP_INVERTED, PROP_REDSTONE_POWER];
    pub SPEC_HOPPER: [PROP_HOPPER_FACING, PROP_ENABLED];
    pub SPEC_SHULKER_BOX: [PROP_FACING];
    // pub SPEC_COLORED_SHULKER_BOX: [PROP_FACING, PROP_COLOR];
    pub SPEC_TURTLE_EGG: [PROP_EGGS, PROP_HATCH];
    pub SPEC_GRINDSTONE: [PROP_FACE, PROP_HORIZONTAL_FACING];
    pub SPEC_LECTERN: [PROP_HORIZONTAL_FACING, PROP_HAS_BOOK, PROP_POWERED];
    pub SPEC_BELL: [PROP_BELL_ATTACHMENT, PROP_HORIZONTAL_FACING, PROP_POWERED];
    pub SPEC_RESPAWN_ANCHOR: [PROP_CHARGES];
    pub SPEC_COMPOSTER: [PROP_COMPOSTER_LEVEL];
    pub SPEC_STRUCTURE_BLOCK: [PROP_STRUCTURE_MODE];
    pub SPEC_JIGSAW: [PROP_JIGSAW_ORIENTATION];
    pub SPEC_LIGHT: [PROP_LIGHT_LEVEL, PROP_WATERLOGGED];

    pub SPEC_RAIL_SPECIAL: [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED, PROP_WATERLOGGED];
    pub SPEC_RAIL: [PROP_RAIL_SHAPE, PROP_WATERLOGGED];

    pub SPEC_PISTON: [PROP_FACING, PROP_EXTENDED];
    pub SPEC_PISTON_HEAD: [PROP_FACING, PROP_SHORT, PROP_PISTON_TYPE];
    pub SPEC_MOVING_PISTON: [PROP_FACING, PROP_PISTON_TYPE];

    pub SPEC_TNT: [PROP_UNSTABLE];
    pub SPEC_WALL_TORCH: [PROP_HORIZONTAL_FACING];
    pub SPEC_FIRE: [PROP_AGE_16, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST, PROP_UP];
    pub SPEC_STAIRS: [PROP_HORIZONTAL_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED];
    pub SPEC_SLAB: [PROP_SLAB_TYPE, PROP_WATERLOGGED];
    pub SPEC_SIGN: [PROP_ROTATION, PROP_WATERLOGGED];
    pub SPEC_WALL_SIGN: [PROP_HORIZONTAL_FACING, PROP_WATERLOGGED];
    pub SPEC_LADDER: [PROP_HORIZONTAL_FACING, PROP_WATERLOGGED];
    pub SPEC_NETHER_PORTAL: [PROP_HORIZONTAL_AXIS];
    pub SPEC_CAKE: [PROP_CAKE_BITES];
    pub SPEC_BARS: [PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED];
    // pub SPEC_COLORED_BARS: [PROP_COLOR, PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED];
    pub SPEC_WALL: [PROP_UP, PROP_WALL_EAST, PROP_WALL_NORTH, PROP_WALL_SOUTH, PROP_WALL_WEST, PROP_WATERLOGGED];
    pub SPEC_FENCE_GATE: [PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_POWERED, PROP_IN_WALL];
    pub SPEC_END_PORTAL: [PROP_END_PORTAL_EYE, PROP_HORIZONTAL_FACING];
    pub SPEC_SKULL: [PROP_ROTATION];
    pub SPEC_WALL_SKULL: [PROP_HORIZONTAL_FACING];
    pub SPEC_BANNER: [PROP_ROTATION/*, PROP_COLOR*/];
    pub SPEC_WALL_BANNER: [PROP_HORIZONTAL_FACING/*, PROP_COLOR*/];
    pub SPEC_FROSTED_ICE: [PROP_AGE_4];
    pub SPEC_BUBBLE_COLUMN: [PROP_DRAG];
    pub SPEC_SCAFFOLDING: [PROP_BOTTOM, PROP_SCAFFOLDING_DISTANCE, PROP_WATERLOGGED];
    pub SPEC_LANTERN: [PROP_HANGING, PROP_WATERLOGGED];
    pub SPEC_CAMPFIRE: [PROP_HORIZONTAL_FACING, PROP_LIT, PROP_SIGNAL_FIRE, PROP_WATERLOGGED];
    pub SPEC_BEEHIVE: [PROP_HORIZONTAL_FACING, PROP_HONEY_LEVEL];
    // pub SPEC_GLAZED_TERRACOTA: [/*PROP_COLOR, */PROP_HORIZONTAL_FACING];
    pub SPEC_CANDLE: [PROP_CANDLES, PROP_LIT, PROP_WATERLOGGED];
    // pub SPEC_COLORED_CANDLE: [PROP_COLOR, PROP_CANDLES, PROP_LIT, PROP_WATERLOGGED];
    pub SPEC_SCULK_SENSOR: [PROP_SCULK_SENSOR_PHASE, PROP_REDSTONE_POWER, PROP_WATERLOGGED];
    pub SPEC_LIGHTNING_ROD: [PROP_FACING, PROP_POWERED, PROP_WATERLOGGED];
    pub SPEC_POINTED_DRIPSTONE: [PROP_VERTICAL_DIRECTION, PROP_DRIPSTONE_THICKNESS, PROP_WATERLOGGED];

    // pub SPEC_COPPER_BLOCK: [PROP_OXYDATION_STATE];
    // pub SPEC_COPPER_STAIRS: [PROP_OXYDATION_STATE, PROP_HORIZONTAL_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED];
    // pub SPEC_COPPER_SLAB: [PROP_OXYDATION_STATE, PROP_SLAB_TYPE];

    pub SPEC_WATERLOGGED: [PROP_WATERLOGGED];
    // pub SPEC_COLORED: [PROP_COLOR];
    pub SPEC_AXIS: [PROP_AXIS];
    pub SPEC_AXIS_WATERLOGGED: [PROP_WATERLOGGED, PROP_AXIS];
    pub SPEC_HORIZONTAL_FACING: [PROP_HORIZONTAL_FACING];
    pub SPEC_FACING: [PROP_FACING];
    pub SPEC_FACING_WATERLOGGED: [PROP_WATERLOGGED, PROP_FACING];
    pub SPEC_HORIZONTAL_FACING_WATERLOGGED: [PROP_WATERLOGGED, PROP_HORIZONTAL_FACING];
    pub SPEC_LIT: [PROP_LIT];
    // pub SPEC_COLORED_LIT: [PROP_LIT, PROP_COLOR];
    pub SPEC_REDSTONE_POWER: [PROP_REDSTONE_POWER];
    pub SPEC_MULTIFACE: [PROP_UP, PROP_DOWN, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST];

}


// Same order as defined in MC code
// Some block has been merged to avoid defining dozen of variations
// for example, for compatibility with Minecraft these blocks may need
// extensions or a specified module for the conversion.
blocks!(pub VANILLA_BLOCKS "minecraft" [

    AIR "air", // Moved here to be the first default block

    STONE "stone",
    GRANITE "granite",
    POLISHED_GRANITE "polished_granite",
    DIORITE "diorite",
    POLISHED_DIORITE "polished_diorite",
    ANDESITE "andesite",
    POLISHED_ANDESITE "polished_andesite",
    GRASS_BLOCK "grass_block" SPEC_GRASS,
    PODZOL "podzol" SPEC_GRASS,
    DIRT "dirt",
    COARSE_DIRT "coarse_dirt",
    COBBLESTONE "cobblestone",

    OAK_PLANKS "oak_planks",
    SPRUCE_PLANKS "spruce_planks",
    BIRCH_PLANKS "birch_planks",
    JUNGLE_PLANKS "jungle_planks",
    ACACIA_PLANKS "acacia_planks",
    DARK_OAK_PLANKS "dark_oak_planks",

    OAK_SAPLING "oak_sapling" SPEC_SAPLING,
    SPRUCE_SAPLING "spruce_sapling" SPEC_SAPLING,
    BIRCH_SAPLING "birch_sapling" SPEC_SAPLING,
    JUNGLE_SAPLING "jungle_sapling" SPEC_SAPLING,
    ACACIA_SAPLING "acacia_sapling" SPEC_SAPLING,
    DARK_OAK_SAPLING "dark_oak_sapling" SPEC_SAPLING,

    BEDROCK "bedrock",
    WATER "water" SPEC_LIQUID,
    LAVA "lava" SPEC_LIQUID,
    SAND "sand",
    RED_SAND "red_sand",
    GRAVEL "gravel",
    GOLD_ORE "gold_ore",
    DEEPSLATE_GOLD_ORE "deepslate_gold_ore",
    IRON_ORE "iron_ore",
    DEEPSLATE_IRON_ORE "deepslate_iron_ore",
    COAL_ORE "coal_ore",
    DEEPSLATE_COAL_ORE "deepslate_coal_ore",
    NETHER_GOLD_ORE "nether_gold_ore",

    OAK_LOG "oak_log" SPEC_AXIS,
    SPRUCE_LOG "spruce_log" SPEC_AXIS,
    BIRCH_LOG "birch_log" SPEC_AXIS,
    JUNGLE_LOG "jungle_log" SPEC_AXIS,
    ACACIA_LOG "acacia_log" SPEC_AXIS,
    DARK_OAK_LOG "dark_oak_log" SPEC_AXIS,

    STRIPPED_OAK_LOG "stripped_oak_log" SPEC_AXIS,
    STRIPPED_SPRUCE_LOG "stripped_spruce_log" SPEC_AXIS,
    STRIPPED_BIRCH_LOG "stripped_birch_log" SPEC_AXIS,
    STRIPPED_JUNGLE_LOG "stripped_jungle_log" SPEC_AXIS,
    STRIPPED_ACACIA_LOG "stripped_acacia_log" SPEC_AXIS,
    STRIPPED_DARK_OAK_LOG "stripped_dark_oak_log" SPEC_AXIS,

    OAK_WOOD "oak_wood" SPEC_AXIS,
    SPRUCE_WOOD "spruce_wood" SPEC_AXIS,
    BIRCH_WOOD "birch_wood" SPEC_AXIS,
    JUNGLE_WOOD "jungle_wood" SPEC_AXIS,
    ACACIA_WOOD "acacia_wood" SPEC_AXIS,
    DARK_OAK_WOOD "dark_oak_wood" SPEC_AXIS,

    STRIPPED_OAK_WOOD "stripped_oak_wood" SPEC_AXIS,
    STRIPPED_SPRUCE_WOOD "stripped_spruce_wood" SPEC_AXIS,
    STRIPPED_BIRCH_WOOD "stripped_birch_wood" SPEC_AXIS,
    STRIPPED_JUNGLE_WOOD "stripped_jungle_wood" SPEC_AXIS,
    STRIPPED_ACACIA_WOOD "stripped_acacia_wood" SPEC_AXIS,
    STRIPPED_DARK_OAK_WOOD "stripped_dark_oak_wood" SPEC_AXIS,

    OAK_LEAVES "oak_leaves" SPEC_LEAVES,
    SPRUCE_LEAVES "spruce_leaves" SPEC_LEAVES,
    BIRCH_LEAVES "birch_leaves" SPEC_LEAVES,
    JUNGLE_LEAVES "jungle_leaves" SPEC_LEAVES,
    ACACIA_LEAVES "acacia_leaves" SPEC_LEAVES,
    DARK_OAK_LEAVES "dark_oak_leaves" SPEC_LEAVES,
    AZALEA_LEAVES "azalea_leaves" SPEC_LEAVES,
    FLOWERING_AZALEA_LEAVES "flowering_azalea_leaves" SPEC_LEAVES,

    SPONGE "sponge",
    WET_SPONGE "wet_sponge",
    GLASS "glass",
    LAPIS_ORE "lapis_ore",
    DEEPSLATE_LAPIS_ORE "deepslate_lapis_ore",
    LAPIS_BLOCK "lapis_block",
    DISPENSER "dispenser" SPEC_DISPENSER,
    SANDSTONE "sandstone",
    CHISELED_SANDSTONE "chiseled_sandstone",
    CUT_SANDSTONE "cut_sandstone",
    NOTE_BLOCK "note_block" SPEC_NOTE_BLOCK,

    // BED "bed" SPEC_BED,
    WHITE_BED "white_bed" SPEC_BED,
    ORANGE_BED "orange_bed" SPEC_BED,
    MAGENTA_BED "magenta_bed" SPEC_BED,
    LIGHT_BLUE_BED "light_blue_bed" SPEC_BED,
    YELLOW_BED "yellow_bed" SPEC_BED,
    LIME_BED "lime_bed" SPEC_BED,
    PINK_BED "pink_bed" SPEC_BED,
    GRAY_BED "gray_bed" SPEC_BED,
    LIGHT_GRAY_BED "light_gray_bed" SPEC_BED,
    CYAN_BED "cyan_bed" SPEC_BED,
    PURPLE_BED "purple_bed" SPEC_BED,
    BLUE_BED "blue_bed" SPEC_BED,
    BROWN_BED "brown_bed" SPEC_BED,
    GREEN_BED "green_bed" SPEC_BED,
    RED_BED "red_bed" SPEC_BED,
    BLACK_BED "black_bed" SPEC_BED,

    POWERED_RAIL "powered_rail" SPEC_RAIL_SPECIAL,
    DETECTOR_RAIL "detector_rail" SPEC_RAIL_SPECIAL,

    PISTON "piston" SPEC_PISTON,
    STICKY_PISTON "sticky_piston" SPEC_PISTON,
    PISTON_HEAD "piston_head" SPEC_PISTON_HEAD,
    MOVING_PISTON "moving_piston" SPEC_MOVING_PISTON,

    COBWEB "cobweb",
    GRASS "grass",
    FERN "fern",
    DEAD_BUSH "dead_bush",
    SEAGRASS "seagrass",
    TALL_SEAGRASS "tall_seagrass" SPEC_DOUBLE_PLANT,

    // WOOL "wool" SPEC_COLORED,
    WHITE_WOOL "white_wool",
    ORANGE_WOOL "orange_wool",
    MAGENTA_WOOL "magenta_wool",
    LIGHT_BLUE_WOOL "light_blue_wool",
    YELLOW_WOOL "yellow_wool",
    LIME_WOOL "lime_wool",
    PINK_WOOL "pink_wool",
    GRAY_WOOL "gray_wool",
    LIGHT_GRAY_WOOL "light_gray_wool",
    CYAN_WOOL "cyan_wool",
    PURPLE_WOOL "purple_wool",
    BLUE_WOOL "blue_wool",
    BROWN_WOOL "brown_wool",
    GREEN_WOOL "green_wool",
    RED_WOOL "red_wool",
    BLACK_WOOL "black_wool",

    DANDELION "dandelion",
    POPPY "poppy",
    BLUE_ORCHID "blue_orchid",
    ALLIUM "allium",
    AZURE_BLUET "azure_bluet",
    RED_TULIP "red_tulip",
    ORANGE_TULIP "orange_tulip",
    WHITE_TULIP "white_tulip",
    PINK_TULIP "pink_tulip",
    OXEYE_DAISY "oxeye_daisy",
    CORNFLOWER "cornflower",
    WITHER_ROSE "wither_rose",
    LILY_OF_THE_VALLEY "lily_of_the_valley",
    BROWN_MUSHROOM "brown_mushroom",
    RED_MUSHROOM "red_mushroom",
    GOLD_BLOCK "gold_block",
    IRON_BLOCK "iron_block",
    BRICKS "bricks",
    TNT "tnt" SPEC_TNT,
    BOOKSHELF "bookshelf",
    MOSSY_COBBLESTONE "mossy_cobblestone",
    OBSIDIAN "obsidian",
    TORCH "torch",
    WALL_TORCH "wall_torch" SPEC_WALL_TORCH,
    FIRE "fire" SPEC_FIRE,
    SOUL_FIRE "soul_fire" SPEC_FIRE,
    SPAWNER "spawner",

    OAK_STAIRS "oak_stairs" SPEC_STAIRS,
    SPRUCE_STAIRS "spruce_stairs" SPEC_STAIRS,
    BIRCH_STAIRS "birch_stairs" SPEC_STAIRS,
    JUNGLE_STAIRS "jungle_stairs" SPEC_STAIRS,
    ACACIA_STAIRS "acacia_stairs" SPEC_STAIRS,
    DARK_OAK_STAIRS "dark_oak_stairs" SPEC_STAIRS,

    CHEST "chest" SPEC_CHEST,
    REDSTONE_WIRE "redstone_wire" SPEC_REDSTONE_WIRE,
    DIAMOND_ORE "diamond_ore",
    DEEPSLATE_DIAMOND_ORE "deepslate_diamond_ore",
    DIAMOND_BLOCK "diamond_block",
    CRAFTING_TABLE "crafting_table",
    WHEAT "wheat" SPEC_CROP,
    FARMLAND "farmland" SPEC_FARMLAND,
    FURNACE "furnace" SPEC_FURNACE_LIKE,

    OAK_SIGN "oak_sign" SPEC_SIGN,
    SPRUCE_SIGN "spruce_sign" SPEC_SIGN,
    BIRCH_SIGN "birch_sign" SPEC_SIGN,
    JUNGLE_SIGN "jungle_sign" SPEC_SIGN,
    ACACIA_SIGN "acacia_sign" SPEC_SIGN,
    DARK_OAK_SIGN "dark_oak_sign" SPEC_SIGN,

    LADDER "ladder" SPEC_LADDER,
    RAIL "rail" SPEC_RAIL,
    COBBLESTONE_STAIRS "cobblestone_stairs" SPEC_STAIRS,

    OAK_WALL_SIGN "oak_wall_sign" SPEC_WALL_SIGN,
    SPRUCE_WALL_SIGN "spruce_wall_sign" SPEC_WALL_SIGN,
    BIRCH_WALL_SIGN "birch_wall_sign" SPEC_WALL_SIGN,
    JUNGLE_WALL_SIGN "jungle_wall_sign" SPEC_WALL_SIGN,
    ACACIA_WALL_SIGN "acacia_wall_sign" SPEC_WALL_SIGN,
    DARK_OAK_WALL_SIGN "dark_oak_wall_sign" SPEC_WALL_SIGN,

    LEVER "lever" SPEC_LEVER,
    STONE_PRESSURE_PLATE "stone_pressure_plate" SPEC_PRESSURE_PLATE,
    IRON_DOOR "iron_door" SPEC_DOOR,

    OAK_PRESSURE_PLATE "oak_pressure_plate" SPEC_PRESSURE_PLATE,
    SPRUCE_PRESSURE_PLATE "spruce_pressure_plate" SPEC_PRESSURE_PLATE,
    BIRCH_PRESSURE_PLATE "birch_pressure_plate" SPEC_PRESSURE_PLATE,
    JUNGLE_PRESSURE_PLATE "jungle_pressure_plate" SPEC_PRESSURE_PLATE,
    ACACIA_PRESSURE_PLATE "acacia_pressure_plate" SPEC_PRESSURE_PLATE,
    DARK_OAK_PRESSURE_PLATE "dark_oak_pressure_plate" SPEC_PRESSURE_PLATE,

    REDSTONE_ORE "redstone_ore" SPEC_LIT,
    DEEPSLATE_REDSTONE_ORE "deepslate_redstone_ore" SPEC_LIT,
    REDSTONE_TORCH "redstone_torch" SPEC_LIT,
    REDSTONE_WALL_TORCH "redstone_wall_torch" SPEC_WALL_REDSTONE_TORCH,
    STONE_BUTTON "stone_button" SPEC_BUTTON,
    SNOW "snow" SPEC_SNOW,
    ICE "ice",
    SNOW_BLOCK "snow_block",
    CACTUS "cactus" SPEC_CACTUS,
    CLAY "clay",
    SUGAR_CANE "sugar_cane" SPEC_SUGAR_CANE,
    JUKEBOX "jukebox" SPEC_JUKEBOX,
    PUMPKIN "pumpkin",
    NETHERRACK "netherrack",
    SOUL_SAND "soul_sand",
    SOUL_SOIL "soul_soil",
    BASALT "basalt" SPEC_AXIS,
    POLISHED_BASALT "polished_basalt" SPEC_AXIS,
    SOUL_TORCH "soul_torch",
    SOUL_WALL_TORCH "soul_wall_torch" SPEC_WALL_TORCH,
    GLOWSTONE "glowstone",
    NETHER_PORTAL "nether_portal" SPEC_NETHER_PORTAL,
    CARVED_PUMPKIN "carved_pumpkin" SPEC_HORIZONTAL_FACING,
    JACK_O_LANTERN "jack_o_lantern" SPEC_HORIZONTAL_FACING,
    CAKE "cake" SPEC_CAKE,
    REPEATER "repeater" SPEC_REPEATER,

    // STAINED_GLASS "stained_glass" SPEC_COLORED,
    WHITE_STAINED_GLASS "white_stained_glass",
    ORANGE_STAINED_GLASS "orange_stained_glass",
    MAGENTA_STAINED_GLASS "magenta_stained_glass",
    LIGHT_BLUE_STAINED_GLASS "light_blue_stained_glass",
    YELLOW_STAINED_GLASS "yellow_stained_glass",
    LIME_STAINED_GLASS "lime_stained_glass",
    PINK_STAINED_GLASS "pink_stained_glass",
    GRAY_STAINED_GLASS "gray_stained_glass",
    LIGHT_GRAY_STAINED_GLASS "light_gray_stained_glass",
    CYAN_STAINED_GLASS "cyan_stained_glass",
    PURPLE_STAINED_GLASS "purple_stained_glass",
    BLUE_STAINED_GLASS "blue_stained_glass",
    BROWN_STAINED_GLASS "brown_stained_glass",
    GREEN_STAINED_GLASS "green_stained_glass",
    RED_STAINED_GLASS "red_stained_glass",
    BLACK_STAINED_GLASS "black_stained_glass",

    OAK_TRAPDOOR "oak_trapdoor" SPEC_TRAPDOOR,
    SPRUCE_TRAPDOOR "spruce_trapdoor" SPEC_TRAPDOOR,
    BIRCH_TRAPDOOR "birch_trapdoor" SPEC_TRAPDOOR,
    JUNGLE_TRAPDOOR "jungle_trapdoor" SPEC_TRAPDOOR,
    ACACIA_TRAPDOOR "acacia_trapdoor" SPEC_TRAPDOOR,
    DARK_OAK_TRAPDOOR "dark_oak_trapdoor" SPEC_TRAPDOOR,

    STONE_BRICKS "stone_bricks",
    MOSSY_STONE_BRICKS "mossy_stone_bricks",
    CRACKED_STONE_BRICKS "cracked_stone_bricks",
    CHISELED_STONE_BRICKS "chiseled_stone_bricks",
    INFESTED_STONE "infested_stone",
    INFESTED_COBBLESTONE "infested_cobblestone",
    INFESTED_STONE_BRICKS "infested_stone_bricks",
    INFESTED_MOSSY_STONE_BRICKS "infested_mossy_stone_bricks",
    INFESTED_CRACKED_STONE_BRICKS "infested_cracked_stone_bricks",
    INFESTED_CHISELED_STONE_BRICKS "infested_chiseled_stone_bricks",
    BROWN_MUSHROOM_BLOCK "brown_mushroom_block" SPEC_MULTIFACE,
    RED_MUSHROOM_BLOCK "red_mushroom_block" SPEC_MULTIFACE,
    MUSHROOM_STEM "mushroom_stem" SPEC_MULTIFACE,
    IRON_BARS "iron_bars" SPEC_BARS,
    CHAIN "chain" SPEC_AXIS_WATERLOGGED,
    GLASS_PANE "glass_pane" SPEC_BARS,
    MELON "melon",
    ATTACHED_PUMPKIN_STEM "attached_pumpkin_stem" SPEC_HORIZONTAL_FACING,
    ATTACHED_MELON_STEM "attached_melon_stem" SPEC_HORIZONTAL_FACING,
    PUMPKIN_STEM "pumpkin_stem" SPEC_CROP,
    MELON_STEM "melon_stem" SPEC_CROP,
    VINE "vine" SPEC_VINE,
    GLOW_LICHEN "glow_lichen" SPEC_GLOW_LICHEN,
    BRICK_STAIRS "brick_stairs" SPEC_STAIRS,
    STONE_BRICK_STAIRS "stone_brick_stairs" SPEC_STAIRS,
    MYCELIUM "mycelium" SPEC_GRASS,
    LILY_PAD "lily_pad",
    NETHER_BRICKS "nether_bricks",
    NETHER_BRICK_FENCE "nether_brick_fence" SPEC_BARS,
    NETHER_BRICK_STAIRS "nether_brick_stairs" SPEC_STAIRS,
    NETHER_WART "nether_wart" SPEC_NETHER_WART,

    ENCHANTING_TABLE "enchanting_table",
    BREWING_STAND "brewing_stand" SPEC_BREWING_STAND,

    CAULDRON "cauldron",
    WATER_CAULDRON "water_cauldron" SPEC_CAULDRON_LEVEL,
    LAVA_CAULDRON "lava_cauldron",
    POWDER_SNOW_CAULDRON "powder_snow_cauldron" SPEC_CAULDRON_LEVEL,

    END_PORTAL "end_portal",
    END_PORTAL_FRAME "end_portal_frame" SPEC_END_PORTAL,
    END_STONE "end_stone",
    DRAGON_EGG "dragon_egg",

    REDSTONE_LAMP "redstone_lamp" SPEC_LIT,
    COCOA "cocoa" SPEC_COCOA,
    SANDSTONE_STAIRS "sandstone_stairs" SPEC_STAIRS,
    EMERALD_ORE "emerald_ore",
    DEEPSLATE_EMERALD_ORE "deepslate_emerald_ore",
    ENDER_CHEST "ender_chest" SPEC_ENDER_CHEST,
    TRIPWIRE_HOOK "tripwire_hook" SPEC_TRIPWIRE_HOOK,
    TRIPWIRE "tripwire" SPEC_TRIPWIRE,
    EMERALD_BLOCK "emerald_block",
    COMMAND_BLOCK "command_block" SPEC_COMMAND_BLOCK,
    CHAIN_COMMAND_BLOCK "chain_command_block" SPEC_COMMAND_BLOCK,
    REPEATING_COMMAND_BLOCK "repeating_command_block" SPEC_COMMAND_BLOCK,
    BEACON "beacon",
    COBBLESTONE_WALL "cobblestone_wall" SPEC_WALL,
    MOSSY_COBBLESTONE_WALL "mossy_cobblestone_wall" SPEC_WALL,

    FLOWER_POT "flower_pot",
    POTTED_OAK_SAPLING "potted_oak_sapling",
    POTTED_SPRUCE_SAPLING "potted_spruce_sapling",
    POTTED_BIRCH_SAPLING "potted_birch_sapling",
    POTTED_JUNGLE_SAPLING "potted_jungle_sapling",
    POTTED_ACACIA_SAPLING "potted_acacia_sapling",
    POTTED_DARK_OAK_SAPLING "potted_dark_oak_sapling",
    POTTED_FERN "potted_fern",
    POTTED_DANDELION "potted_dandelion",
    POTTED_POPPY "potted_poppy",
    POTTED_BLUE_ORCHID "potted_blue_orchid",
    POTTED_ALLIUM "potted_allium",
    POTTED_AZURE_BLUET "potted_azure_bluet",
    POTTED_RED_TULIP "potted_red_tulip",
    POTTED_ORANGE_TULIP "potted_orange_tulip",
    POTTED_WHITE_TULIP "potted_white_tulip",
    POTTED_PINK_TULIP "potted_pink_tulip",
    POTTED_OXEYE_DAISY "potted_oxeye_daisy",
    POTTED_CORNFLOWER "potted_cornflower",
    POTTED_LILY_OF_THE_VALLEY "potted_lily_of_the_valley",
    POTTED_WITHER_ROSE "potted_wither_rose",
    POTTED_RED_MUSHROOM "potted_red_mushroom",
    POTTED_BROWN_MUSHROOM "potted_brown_mushroom",
    POTTED_DEAD_BUSH "potted_dead_bush",
    POTTED_CACTUS "potted_cactus",

    CARROTS "carrots" SPEC_CROP,
    POTATOES "potatoes" SPEC_CROP,

    OAK_BUTTON "oak_button" SPEC_BUTTON,
    SPRUCE_BUTTON "spruce_button" SPEC_BUTTON,
    BIRCH_BUTTON "birch_button" SPEC_BUTTON,
    JUNGLE_BUTTON "jungle_button" SPEC_BUTTON,
    ACACIA_BUTTON "acacia_button" SPEC_BUTTON,
    DARK_OAK_BUTTON "dark_oak_button" SPEC_BUTTON,

    // Skulls
    SKELETON_SKULL "skeleton_skull" SPEC_SKULL,
    SKELETON_WALL_SKULL "skeleton_wall_skull" SPEC_WALL_SKULL,
    WITHER_SKELETON_SKULL "wither_skeleton_skull" SPEC_SKULL,
    WITHER_SKELETON_WALL_SKULL "wither_skeleton_wall_skull" SPEC_WALL_SKULL,
    ZOMBIE_HEAD "zombie_head" SPEC_SKULL,
    ZOMBIE_WALL_HEAD "zombie_wall_head" SPEC_WALL_SKULL,
    PLAYER_HEAD "player_head" SPEC_SKULL,
    PLAYER_WALL_HEAD "player_wall_head" SPEC_WALL_SKULL,
    CREEPER_HEAD "creeper_head" SPEC_SKULL,
    CREEPER_WALL_HEAD "creeper_wall_head" SPEC_WALL_SKULL,
    DRAGON_HEAD "dragon_head" SPEC_SKULL,
    DRAGON_WALL_HEAD "dragon_wall_head" SPEC_WALL_SKULL,
    // Anvils
    ANVIL "anvil" SPEC_HORIZONTAL_FACING,
    CHIPPED_ANVIL "chipped_anvil" SPEC_HORIZONTAL_FACING,
    DAMAGED_ANVIL "damaged_anvil" SPEC_HORIZONTAL_FACING,
    TRAPPED_CHEST "trapped_chest" SPEC_CHEST,
    LIGHT_WEIGHTED_PRESSURE_PLATE "light_weighted_pressure_plate" SPEC_REDSTONE_POWER,
    HEAVY_WEIGHTED_PRESSURE_PLATE "heavy_weighted_pressure_plate" SPEC_REDSTONE_POWER,
    COMPARATOR "comparator" SPEC_COMPARATOR,
    DAYLIGHT_DETECTOR "daylight_detector" SPEC_DAYLIGHT_DETECTOR,
    REDSTONE_BLOCK "redstone_block",
    NETHER_QUARTZ_ORE "nether_quartz_ore",
    HOPPER "hopper" SPEC_HOPPER,
    QUARTZ_BLOCK "quartz_block",
    CHISELED_QUARTZ_BLOCK "chiseled_quartz_block",
    QUARTZ_PILLAR "quartz_pillar" SPEC_AXIS,
    QUARTZ_STAIRS "quartz_stairs" SPEC_STAIRS,
    ACTIVATOR_RAIL "activator_rail" SPEC_RAIL_SPECIAL,
    DROPPER "dropper" SPEC_DROPPER,
    TERRACOTTA "terracotta",

    // COLORED_TERRACOTTA "colored_terracotta" SPEC_COLORED, // Merged
    WHITE_TERRACOTTA "white_terracotta",
    ORANGE_TERRACOTTA "orange_terracotta",
    MAGENTA_TERRACOTTA "magenta_terracotta",
    LIGTH_BLUE_TERRACOTTA "light_blue_terracotta",
    YELLOW_TERRACOTTA "yellow_terracotta",
    LIME_TERRACOTTA "lime_terracotta",
    PINK_TERRACOTTA "pink_terracotta",
    GRAY_TERRACOTTA "gray_terracotta",
    LIGHT_GRAY_TERRACOTTA "light_gray_terracotta",
    CYAN_TERRACOTTA "cyan_terracotta",
    PURPLE_TERRACOTTA "purple_terracotta",
    BLUE_TERRACOTTA "blue_terracotta",
    BROWN_TERRACOTTA "brown_terracotta",
    GREEN_TERRACOTTA "green_terracotta",
    RED_TERRACOTTA "red_terracotta",
    BLACK_TERRACOTTA "black_terracotta",

    // STAINED_GLASS_PANE "stained_glass_pane" SPEC_COLORED_BARS, // Merged
    WHITE_STAINED_GLASS_PANE "white_stained_glass_pane" SPEC_BARS,
    ORANGE_STAINED_GLASS_PANE "orange_stained_glass_pane" SPEC_BARS,
    MAGENTA_STAINED_GLASS_PANE "magenta_stained_glass_pane" SPEC_BARS,
    LIGHT_BLUE_STAINED_GLASS_PANE "light_blue_stained_glass_pane" SPEC_BARS,
    YELLOW_STAINED_GLASS_PANE "yellow_stained_glass_pane" SPEC_BARS,
    LIME_STAINED_GLASS_PANE "lime_stained_glass_pane" SPEC_BARS,
    PINK_STAINED_GLASS_PANE "pink_stained_glass_pane" SPEC_BARS,
    GRAY_STAINED_GLASS_PANE "gray_stained_glass_pane" SPEC_BARS,
    LIGHT_GRAY_STAINED_GLASS_PANE "light_gray_stained_glass_pane" SPEC_BARS,
    CYAN_STAINED_GLASS_PANE "cyan_stained_glass_pane" SPEC_BARS,
    PURPLE_STAINED_GLASS_PANE "purple_stained_glass_pane" SPEC_BARS,
    BLUE_STAINED_GLASS_PANE "blue_stained_glass_pane" SPEC_BARS,
    BROWN_STAINED_GLASS_PANE "brown_stained_glass_pane" SPEC_BARS,
    GREEN_STAINED_GLASS_PANE "green_stained_glass_pane" SPEC_BARS,
    RED_STAINED_GLASS_PANE "red_stained_glass_pane" SPEC_BARS,
    BLACK_STAINED_GLASS_PANE "black_stained_glass_pane" SPEC_BARS,

    SLIME_BLOCK "slime_block",
    BARRIER "barrier",
    LIGHT "light" SPEC_LIGHT,
    IRON_TRAPDOOR "iron_trapdoor" SPEC_TRAPDOOR,
    PRISMARINE "prismarine",
    PRISMARINE_BRICKS "prismarine_bricks",
    DARK_PRISMARINE "dark_prismarine",
    PRISMARINE_STAIRS "prismarine_stairs" SPEC_STAIRS,
    PRISMARINE_BRICK_STAIRS "prismarine_brick_stairs" SPEC_STAIRS,
    DARK_PRISMARINE_STAIRS "dark_prismarine_stairs" SPEC_STAIRS,
    PRISMARINE_SLAB "prismarine_slab" SPEC_SLAB,
    PRISMARINE_BRICK_SLAB "prismarine_brick_slab" SPEC_SLAB,
    DARK_PRISMARINE_SLAB "dark_prismarine_slab" SPEC_SLAB,
    SEA_LANTERN "sea_lantern",
    HAY_BLOCK "hay_block" SPEC_AXIS,

    // CARPET "carpet" SPEC_COLORED, // Merged
    WHITE_CARPET "white_carpet",
    ORANGE_CARPET "orange_carpet",
    MAGENTA_CARPET "magenta_carpet",
    LIGHT_BLUE_CARPET "light_blue_carpet",
    YELLOW_CARPET "yellow_carpet",
    LIME_CARPET "lime_carpet",
    PINK_CARPET "pink_carpet",
    GRAY_CARPET "gray_carpet",
    LIGHT_GRAY_CARPET "light_gray_carpet",
    CYAN_CARPET "cyan_carpet",
    PURPLE_CARPET "purple_carpet",
    BLUE_CARPET "blue_carpet",
    BROWN_CARPET "brown_carpet",
    GREEN_CARPET "green_carpet",
    RED_CARPET "red_carpet",
    BLACK_CARPET "black_carpet",

    COAL_BLOCK "coal_block",
    PACKED_ICE "packed_ice",
    SUNFLOWER "sunflower" SPEC_DOUBLE_PLANT,
    LILAC "lilac" SPEC_DOUBLE_PLANT,
    ROSE_BUSH "rose_bush" SPEC_DOUBLE_PLANT,
    PEONY "peony" SPEC_DOUBLE_PLANT,
    TALL_GRASS "tall_grass" SPEC_DOUBLE_PLANT,
    LARGE_FERN "large_fern" SPEC_DOUBLE_PLANT,

    // BANNER "banner" SPEC_BANNER, // Merged
    WHITE_BANNER "white_banner" SPEC_BANNER,
    ORANGE_BANNER "orange_banner" SPEC_BANNER,
    MAGENTA_BANNER "magenta_banner" SPEC_BANNER,
    LIGHT_BLUE_BANNER "light_blue_banner" SPEC_BANNER,
    YELLOW_BANNER "yellow_banner" SPEC_BANNER,
    LIME_BANNER "lime_banner" SPEC_BANNER,
    PINK_BANNER "pink_banner" SPEC_BANNER,
    GRAY_BANNER "gray_banner" SPEC_BANNER,
    LIGHT_GRAY_BANNER "light_gray_banner" SPEC_BANNER,
    CYAN_BANNER "cyan_banner" SPEC_BANNER,
    PURPLE_BANNER "purple_banner" SPEC_BANNER,
    BLUE_BANNER "blue_banner" SPEC_BANNER,
    BROWN_BANNER "brown_banner" SPEC_BANNER,
    GREEN_BANNER "green_banner" SPEC_BANNER,
    RED_BANNER "red_banner" SPEC_BANNER,
    BLACK_BANNER "black_banner" SPEC_BANNER,

    // WALL_BANNER "wall_banner" SPEC_WALL_BANNER, // Merged
    WHITE_WALL_BANNER "white_wall_banner" SPEC_WALL_BANNER,
    ORANGE_WALL_BANNER "orange_wall_banner" SPEC_WALL_BANNER,
    MAGENTA_WALL_BANNER "magenta_wall_banner" SPEC_WALL_BANNER,
    LIGHT_BLUE_WALL_BANNER "light_blue_wall_banner" SPEC_WALL_BANNER,
    YELLOW_WALL_BANNER "yellow_wall_banner" SPEC_WALL_BANNER,
    LIME_WALL_BANNER "lime_wall_banner" SPEC_WALL_BANNER,
    PINK_WALL_BANNER "pink_wall_banner" SPEC_WALL_BANNER,
    GRAY_WALL_BANNER "gray_wall_banner" SPEC_WALL_BANNER,
    LIGHT_GRAY_WALL_BANNER "light_gray_wall_banner" SPEC_WALL_BANNER,
    CYAN_WALL_BANNER "cyan_wall_banner" SPEC_WALL_BANNER,
    PURPLE_WALL_BANNER "purple_wall_banner" SPEC_WALL_BANNER,
    BLUE_WALL_BANNER "blue_wall_banner" SPEC_WALL_BANNER,
    BROWN_WALL_BANNER "brown_wall_banner" SPEC_WALL_BANNER,
    GREEN_WALL_BANNER "green_wall_banner" SPEC_WALL_BANNER,
    RED_WALL_BANNER "red_wall_banner" SPEC_WALL_BANNER,
    BLACK_WALL_BANNER "black_wall_banner" SPEC_WALL_BANNER,

    RED_SANDSTONE "red_sandstone",
    CHISELED_RED_SANDSTONE "chiseled_red_sandstone",
    CUT_RED_SANDSTONE "cut_red_sandstone",
    RED_SANDSTONE_STAIRS "red_sandstone_stairs" SPEC_STAIRS,

    OAK_SLAB "oak_slab" SPEC_SLAB,
    SPRUCE_SLAB "spruce_slab" SPEC_SLAB,
    BIRCH_SLAB "birch_slab" SPEC_SLAB,
    JUNGLE_SLAB "jungle_slab" SPEC_SLAB,
    ACACIA_SLAB "acacia_slab" SPEC_SLAB,
    DARK_OAK_SLAB "dark_oak_slab" SPEC_SLAB,

    STONE_SLAB "stone_slab" SPEC_SLAB,
    SMOOTH_STONE_SLAB "smooth_stone_slab" SPEC_SLAB,
    SANDSTONE_SLAB "sandstone_slab" SPEC_SLAB,
    CUT_SANDSTONE_SLAB "cut_sandstone_slab" SPEC_SLAB,
    PETRIFIED_OAK_SLAB "petrified_oak_slab" SPEC_SLAB,
    COBBLESTONE_SLAB "cobblestone_slab" SPEC_SLAB,
    BRICK_SLAB "brick_slab" SPEC_SLAB,
    STONE_BRICK_SLAB "stone_brick_slab" SPEC_SLAB,
    NETHER_BRICK_SLAB "nether_brick_slab" SPEC_SLAB,
    QUARTZ_SLAB "quartz_slab" SPEC_SLAB,
    RED_SANDSTONE_SLAB "red_sandstone_slab" SPEC_SLAB,
    CUT_RED_SANDSTONE_SLAB "cut_red_sandstone_slab" SPEC_SLAB,
    PURPUR_SLAB "purpur_slab" SPEC_SLAB,
    SMOOTH_STONE "smooth_stone",
    SMOOTH_SANDSTONE "smooth_sandstone",
    SMOOTH_QUARTZ "smooth_quartz",
    SMOOTH_RED_SANDSTONE "smooth_red_sandstone",

    OAK_FENCE_GATE "oak_fence_gate" SPEC_FENCE_GATE,
    SPRUCE_FENCE_GATE "spruce_fence_gate" SPEC_FENCE_GATE,
    BIRCH_FENCE_GATE "birch_fence_gate" SPEC_FENCE_GATE,
    JUNGLE_FENCE_GATE "jungle_fence_gate" SPEC_FENCE_GATE,
    ACACIA_FENCE_GATE "acacia_fence_gate" SPEC_FENCE_GATE,
    DARK_OAK_FENCE_GATE "dark_oak_fence_gate" SPEC_FENCE_GATE,

    OAK_FENCE "oak_fence" SPEC_BARS,
    SPRUCE_FENCE "spruce_fence" SPEC_BARS,
    BIRCH_FENCE "birch_fence" SPEC_BARS,
    JUNGLE_FENCE "jungle_fence" SPEC_BARS,
    ACACIA_FENCE "acacia_fence" SPEC_BARS,
    DARK_OAK_FENCE "dark_oak_fence" SPEC_BARS,

    OAK_DOOR "oak_door" SPEC_DOOR,
    SPRUCE_DOOR "spruce_door" SPEC_DOOR,
    BIRCH_DOOR "birch_door" SPEC_DOOR,
    JUNGLE_DOOR "jungle_door" SPEC_DOOR,
    ACACIA_DOOR "acacia_door" SPEC_DOOR,
    DARK_OAK_DOOR "dark_oak_door" SPEC_DOOR,

    END_ROD "end_rod" SPEC_FACING,
    CHORUS_PLANT "chorus_plant" SPEC_MULTIFACE,
    CHORUS_FLOWER "chorus_flower" SPEC_CHORUS_FLOWER,
    PURPUR_BLOCK "purpur_block",
    PURPUR_PILLAR "purpur_pillar" SPEC_AXIS,
    PURPUR_STAIRS "purpur_stairs" SPEC_STAIRS,
    END_STONE_BRICKS "end_stone_bricks",
    BEETROOTS "beetroots" SPEC_BEETROOTS,
    DIRT_PATH "dirt_path",
    END_GATEWAY "end_gateway",
    FROSTED_ICE "frosted_ice" SPEC_FROSTED_ICE,
    MAGMA_BLOCK "magma_block",
    NETHER_WART_BLOCK "nether_wart_block",
    RED_NETHER_BRICKS "red_nether_bricks",
    BONE_BLOCK "bone_block" SPEC_AXIS,
    STRUCTURE_VOID "structure_void",
    OBSERVER "observer" SPEC_OBSERVER,

    SHULKER_BOX "shulker_box" SPEC_SHULKER_BOX,
    // COLORED_SHULKER_BOX "colored_shulker_box" SPEC_COLORED_SHULKER_BOX, // Merged
    WHITE_SHULKER_BOX "white_shulker_box" SPEC_SHULKER_BOX,
    ORANGE_SHULKER_BOX "orange_shulker_box" SPEC_SHULKER_BOX,
    MAGENTA_SHULKER_BOX "magenta_shulker_box" SPEC_SHULKER_BOX,
    LIGHT_BLUE_SHULKER_BOX "light_blue_shulker_box" SPEC_SHULKER_BOX,
    YELLOW_SHULKER_BOX "yellow_shulker_box" SPEC_SHULKER_BOX,
    LIME_SHULKER_BOX "lime_shulker_box" SPEC_SHULKER_BOX,
    PINK_SHULKER_BOX "pink_shulker_box" SPEC_SHULKER_BOX,
    GRAY_SHULKER_BOX "gray_shulker_box" SPEC_SHULKER_BOX,
    LIGHT_GRAY_SHULKER_BOX "light_gray_shulker_box" SPEC_SHULKER_BOX,
    CYAN_SHULKER_BOX "cyan_shulker_box" SPEC_SHULKER_BOX,
    PURPLE_SHULKER_BOX "purple_shulker_box" SPEC_SHULKER_BOX,
    BLUE_SHULKER_BOX "blue_shulker_box" SPEC_SHULKER_BOX,
    BROWN_SHULKER_BOX "brown_shulker_box" SPEC_SHULKER_BOX,
    GREEN_SHULKER_BOX "green_shulker_box" SPEC_SHULKER_BOX,
    RED_SHULKER_BOX "red_shulker_box" SPEC_SHULKER_BOX,
    BLACK_SHULKER_BOX "black_shulker_box" SPEC_SHULKER_BOX,

    // GLAZED_TERRACOTTA "glazed_terracotta" SPEC_GLAZED_TERRACOTA,
    WHITE_GLAZED_TERRACOTTA "white_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    ORANGE_GLAZED_TERRACOTTA "orange_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    MAGENTA_GLAZED_TERRACOTTA "magenta_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    LIGHT_BLUE_GLAZED_TERRACOTTA "light_blue_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    YELLOW_GLAZED_TERRACOTTA "yellow_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    LIME_GLAZED_TERRACOTTA "lime_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    PINK_GLAZED_TERRACOTTA "pink_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    GRAY_GLAZED_TERRACOTTA "gray_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    LIGHT_GRAY_GLAZED_TERRACOTTA "light_gray_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    CYAN_GLAZED_TERRACOTTA "cyan_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    PURPLE_GLAZED_TERRACOTTA "purple_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    BLUE_GLAZED_TERRACOTTA "blue_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    BROWN_GLAZED_TERRACOTTA "brown_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    GREEN_GLAZED_TERRACOTTA "green_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    RED_GLAZED_TERRACOTTA "red_glazed_terracotta" SPEC_HORIZONTAL_FACING,
    BLACK_GLAZED_TERRACOTTA "black_glazed_terracotta" SPEC_HORIZONTAL_FACING,

    // CONCRETE "concrete" SPEC_COLORED,
    WHITE_CONCRETE "white_concrete",
    ORANGE_CONCRETE "orange_concrete",
    MAGENTA_CONCRETE "magenta_concrete",
    LIGHT_BLUE_CONCRETE "light_blue_concrete",
    YELLOW_CONCRETE "yellow_concrete",
    LIME_CONCRETE "lime_concrete",
    PINK_CONCRETE "pink_concrete",
    GRAY_CONCRETE "gray_concrete",
    LIGHT_GRAY_CONCRETE "light_gray_concrete",
    CYAN_CONCRETE "cyan_concrete",
    PURPLE_CONCRETE "purple_concrete",
    BLUE_CONCRETE "blue_concrete",
    BROWN_CONCRETE "brown_concrete",
    GREEN_CONCRETE "green_concrete",
    RED_CONCRETE "red_concrete",
    BLACK_CONCRETE "black_concrete",

    // CONCRETE_POWDER "concrete_powder" SPEC_COLORED,
    WHITE_CONCRETE_POWDER "white_concrete_powder",
    ORANGE_CONCRETE_POWDER "orange_concrete_powder",
    MAGENTA_CONCRETE_POWDER "magenta_concrete_powder",
    LIGHT_BLUE_CONCRETE_POWDER "light_blue_concrete_powder",
    YELLOW_CONCRETE_POWDER "yellow_concrete_powder",
    LIME_CONCRETE_POWDER "lime_concrete_powder",
    PINK_CONCRETE_POWDER "pink_concrete_powder",
    GRAY_CONCRETE_POWDER "gray_concrete_powder",
    LIGHT_GRAY_CONCRETE_POWDER "light_gray_concrete_powder",
    CYAN_CONCRETE_POWDER "cyan_concrete_powder",
    PURPLE_CONCRETE_POWDER "purple_concrete_powder",
    BLUE_CONCRETE_POWDER "blue_concrete_powder",
    BROWN_CONCRETE_POWDER "brown_concrete_powder",
    GREEN_CONCRETE_POWDER "green_concrete_powder",
    RED_CONCRETE_POWDER "red_concrete_powder",
    BLACK_CONCRETE_POWDER "black_concrete_powder",

    KELP "kelp" SPEC_KELP,
    KELP_PLANT "kelp_plant",
    DRIED_KELP_BLOCK "dried_kelp_block",
    TURTLE_EGG "turtle_egg" SPEC_TURTLE_EGG,

    // CORAL_BLOCK "coral_block" SPEC_CORAL_BLOCK, // Merged
    TUBE_CORAL_BLOCK "tube_coral_block",
    BRAIN_CORAL_BLOCK "brain_coral_block",
    BUBBLE_CORAL_BLOCK "bubble_coral_block",
    FIRE_CORAL_BLOCK "fire_coral_block",
    HORN_CORAL_BLOCK "horn_coral_block",

    // DEAD_CORAL_BLOCK "dead_coral_block" SPEC_CORAL_BLOCK, // Merged
    DEAD_TUBE_CORAL_BLOCK "dead_tube_coral_block",
    DEAD_BRAIN_CORAL_BLOCK "dead_brain_coral_block",
    DEAD_BUBBLE_CORAL_BLOCK "dead_bubble_coral_block",
    DEAD_FIRE_CORAL_BLOCK "dead_fire_coral_block",
    DEAD_HORN_CORAL_BLOCK "dead_horn_coral_block",

    // CORAL "coral" SPEC_CORAL, // Merged
    TUBE_CORAL "tube_coral" SPEC_WATERLOGGED,
    BRAIN_CORAL "brain_coral" SPEC_WATERLOGGED,
    BUBBLE_CORAL "bubble_coral" SPEC_WATERLOGGED,
    FIRE_CORAL "fire_coral" SPEC_WATERLOGGED,
    HORN_CORAL "horn_coral" SPEC_WATERLOGGED,

    // DEAD_CORAL "dead_coral" SPEC_CORAL, // Merged
    DEAD_TUBE_CORAL "dead_tube_coral" SPEC_WATERLOGGED,
    DEAD_BRAIN_CORAL "dead_brain_coral" SPEC_WATERLOGGED,
    DEAD_BUBBLE_CORAL "dead_bubble_coral" SPEC_WATERLOGGED,
    DEAD_FIRE_CORAL "dead_fire_coral" SPEC_WATERLOGGED,
    DEAD_HORN_CORAL "dead_horn_coral" SPEC_WATERLOGGED,

    // CORAL_FAN "coral_fan" SPEC_CORAL, // Merged
    TUBE_CORAL_FAN "tube_coral_fan" SPEC_WATERLOGGED,
    BRAIN_CORAL_FAN "brain_coral_fan" SPEC_WATERLOGGED,
    BUBBLE_CORAL_FAN "bubble_coral_fan" SPEC_WATERLOGGED,
    FIRE_CORAL_FAN "fire_coral_fan" SPEC_WATERLOGGED,
    HORN_CORAL_FAN "horn_coral_fan" SPEC_WATERLOGGED,

    // DEAD_CORAL_FAN "dead_coral_fan" SPEC_CORAL, // Merged
    DEAD_TUBE_CORAL_FAN "dead_tube_coral_fan" SPEC_WATERLOGGED,
    DEAD_BRAIN_CORAL_FAN "dead_brain_coral_fan" SPEC_WATERLOGGED,
    DEAD_BUBBLE_CORAL_FAN "dead_bubble_coral_fan" SPEC_WATERLOGGED,
    DEAD_FIRE_CORAL_FAN "dead_fire_coral_fan" SPEC_WATERLOGGED,
    DEAD_HORN_CORAL_FAN "dead_horn_coral_fan" SPEC_WATERLOGGED,

    // CORAL_WALL_FAN "coral_wall_fan" SPEC_CORAL_WALL, // Merged
    TUBE_CORAL_WALL_FAN "tube_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    BRAIN_CORAL_WALL_FAN "brain_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    BUBBLE_CORAL_WALL_FAN "bubble_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    FIRE_CORAL_WALL_FAN "fire_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    HORN_CORAL_WALL_FAN "horn_coral_wall_fan" SPEC_FACING_WATERLOGGED,

    // DEAD_CORAL_WALL_FAN "dead_coral_wall_fan" SPEC_CORAL_WALL, // Merged
    DEAD_TUBE_CORAL_WALL_FAN "dead_tube_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    DEAD_BRAIN_CORAL_WALL_FAN "dead_brain_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    DEAD_BUBBLE_CORAL_WALL_FAN "dead_bubble_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    DEAD_FIRE_CORAL_WALL_FAN "dead_fire_coral_wall_fan" SPEC_FACING_WATERLOGGED,
    DEAD_HORN_CORAL_WALL_FAN "dead_horn_coral_wall_fan" SPEC_FACING_WATERLOGGED,

    SEA_PICKLE "sea_pickle" SPEC_SEA_PICKLE,
    BLUE_ICE "blue_ice",
    CONDUIT "conduit" SPEC_WATERLOGGED,

    BAMBOO_SAPLING "bamboo_sapling",
    BAMBOO "bamboo" SPEC_BAMBOO,
    POTTED_BAMBOO "potted_bamboo",

    VOID_AIR "void_air",
    CAVE_AIR "cave_air",

    BUBBLE_COLUMN "bubble_column" SPEC_BUBBLE_COLUMN,
    POLISHED_GRANITE_STAIRS "polished_granite_stairs" SPEC_STAIRS,
    SMOOTH_RED_SANDSTONE_STAIRS "smooth_red_sandstone_stairs" SPEC_STAIRS,
    MOSSY_STONE_BRICK_STAIRS "mossy_stone_brick_stairs" SPEC_STAIRS,
    POLISHED_DIORITE_STAIRS "polished_diorite_stairs" SPEC_STAIRS,
    MOSSY_COBBLESTONE_STAIRS "mossy_cobblestone_stairs" SPEC_STAIRS,
    END_STONE_BRICK_STAIRS "end_stone_brick_stairs" SPEC_STAIRS,
    STONE_STAIRS "stone_stairs" SPEC_STAIRS,
    SMOOTH_SANDSTONE_STAIRS "smooth_sandstone_stairs" SPEC_STAIRS,
    SMOOTH_QUARTZ_STAIRS "smooth_quartz_stairs" SPEC_STAIRS,
    GRANITE_STAIRS "granite_stairs" SPEC_STAIRS,
    ANDESITE_STAIRS "andesite_stairs" SPEC_STAIRS,
    RED_NETHER_BRICK_STAIRS "red_nether_brick_stairs" SPEC_STAIRS,
    POLISHED_ANDESITE_STAIRS "polished_andesite_stairs" SPEC_STAIRS,
    DIORITE_STAIRS "diorite_stairs" SPEC_STAIRS,
    POLISHED_GRANITE_SLAB "polished_granite_slab" SPEC_SLAB,
    SMOOTH_RED_SANDSTONE_SLAB "smooth_red_sandstone_slab" SPEC_SLAB,
    MOSSY_STONE_BRICK_SLAB "mossy_stone_brick_slab" SPEC_SLAB,
    POLISHED_DIORITE_SLAB "polished_diorite_slab" SPEC_SLAB,
    MOSSY_COBBLESTONE_SLAB "mossy_cobblestone_slab" SPEC_SLAB,
    END_STONE_BRICK_SLAB "end_stone_brick_slab" SPEC_SLAB,
    SMOOTH_SANDSTONE_SLAB "smooth_sandstone_slab" SPEC_SLAB,
    SMOOTH_QUARTZ_SLAB "smooth_quartz_slab" SPEC_SLAB,
    GRANITE_SLAB "granite_slab" SPEC_SLAB,
    ANDESITE_SLAB "andesite_slab" SPEC_SLAB,
    RED_NETHER_BRICK_SLAB "red_nether_brick_slab" SPEC_SLAB,
    POLISHED_ANDESITE_SLAB "polished_andesite_slab" SPEC_SLAB,
    DIORITE_SLAB "diorite_slab" SPEC_SLAB,
    BRICK_WALL "brick_wall" SPEC_WALL,
    PRISMARINE_WALL "prismarine_wall" SPEC_WALL,
    RED_SANDSTONE_WALL "red_sandstone_wall" SPEC_WALL,
    MOSSY_STONE_BRICK_WALL "mossy_stone_brick_wall" SPEC_WALL,
    GRANITE_WALL "granite_wall" SPEC_WALL,
    STONE_BRICK_WALL "stone_brick_wall" SPEC_WALL,
    NETHER_BRICK_WALL "nether_brick_wall" SPEC_WALL,
    ANDESITE_WALL "andesite_wall" SPEC_WALL,
    RED_NETHER_BRICK_WALL "red_nether_brick_wall" SPEC_WALL,
    SANDSTONE_WALL "sandstone_wall" SPEC_WALL,
    END_STONE_BRICK_WALL "end_stone_brick_wall" SPEC_WALL,
    DIORITE_WALL "diorite_wall" SPEC_WALL,
    SCAFFOLDING "scaffolding" SPEC_SCAFFOLDING,
    LOOM "loom" SPEC_HORIZONTAL_FACING,
    BARREL "barrel" SPEC_BARREL,
    SMOKER "smoker" SPEC_FURNACE_LIKE,
    BLAST_FURNACE "blast_furnace" SPEC_FURNACE_LIKE,
    CARTOGRAPHY_TABLE "cartography_table",
    FLETCHING_TABLE "fletching_table",
    GRINDSTONE "grindstone" SPEC_GRINDSTONE,
    LECTERN "lectern" SPEC_LECTERN,
    SMITHING_TABLE "smithing_table",
    STONECUTTER "stonecutter" SPEC_HORIZONTAL_FACING,
    BELL "bell" SPEC_BELL,
    LANTERN "lantern" SPEC_LANTERN,
    SOUL_LANTERN "soul_lantern" SPEC_LANTERN,
    CAMPFIRE "campfire" SPEC_CAMPFIRE,
    SOUL_CAMPFIRE "soul_campfire" SPEC_CAMPFIRE,
    SWEET_BERRY_BUSH "sweet_berry_bush" SPEC_SWEET_BERRY_BUSH,

    WARPED_STEM "warped_stem" SPEC_AXIS,
    STRIPPED_WARPED_STEM "stripped_warped_stem" SPEC_AXIS,
    WARPED_HYPHAE "warped_hyphae" SPEC_AXIS,
    STRIPPED_WARPED_HYPHAE "stripped_warped_hyphae" SPEC_AXIS,
    WARPED_NYLIUM "warped_nylium",
    WARPED_FUNGUS "warped_fungus",
    WARPED_WART_BLOCK "warped_wart_block",
    WARPED_ROOTS "warped_roots",

    CRIMSON_STEM "crimson_stem" SPEC_AXIS,
    STRIPPED_CRIMSON_STEM "stripped_crimson_stem" SPEC_AXIS,
    CRIMSON_HYPHAE "crimson_hyphae" SPEC_AXIS,
    STRIPPED_CRIMSON_HYPHAE "stripped_crimson_hyphae" SPEC_AXIS,
    CRIMSON_NYLIUM "crimson_nylium",
    CRIMSON_FUNGUS "crimson_fungus",
    CRIMSON_ROOTS "crimson_roots",

    NETHER_SPROUTS "nether_sprouts",
    SHROOMLIGHT "shroomlight",
    WEEPING_VINES "weeping_vines" SPEC_NETHER_VINE,
    WEEPING_VINES_PLANT "weeping_vines_plant",
    TWISTING_VINES "twisting_vines" SPEC_NETHER_VINE,
    TWISTING_VINES_PLANT "twisting_vines_plant",

    CRIMSON_PLANKS "crimson_planks",
    WARPED_PLANKS "warped_planks",
    CRIMSON_SLAB "crimson_slab" SPEC_SLAB,
    WARPED_SLAB "warped_slab" SPEC_SLAB,
    CRIMSON_PRESSURE_PLATE "crimson_pressure_plate" SPEC_PRESSURE_PLATE,
    WARPED_PRESSURE_PLATE "warped_pressure_plate" SPEC_PRESSURE_PLATE,
    CRIMSON_FENCE "crimson_fence" SPEC_BARS,
    WARPED_FENCE "warped_fence" SPEC_BARS,
    CRIMSON_TRAPDOOR "crimson_trapdoor" SPEC_TRAPDOOR,
    WARPED_TRAPDOOR "warped_trapdoor" SPEC_TRAPDOOR,
    CRIMSON_FENCE_GATE "crimson_fence_gate" SPEC_FENCE_GATE,
    WARPED_FENCE_GATE "warped_fence_gate" SPEC_FENCE_GATE,
    CRIMSON_STAIRS "crimson_stairs" SPEC_STAIRS,
    WARPED_STAIRS "warped_stairs" SPEC_STAIRS,
    CRIMSON_BUTTON "crimson_button" SPEC_BUTTON,
    WARPED_BUTTON "warped_button" SPEC_BUTTON,
    CRIMSON_DOOR "crimson_door" SPEC_DOOR,
    WARPED_DOOR "warped_door" SPEC_DOOR,
    CRIMSON_SIGN "crimson_sign" SPEC_SIGN,
    WARPED_SIGN "warped_sign" SPEC_SIGN,
    CRIMSON_WALL_SIGN "crimson_wall_sign" SPEC_WALL_SIGN,
    WARPED_WALL_SIGN "warped_wall_sign" SPEC_WALL_SIGN,

    STRUCTURE_BLOCK "structure_block" SPEC_STRUCTURE_BLOCK,
    JIGSAW "jigsaw" SPEC_JIGSAW,
    COMPOSTER "composter" SPEC_COMPOSTER,
    TARGET "target" SPEC_REDSTONE_POWER,
    BEE_NEST "bee_nest" SPEC_BEEHIVE,
    BEEHIVE "beehive" SPEC_BEEHIVE,
    HONEY_BLOCK "honey_block",
    HONEYCOMB_BLOCK "honeycomb_block",
    NETHERITE_BLOCK "netherite_block",
    ANCIENT_DEBRIS "ancient_debris",
    CRYING_OBSIDIAN "crying_obsidian",
    RESPAWN_ANCHOR "respawn_anchor" SPEC_RESPAWN_ANCHOR,

    POTTED_CRIMSON_FUNGUS "potted_crimson_fungus",
    POTTED_WARPED_FUNGUS "potted_warped_fungus",
    POTTED_CRIMSON_ROOTS "potted_crimson_roots",
    POTTED_WARPED_ROOTS "potted_warped_roots",

    LODESTONE "lodestone",
    BLACKSTONE "blackstone",
    BLACKSTONE_STAIRS "blackstone_stairs" SPEC_STAIRS,
    BLACKSTONE_WALL "blackstone_wall" SPEC_WALL,
    BLACKSTONE_SLAB "blackstone_slab" SPEC_SLAB,
    POLISHED_BLACKSTONE "polished_blackstone",
    POLISHED_BLACKSTONE_BRICKS "polished_blackstone_bricks",
    CRACKED_POLISHED_BLACKSTONE_BRICKS "cracked_polished_blackstone_bricks",
    CHISELED_POLISHED_BLACKSTONE "chiseled_polished_blackstone",
    POLISHED_BLACKSTONE_BRICK_SLAB "polished_blackstone_brick_slab" SPEC_SLAB,
    POLISHED_BLACKSTONE_BRICK_STAIRS "polished_blackstone_brick_stairs" SPEC_STAIRS,
    POLISHED_BLACKSTONE_BRICK_WALL "polished_blackstone_brick_wall" SPEC_WALL,
    GILDED_BLACKSTONE "gilded_blackstone" SPEC_WALL,
    POLISHED_BLACKSTONE_STAIRS "polished_blackstone_stairs" SPEC_STAIRS,
    POLISHED_BLACKSTONE_SLAB "polished_blackstone_slab" SPEC_SLAB,
    POLISHED_BLACKSTONE_PRESSURE_PLATE "polished_blackstone_pressure_plate" SPEC_PRESSURE_PLATE,
    POLISHED_BLACKSTONE_BUTTON "polished_blackstone_button" SPEC_BUTTON,
    POLISHED_BLACKSTONE_WALL "polished_blackstone_wall" SPEC_WALL,
    CHISELED_NETHER_BRICKS "chiseled_nether_bricks",
    CRACKED_NETHER_BRICKS "cracked_nether_bricks",
    QUARTZ_BRICKS "quartz_bricks",

    CANDLE "candle" SPEC_CANDLE,
    // COLORED_CANDLE "candle" SPEC_COLORED_CANDLE,  // Merged
    WHITE_CANDLE "white_candle" SPEC_CANDLE,
    ORANGE_CANDLE "orange_candle" SPEC_CANDLE,
    MAGENTA_CANDLE "magenta_candle" SPEC_CANDLE,
    LIGHT_BLUE_CANDLE "light_blue_candle" SPEC_CANDLE,
    YELLOW_CANDLE "yellow_candle" SPEC_CANDLE,
    LIME_CANDLE "lime_candle" SPEC_CANDLE,
    PINK_CANDLE "pink_candle" SPEC_CANDLE,
    GRAY_CANDLE "gray_candle" SPEC_CANDLE,
    LIGHT_GRAY_CANDLE "light_gray_candle" SPEC_CANDLE,
    CYAN_CANDLE "cyan_candle" SPEC_CANDLE,
    PURPLE_CANDLE "purple_candle" SPEC_CANDLE,
    BLUE_CANDLE "blue_candle" SPEC_CANDLE,
    BROWN_CANDLE "brown_candle" SPEC_CANDLE,
    GREEN_CANDLE "green_candle" SPEC_CANDLE,
    RED_CANDLE "red_candle" SPEC_CANDLE,
    BLACK_CANDLE "black_candle" SPEC_CANDLE,

    CANDLE_CAKE "candle_cake" SPEC_LIT,
    // COLORED_CANDLE_CAKE "candle_cake" SPEC_COLORED_LIT,  // Merged
    WHITE_CANDLE_CAKE "white_candle_cake" SPEC_LIT,
    ORANGE_CANDLE_CAKE "orange_candle_cake" SPEC_LIT,
    MAGENTA_CANDLE_CAKE "magenta_candle_cake" SPEC_LIT,
    LIGHT_BLUE_CANDLE_CAKE "light_blue_candle_cake" SPEC_LIT,
    YELLOW_CANDLE_CAKE "yellow_candle_cake" SPEC_LIT,
    LIME_CANDLE_CAKE "lime_candle_cake" SPEC_LIT,
    PINK_CANDLE_CAKE "pink_candle_cake" SPEC_LIT,
    GRAY_CANDLE_CAKE "gray_candle_cake" SPEC_LIT,
    LIGHT_GRAY_CANDLE_CAKE "light_gray_candle_cake" SPEC_LIT,
    CYAN_CANDLE_CAKE "cyan_candle_cake" SPEC_LIT,
    PURPLE_CANDLE_CAKE "purple_candle_cake" SPEC_LIT,
    BLUE_CANDLE_CAKE "blue_candle_cake" SPEC_LIT,
    BROWN_CANDLE_CAKE "brown_candle_cake" SPEC_LIT,
    GREEN_CANDLE_CAKE "green_candle_cake" SPEC_LIT,
    RED_CANDLE_CAKE "red_candle_cake" SPEC_LIT,
    BLACK_CANDLE_CAKE "black_candle_cake" SPEC_LIT,

    AMETHYST_BLOCK "amethyst_block",
    BUDDING_AMETHYST "budding_amethyst",
    AMETHYST_CLUSTER "amethyst_cluster" SPEC_FACING_WATERLOGGED,
    LARGE_AMETHYST_BUD "large_amethyst_bud" SPEC_FACING_WATERLOGGED,
    MEDIUM_AMETHYST_BUD "medium_amethyst_bud" SPEC_FACING_WATERLOGGED,
    SMALL_AMETHYST_BUD "small_amethyst_bud" SPEC_FACING_WATERLOGGED,

    TUFF "tuff",
    CALCITE "calcite",
    TINTED_GLASS "tinted_glass",
    POWDER_SNOW "powder_snow",
    SCULK_SENSOR "sculk_sensor" SPEC_SCULK_SENSOR,

    COPPER_ORE "copper_ore",
    DEEPSLATE_COPPER_ORE "deepslate_copper_ore",

    COPPER_BLOCK "copper_block", // SPEC_COPPER_BLOCK,  // Merged
    EXPOSED_COPPER_BLOCK "exposed_copper",
    WEATHERED_COPPER_BLOCK "weathered_copper",
    OXIDIZED_COPPER_BLOCK "oxidized_copper",

    CUT_COPPER "cut_copper", // SPEC_COPPER_BLOCK,  // Merged
    EXPOSED_CUT_COPPER "exposed_cut_copper",
    WEATHERED_CUT_COPPER "weathered_cut_copper",
    OXIDIZED_CUT_COPPER "oxidized_cut_copper",

    CUT_COPPER_STAIRS "cut_copper_stairs" SPEC_STAIRS, // SPEC_COPPER_STAIRS,  // Merged
    EXPOSED_CUT_COPPER_STAIRS "exposed_cut_copper_stairs" SPEC_STAIRS,
    WEATHERED_CUT_COPPER_STAIRS "weathered_cut_copper_stairs" SPEC_STAIRS,
    OXIDIZED_CUT_COPPER_STAIRS "oxidized_cut_copper_stairs" SPEC_STAIRS,

    CUT_COPPER_SLAB "cut_copper_slab" SPEC_SLAB, // SPEC_COPPER_SLAB,  // Merged
    EXPOSED_CUT_COPPER_SLAB "exposed_cut_copper_slab" SPEC_SLAB,
    WEATHERED_CUT_COPPER_SLAB "weathered_cut_copper_slab" SPEC_SLAB,
    OXIDIZED_CUT_COPPER_SLAB "oxidized_cut_copper_slab" SPEC_SLAB,

    WAXED_COPPER_BLOCK "waxed_copper_block", // SPEC_COPPER_BLOCK,  // Merged
    WAXED_EXPOSED_COPPER_BLOCK "waxed_exposed_copper",
    WAXED_WEATHERED_COPPER_BLOCK "waxed_weathered_copper",
    WAXED_OXIDIZED_COPPER_BLOCK "waxed_oxidized_copper",

    WAXED_CUT_COPPER "waxed_cut_copper", // SPEC_COPPER_BLOCK,  // Merged
    WAXED_EXPOSED_CUT_COPPER "waxed_exposed_cut_copper",
    WAXED_WEATHERED_CUT_COPPER "waxed_weathered_cut_copper",
    WAXED_OXIDIZED_CUT_COPPER "waxed_oxidized_cut_copper",

    WAXED_CUT_COPPER_STAIRS "waxed_cut_copper_stairs" SPEC_STAIRS, // SPEC_COPPER_STAIRS,  // Merged
    WAXED_EXPOSED_CUT_COPPER_STAIRS "waxed_exposed_cut_copper_stairs" SPEC_STAIRS,
    WAXED_WEATHERED_CUT_COPPER_STAIRS "waxed_weathered_cut_copper_stairs" SPEC_STAIRS,
    WAXED_OXIDIZED_CUT_COPPER_STAIRS "waxed_oxidized_cut_copper_stairs" SPEC_STAIRS,

    WAXED_CUT_COPPER_SLAB "waxed_cut_copper_slab" SPEC_SLAB, // SPEC_COPPER_SLAB,  // Merged
    WAXED_EXPOSED_CUT_COPPER_SLAB "waxed_exposed_cut_copper_slab" SPEC_SLAB,
    WAXED_WEATHERED_CUT_COPPER_SLAB "waxed_weathered_cut_copper_slab" SPEC_SLAB,
    WAXED_OXIDIZED_CUT_COPPER_SLAB "waxed_oxidized_cut_copper_slab" SPEC_SLAB,

    LIGHTNING_ROD "lightning_rod" SPEC_LIGHTNING_ROD,

    POINTED_DRIPSTONE "pointed_dripstone" SPEC_POINTED_DRIPSTONE,
    DRIPSTONE_BLOCK "dripstone_block",

    CAVE_VINES "cave_vines" SPEC_CAVE_VINES,
    CAVE_VINES_PLANT "cave_vines_plant" SPEC_CAVE_VINES_PLANT,
    SPORE_BLOSSOM "spore_blossom",
    AZALEA "azalea",
    FLOWERING_AZALEA "flowering_azalea",
    MOSS_CARPET "moss_carpet",
    MOSS_BLOCK "moss_block",
    BIG_DRIPLEAF "big_dripleaf" SPEC_BIG_DRIPLEAF,
    BIG_DRIPLEAF_STEM "big_dripleaf_stem" SPEC_HORIZONTAL_FACING_WATERLOGGED,
    SMALL_DRIPLEAF "small_dripleaf" SPEC_SMALL_DRIPLEAF,
    HANGING_ROOTS "hanging_roots" SPEC_WATERLOGGED,
    ROOTED_DIRT "rooted_dirt",

    DEEPSLATE "deepslate" SPEC_AXIS,
    COBBLED_DEEPSLATE "cobbled_deepslate",
    COBBLED_DEEPSLATE_STAIRS "cobbled_deepslate_stairs" SPEC_STAIRS,
    COBBLED_DEEPSLATE_SLAB "cobbled_deepslate_slab" SPEC_SLAB,
    COBBLED_DEEPSLATE_WALL "cobbled_deepslate_wall" SPEC_WALL,
    POLISHED_DEEPSLATE "polished_deepslate",
    POLISHED_DEEPSLATE_STAIRS "polished_deepslate_stairs" SPEC_STAIRS,
    POLISHED_DEEPSLATE_SLAB "polished_deepslate_slab" SPEC_SLAB,
    POLISHED_DEEPSLATE_WALL "polished_deepslate_wall" SPEC_WALL,
    DEEPSLATE_TILES "deepslate_tiles",
    DEEPSLATE_TILE_STAIRS "deepslate_tile_stairs" SPEC_STAIRS,
    DEEPSLATE_TILE_SLAB "deepslate_tile_slab" SPEC_SLAB,
    DEEPSLATE_TILE_WALL "deepslate_tile_wall" SPEC_WALL,
    DEEPSLATE_BRICKS "deepslate_bricks",
    DEEPSLATE_BRICK_STAIRS "deepslate_brick_stairs" SPEC_STAIRS,
    DEEPSLATE_BRICK_SLAB "deepslate_brick_slab" SPEC_SLAB,
    DEEPSLATE_BRICK_WALL "deepslate_brick_wall" SPEC_WALL,
    CHISELED_DEEPSLATE "chiseled_deepslate",
    CRACKED_DEEPSLATE_BRICKS "cracked_deepslate_bricks",
    CRACKED_DEEPSLATE_TILES "cracked_deepslate_tiles",
    INFESTED_DEEPSLATE "infested_deepslate" SPEC_AXIS,
    SMOOTH_BASALT "smooth_basalt",

    RAW_IRON_BLOCK "raw_iron_block",
    RAW_COPPER_BLOCK "raw_copper_block",
    RAW_GOLD_BLOCK "raw_gold_block",

    POTTED_AZALEA "potted_azalea_bush",
    POTTED_FLOWERING_AZALEA "potted_flowering_azalea_bush",

]);


def_enum_serializable!(BambooLeaves {
    None: "none",
    Large: "large",
    Small: "small"
});

def_enum_serializable!(BedPart {
    Foot: "foot",
    Head: "head"
});

def_enum_serializable!(Face {
    Ceiling: "ceiling",
    Floor: "floor",
    Wall: "wall",
    DoubleWall: "double_wall",
    SingleWall: "single_wall"
});

def_enum_serializable!(Instrument {
    Banjo: "banjo",
    BassDrum: "basedrum",
    Bass: "bass",
    Bell: "bell",
    Bit: "bit",
    Chime: "chime",
    CowBell: "cow_bell",
    Didjeridoo: "didgeridoo",
    Flute: "flute",
    Guitar: "guitar",
    Harp: "harp",
    Hat: "hat",
    IronXylophone: "iron_xylophone",
    Pling: "pling",
    Snare: "snare",
    Xylophone: "xylophone"
});

def_enum_serializable!(RailShape {
    EastWest: "east_west",
    NorthEast: "north_east",
    NorthSouth: "north_south",
    NorthWest: "north_west",
    SouthEast: "south_east",
    SouthWest: "south_west",
    AscendingEast: "ascending_east",
    AscendingNorth: "ascending_north",
    AscendingSouth: "ascending_south",
    AscendingWest: "ascending_west"
});

def_enum_serializable!(ComparatorMode {
    Compare: "compare",
    Subtract: "subtract"
});

def_enum_serializable!(RedstoneWireMode {
    None: "none",
    Side: "side",
    Up: "up"
});

def_enum_serializable!(DoubleBlockHalf {
    Lower: "lower",
    Upper: "upper"
});

def_enum_serializable!(Half {
    Top: "top",
    Bottom: "bottom"
});

def_enum_serializable!(DoorHingeSide {
    Left: "left",
    Right: "right"
});

def_enum_serializable!(ChestType {
    Single: "single",
    Left: "left",
    Right: "right"
});

def_enum_serializable!(StairsShape {
    Straight: "straight",
    InnerLeft: "inner_left",
    InnerRight: "inner_right",
    OuterLeft: "outer_left",
    OuterRight: "outer_right"
});

def_enum_serializable!(WallSide {
    None: "none",
    Low: "low",
    Tall: "tall"
});

def_enum_serializable!(SlabType {
    Top: "top",
    Bottom: "bottom",
    Double: "double"
});

def_enum_serializable!(PistonType {
    Normal: "normal",
    Sticky: "sticky"
});

/*def_enum_serializable!(CommandBlockType {
    Impulse: "impulse",
    Repeating: "repeating",
    Chain: "chain"
});*/

def_enum_serializable!(CoralType {
    Tube: "tube",
    Brain: "brain",
    Bubble: "bubble",
    Fire: "fire",
    Horn: "horn"
});

def_enum_serializable!(StructureMode {
    Save: "save",
    Load: "load",
    Corner: "corner",
    Data: "data"
});

def_enum_serializable!(FrontAndTop {
    DownEast: "down_east",
    DownNorth: "down_north",
    DownSouth: "down_south",
    DownWest: "down_west",
    UpEast: "up_east",
    UpNorth: "up_north",
    UpSouth: "up_south",
    UpWest: "up_west",
    WestUp: "west_up",
    EastUp: "east_up",
    NorthUp: "north_up",
    SouthUp: "south_up"
});

def_enum_serializable!(SculkSensorPhase {
    Inactive: "inactive",
    Active: "active",
    Cooldown: "cooldown"
});

/*def_enum_serializable!(OxydationState {
    Unaffected: "unaffected",
    Exposed: "exposed",
    Weathered: "weathered",
    Oxidized: "oxidized"
});*/

def_enum_serializable!(DripstoneThickness {
    TipMerge: "tip_merge",
    Tip: "tip",
    Frustum: "frustum",
    Middle: "middle",
    Base: "base"
});

def_enum_serializable!(DripleafTilt {
    None: "none",
    Unstable: "unstable",
    Partial: "partial",
    Full: "full"
});
