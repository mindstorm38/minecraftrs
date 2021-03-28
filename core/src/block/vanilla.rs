use crate::{blocks, properties, blocks_specs, impl_enum_serializable};
use crate::util::{Direction, Axis, DyeColor};


impl_enum_serializable!(Direction {
    East: "east",
    West: "west",
    South: "south",
    North: "north",
    Up: "up",
    Down: "down"
});


impl_enum_serializable!(Axis {
    X: "x",
    Y: "y",
    Z: "z"
});


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
    pub PROP_HAS_RECORD: bool("has_record");
    pub PROP_HANGING: bool("hanging");
    pub PROP_HAS_BOOK: bool("has_book");
    pub PROP_NOTE: int("note", 25);
    pub PROP_STICKY: bool("sticky");
    pub PROP_EXTENDED: bool("extended");
    pub PROP_SHORT: bool("short");
    pub PROP_LOCKED: bool("locked");
    pub PROP_REPEATER_DELAY: int("delay", 4); // Real is 1 to 4
    pub PROP_CHARGES: int("charges", 5);
    pub PROP_SCAFFOLDING_DISTANCE: int("distance", 8);
    pub PROP_PICKLES: int("pickles", 4);  // Real is 1 to 4
    pub PROP_SNOW_LAYERS: int("layers", 8);  // Real is 1 to 8
    pub PROP_UNSTABLE: bool("unstable");
    pub PROP_ATTACHED: bool("attached");
    pub PROP_DISARMED: bool("disarmed");
    pub PROP_EGGS: int("eggs", 4);  // Real is 1 to 4
    pub PROP_HATCH: int("hatch", 3);
    pub PROP_LIQUID_LEVEL: int("level", 8);
    pub PROP_LIQUID_FALLING: bool("falling");
    pub PROP_IN_WALL: bool("in_wall");
    pub PROP_CONDITIONAL: bool("conditional");
    pub PROP_DRAG: bool("drag");
    pub PROP_PERSISTENT: bool("persistent");
    pub PROP_LEAVES_DISTANCE: int("distance", 7);

    pub PROP_DOWN: bool("down");
    pub PROP_EAST: bool("east");
    pub PROP_NORTH: bool("north");
    pub PROP_SOUTH: bool("south");
    pub PROP_UP: bool("up");
    pub PROP_WEST: bool("west");
    pub PROP_BOTTOM: bool("bottom");

    pub PROP_HORIZONTAL_FACING: enum("facing", Direction, HORIZONTAL_FACING, [
        East, North, South, West
    ]);

    pub PROP_FACING: enum("facing", Direction, FACING, [
        Down, East, North, South, Up, West
    ]);

    pub PROP_AXIS: enum("axis", Axis, AXIS, [X, Y, Z]);
    pub PROP_HORIZONTAL_AXIS: enum("axis", Axis, HORIZONTAL_AXIS, [X, Z]);

    pub PROP_BAMBOO_LEAVES: enum("leaves", BambooLeaves, BAMBOO_LEAVES, [Large, None, Small]);
    pub PROP_BED_PART: enum("part", BedPart, BED_PART, [Foot, Head]);

    pub PROP_BELL_ATTACHMENT: enum("attachment", Face, BELL_ATTACHMENT, [
        Ceiling, DoubleWall, Floor, SingleWall
    ]);

    pub PROP_FACE: enum("face", Face, FACE, [Ceiling, Floor, Wall]);
    pub PROP_DOUBLE_BLOCK_HALF: enum("half", DoubleBlockHalf, DOUBLE_BLOCK_HALF, [Lower, Upper]);
    pub PROP_DOOR_HINGE: enum("hinge", DoorHingeSide, DOOR_HINGE, [Left, Right]);
    pub PROP_HALF: enum("half", Half, HALF, [Top, Bottom]);

    pub PROP_COLOR: enum("color", DyeColor, COLOR, [
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

    pub PROP_INSTRUMENT: enum("instrument", Instrument, INSTRUMENT, [
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

    pub PROP_RAIL_SHAPE: enum("shape", RailShape, RAIL_SHAPE, [
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

    pub PROP_RAIL_SHAPE_SPECIAL: enum("shape", RailShape, RAIL_SHAPE_SPECIAL, [
        EastWest, NorthSouth, AscendingEast, AscendingNorth, AscendingSouth, AscendingWest
    ]);

    pub PROP_COMPARATOR_MODE: enum("mode", ComparatorMode, COMPARATOR_MODE, [
        Compare, Subtract
    ]);

    pub PROP_OVERWORLD_WOOD_TYPE: enum("wood_type", WoodType, OVERWORLD_WOOD_TYPE, [
        Oak, Spruce, Birch, Jungle, Acacia, DarkOak //, Crimson, Warped
    ]);

    pub PROP_NETHER_WOOD_TYPE: enum("wood_type", WoodType, NETHER_WOOD_TYPE, [
        Crimson, Warped
    ]);

    pub PROP_ALL_WOOD_TYPE: enum("wood_type", WoodType, ALL_WOOD_TYPE, [
        Oak, Spruce, Birch, Jungle, Acacia, DarkOak, Crimson, Warped
    ]);

    pub PROP_REDSTONE_EAST: enum("east", RedstoneWireMode, REDSTONE_MODE);
    pub PROP_REDSTONE_NORTH: enum("north", RedstoneWireMode, REDSTONE_MODE);
    pub PROP_REDSTONE_SOUTH: enum("south", RedstoneWireMode, REDSTONE_MODE);
    pub PROP_REDSTONE_WEST: enum("west", RedstoneWireMode, REDSTONE_MODE);

    pub PROP_WALL_EAST: enum("east", WallSide, WALL_SIDE);
    pub PROP_WALL_NORTH: enum("north", WallSide, WALL_SIDE);
    pub PROP_WALL_SOUTH: enum("south", WallSide, WALL_SIDE);
    pub PROP_WALL_WEST: enum("west", WallSide, WALL_SIDE);

    pub PROP_CHEST_TYPE: enum("type", ChestType, CHEST_TYPE, [Single, Left, Right]);
    pub PROP_SLAB_TYPE: enum("type", SlabType, SLAB_TYPE, [Top, Bottom, Double]);

    pub PROP_COMMAND_BLOCK_TYPE: enum("type", CommandBlockType, COMMAND_BLOCK_TYPE, [
        Impulse, Repeating, Chain
    ]);

    pub PROP_STRUCTURE_MODE: enum("mode", StructureMode, STRUCTURE_MODE, [
        Save, Load, Corner, Data
    ]);

    pub PROP_CORAL_TYPE: enum("type", CoralType, CORAL_TYPE, [
        Tube, Brain, Bubble, Fire, Horn
    ]);

    pub PROP_STAIRS_SHAPE: enum("shape", StairsShape, STAIRS_SHAPE, [
        Straight, InnerLeft, InnerRight, OuterLeft, OuterRight
    ]);

    pub PROP_POT_CONTENT: enum("content", PotContent, POT_CONTENT, [
        None,
        OakSapling,
        SpruceSapling,
        BirchSapling,
        JungleSapling,
        AcaciaSapling,
        DarkOakSapling,
        Fern,
        Dandelion,
        Poppy,
        BlueOrchid,
        Allium,
        AzureBluet,
        RedTulip,
        OrangeTulip,
        WhiteTulip,
        PinkTulip,
        OxeyeDaisy,
        Cornflower,
        LilyOfTheValley,
        WitherRose,
        RedMushroom,
        BrownMushroom,
        DeadBush,
        Cactus,
        Bamboo,
        CrimsonFungus,
        WarpedFungus,
        CrimsonRoots,
        WarpedRoots
    ]);

    pub PROP_JIGSAW_ORIENTATION: enum("orientation", FrontAndTop, JIGSAW_ORIENTATION, [
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

}

blocks_specs! {

    pub SPEC_GRASS: [PROP_SNOWY];
    pub SPEC_LEAVES: [PROP_LEAVES_DISTANCE, PROP_PERSISTENT];
    // pub SPEC_PLANKS: [PROP_ALL_WOOD_TYPE];
    pub SPEC_FARMLAND: [PROP_FARMLAND_MOISTURE];
    pub SPEC_SNOW: [PROP_SNOW_LAYERS];
    pub SPEC_MUSHROOM_BLOCK: [PROP_UP, PROP_DOWN, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST];
    pub SPEC_VINE: [PROP_UP, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST];
    pub SPEC_CHORUS_PLANT: [PROP_DOWN, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_UP, PROP_WEST];
    pub SPEC_CORAL: [PROP_CORAL_TYPE];
    pub SPEC_SEA_PICKLE: [PROP_PICKLES, PROP_WATERLOGGED];
    pub SPEC_BAMBOO: [PROP_BAMBOO_AGE, PROP_BAMBOO_LEAVES, PROP_BAMBOO_STAGE];

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

    pub SPEC_STATIC_LIQUID: [PROP_LIQUID_FALLING];
    pub SPEC_FLOWING_LIQUID: [PROP_LIQUID_FALLING, PROP_LIQUID_LEVEL];

    pub SPEC_DISPENSER: [PROP_FACING, PROP_TRIGGERED];
    pub SPEC_DROPPER: [PROP_FACING, PROP_TRIGGERED];
    pub SPEC_OBSERVER: [PROP_FACING, PROP_POWERED];
    pub SPEC_FURNACE_LIKE: [PROP_HORIZONTAL_FACING, PROP_LIT];
    pub SPEC_BARREL: [PROP_FACING, PROP_OPEN];
    pub SPEC_NOTE_BLOCK: [PROP_INSTRUMENT, PROP_NOTE, PROP_POWERED];
    pub SPEC_BED: [PROP_COLOR, PROP_HORIZONTAL_FACING, PROP_BED_PART, PROP_OCCUPIED];
    pub SPEC_BREWING_STAND: [PROP_HAS_BOTTLE_0, PROP_HAS_BOTTLE_1, PROP_HAS_BOTTLE_2];
    pub SPEC_BUTTON: [PROP_HORIZONTAL_FACING, PROP_POWERED, PROP_FACE];
    pub SPEC_WOODEN_BUTTON: [PROP_ALL_WOOD_TYPE, PROP_HORIZONTAL_FACING, PROP_POWERED, PROP_FACE];
    pub SPEC_CHEST: [PROP_HORIZONTAL_FACING, PROP_CHEST_TYPE, PROP_WATERLOGGED];
    pub SPEC_ENDER_CHEST: [PROP_HORIZONTAL_FACING, PROP_WATERLOGGED];
    pub SPEC_REDSTONE_WIRE: [PROP_REDSTONE_POWER, PROP_REDSTONE_EAST, PROP_REDSTONE_NORTH, PROP_REDSTONE_SOUTH, PROP_REDSTONE_WEST];
    pub SPEC_LEVER: [PROP_FACE, PROP_HORIZONTAL_FACING, PROP_POWERED];
    pub SPEC_PRESSURE_PLATE: [PROP_POWERED];
    pub SPEC_WOODEN_PRESSURE_PLATE: [PROP_ALL_WOOD_TYPE, PROP_POWERED];
    pub SPEC_DOOR: [PROP_DOUBLE_BLOCK_HALF, PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_DOOR_HINGE, PROP_POWERED];
    pub SPEC_WOODEN_DOOR: [PROP_ALL_WOOD_TYPE, PROP_DOUBLE_BLOCK_HALF, PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_DOOR_HINGE, PROP_POWERED];
    pub SPEC_WALL_REDSTONE_TORCH: [PROP_HORIZONTAL_FACING, PROP_LIT];
    pub SPEC_JUKEBOX: [PROP_HAS_RECORD];
    pub SPEC_REPEATER: [PROP_REPEATER_DELAY, PROP_HORIZONTAL_FACING, PROP_LOCKED, PROP_POWERED];
    pub SPEC_TRAPDOOR: [PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_HALF, PROP_POWERED, PROP_WATERLOGGED];
    pub SPEC_WOODEN_TRAPDOOR: [PROP_ALL_WOOD_TYPE, PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_HALF, PROP_POWERED, PROP_WATERLOGGED];
    pub SPEC_CAULDRON: [PROP_CAULDRON_LEVEL];
    pub SPEC_TRIPWIRE_HOOK: [PROP_ATTACHED, PROP_HORIZONTAL_FACING, PROP_POWERED];
    pub SPEC_TRIPWIRE: [PROP_ATTACHED, PROP_DISARMED, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_WEST, PROP_POWERED];
    pub SPEC_COMMAND_BLOCK: [PROP_COMMAND_BLOCK_TYPE, PROP_HORIZONTAL_FACING, PROP_CONDITIONAL];
    pub SPEC_COMPARATOR: [PROP_HORIZONTAL_FACING, PROP_COMPARATOR_MODE, PROP_POWERED];
    pub SPEC_DAYLIGHT_DETECTOR: [PROP_INVERTED, PROP_REDSTONE_POWER];
    pub SPEC_HOPPER: [PROP_HORIZONTAL_FACING, PROP_ENABLED];
    pub SPEC_SHULKER_BOX: [PROP_FACING];
    pub SPEC_COLORED_SHULKER_BOX: [PROP_FACING, PROP_COLOR];
    pub SPEC_TURTLE_EGG: [PROP_EGGS, PROP_HATCH];
    pub SPEC_GRINDSTONE: [PROP_FACE, PROP_HORIZONTAL_FACING];
    pub SPEC_LECTERN: [PROP_HORIZONTAL_FACING, PROP_HAS_BOOK, PROP_POWERED];
    pub SPEC_BELL: [PROP_BELL_ATTACHMENT, PROP_HORIZONTAL_FACING, PROP_POWERED];
    pub SPEC_RESPAWN_ANCHOR: [PROP_CHARGES];
    pub SPEC_COMPOSTER: [PROP_COMPOSTER_LEVEL];
    pub SPEC_STRUCTURE_BLOCK: [PROP_STRUCTURE_MODE];
    pub SPEC_JIGSAW: [PROP_JIGSAW_ORIENTATION];

    pub SPEC_RAIL_SPECIAL: [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED];
    pub SPEC_RAIL: [PROP_RAIL_SHAPE];

    pub SPEC_PISTON: [PROP_STICKY, PROP_FACING, PROP_EXTENDED];
    pub SPEC_PISTON_HEAD: [PROP_FACING, PROP_STICKY, PROP_SHORT];
    pub SPEC_MOVING_PISTON: [PROP_FACING, PROP_STICKY];

    pub SPEC_TNT: [PROP_UNSTABLE];
    pub SPEC_WALL_TORCH: [PROP_HORIZONTAL_FACING];
    pub SPEC_FIRE: [PROP_AGE_16, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST, PROP_UP];
    pub SPEC_STAIRS: [PROP_HORIZONTAL_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED];
    pub SPEC_WOODEN_STAIRS: [PROP_ALL_WOOD_TYPE, PROP_HORIZONTAL_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED];
    pub SPEC_SLAB: [PROP_SLAB_TYPE];
    pub SPEC_WOODEN_SLAB: [PROP_ALL_WOOD_TYPE, PROP_SLAB_TYPE];
    pub SPEC_SIGN: [PROP_ALL_WOOD_TYPE, PROP_ROTATION, PROP_WATERLOGGED];
    pub SPEC_WALL_SIGN: [PROP_ALL_WOOD_TYPE, PROP_HORIZONTAL_FACING, PROP_WATERLOGGED];
    pub SPEC_LADDER: [PROP_HORIZONTAL_FACING, PROP_WATERLOGGED];
    pub SPEC_NETHER_PORTAL: [PROP_HORIZONTAL_AXIS];
    pub SPEC_CAKE: [PROP_CAKE_BITES];
    pub SPEC_BARS: [PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED];
    pub SPEC_BARS_COLORED: [PROP_COLOR, PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED];
    pub SPEC_WOODEN_FENCE: [PROP_ALL_WOOD_TYPE, PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED];
    pub SPEC_WALL: [PROP_UP, PROP_WALL_EAST, PROP_WALL_NORTH, PROP_WALL_SOUTH, PROP_WALL_WEST, PROP_WATERLOGGED];
    pub SPEC_FENCE_GATE: [PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_POWERED, PROP_IN_WALL];
    // pub SPEC_WOODEN_FENCE_GATE: [PROP_ALL_WOOD_TYPE, PROP_HORIZONTAL_FACING, PROP_OPEN, PROP_POWERED, PROP_IN_WALL];
    pub SPEC_END_PORTAL: [PROP_END_PORTAL_EYE, PROP_HORIZONTAL_FACING];
    pub SPEC_SKULL: [PROP_ROTATION];
    pub SPEC_WALL_SKULL: [PROP_HORIZONTAL_FACING];
    pub SPEC_BANNER: [PROP_ROTATION, PROP_COLOR];
    pub SPEC_WALL_BANNER: [PROP_HORIZONTAL_FACING, PROP_COLOR];
    pub SPEC_FROSTED_ICE: [PROP_AGE_4];
    pub SPEC_BUBBLE_COLUMN: [PROP_DRAG];
    pub SPEC_SCAFFOLDING: [PROP_BOTTOM, PROP_SCAFFOLDING_DISTANCE, PROP_WATERLOGGED];
    pub SPEC_LANTERN: [PROP_HANGING];
    pub SPEC_CAMPFIRE: [PROP_HORIZONTAL_FACING, PROP_LIT, PROP_SIGNAL_FIRE, PROP_WATERLOGGED];
    pub SPEC_BEEHIVE: [PROP_HORIZONTAL_FACING, PROP_HONEY_LEVEL];
    pub SPEC_GLAZED_TERRACOTA: [PROP_COLOR, PROP_HORIZONTAL_FACING];

    pub SPEC_COLORED: [PROP_COLOR];
    pub SPEC_AXIS: [PROP_AXIS];
    pub SPEC_HORIZONTAL_FACING: [PROP_HORIZONTAL_FACING];
    pub SPEC_FACING: [PROP_FACING];
    pub SPEC_AXIS_WATERLOGGED: [PROP_AXIS, PROP_WATERLOGGED];
    pub SPEC_LIT: [PROP_LIT];
    pub SPEC_WATERLOGGED: [PROP_WATERLOGGED];
    pub SPEC_REDSTONE_POWER: [PROP_REDSTONE_POWER];
    pub SPEC_NETHER_WOOD_TYPE: [PROP_NETHER_WOOD_TYPE];
    pub SPEC_OVERWORLD_WOOD_TYPE : [PROP_OVERWORLD_WOOD_TYPE];

}


// Same order as defined in MC code
// Some block has been merged to avoid defining dozen of wooden variations
// for example, for compatibility with Minecraft these blocks may need
// extensions or a specified module for the conversion.
blocks!(VanillaBlocksStruct VanillaBlocks [
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
    // PLANKS "planks" SPEC_PLANKS, // Merged

    OAK_SAPLING "oak_sapling",
    SPRUCE_SAPLING "spruce_sapling",
    BIRCH_SAPLING "birch_sapling",
    JUNGLE_SAPLING "jungle_sapling",
    ACACIA_SAPLING "acacia_sapling",
    DARK_OAK_SAPLING "dark_oak_sapling",
    // SAPLING "sapling" SPEC_OVERWORLD_WOOD_TYPE, // Merged

    BEDROCK "bedrock",
    WATER "water" SPEC_STATIC_LIQUID,
    FLOWING_WATER "flowing_water" SPEC_FLOWING_LIQUID,
    LAVA "lava" SPEC_STATIC_LIQUID,
    FLOWING_LAVA "flowing_lava" SPEC_FLOWING_LIQUID,
    SAND "sand",
    RED_SAND "red_sand",
    GRAVEL "gravel",
    GOLD_ORE "gold_ore",
    IRON_ORE "iron_ore",
    COAL_ORE "coal_ore",
    NETHER_GOLD_ORE "nether_gold_ore",

    OAK_LOG "oak_log" SPEC_AXIS,
    SPRUCE_LOG "spruce_log" SPEC_AXIS,
    BIRCH_LOG "birch_log" SPEC_AXIS,
    JUNGLE_LOG "jungle_log" SPEC_AXIS,
    ACACIA_LOG "acacia_log" SPEC_AXIS,
    DARK_OAK_LOG "dark_oak_log" SPEC_AXIS,
    // LOG "log" SPEC_OVERWORLD_WOOD_TYPE, // Merged

    STRIPPED_OAK_LOG "stripped_oak_log" SPEC_AXIS,
    STRIPPED_SPRUCE_LOG "stripped_spruce_log" SPEC_AXIS,
    STRIPPED_BIRCH_LOG "stripped_birch_log" SPEC_AXIS,
    STRIPPED_JUNGLE_LOG "stripped_jungle_log" SPEC_AXIS,
    STRIPPED_ACACIA_LOG "stripped_acacia_log" SPEC_AXIS,
    STRIPPED_DARK_OAK_LOG "stripped_dark_oak_log" SPEC_AXIS,
    // STRIPPED_LOG "stripped_log" SPEC_OVERWORLD_WOOD_TYPE, // Merged

    OAK_WOOD "oak_wood" SPEC_AXIS,
    SPRUCE_WOOD "spruce_wood" SPEC_AXIS,
    BIRCH_WOOD "birch_wood" SPEC_AXIS,
    JUNGLE_WOOD "jungle_wood" SPEC_AXIS,
    ACACIA_WOOD "acacia_wood" SPEC_AXIS,
    DARK_OAK_WOOD "dark_oak_wood" SPEC_AXIS,
    // WOOD "wood" SPEC_OVERWORLD_WOOD_TYPE, // Merged

    STRIPPED_OAK_WOOD "stripped_oak_wood" SPEC_AXIS,
    STRIPPED_SPRUCE_WOOD "stripped_spruce_wood" SPEC_AXIS,
    STRIPPED_BIRCH_WOOD "stripped_birch_wood" SPEC_AXIS,
    STRIPPED_JUNGLE_WOOD "stripped_jungle_wood" SPEC_AXIS,
    STRIPPED_ACACIA_WOOD "stripped_acacia_wood" SPEC_AXIS,
    STRIPPED_DARK_OAK_WOOD "stripped_dark_oak_wood" SPEC_AXIS,
    // STRIPPED_WOOD "stripped_wood" SPEC_OVERWORLD_WOOD_TYPE, // Merged

    OAK_LEAVES "oak_leaves" SPEC_LEAVES,
    SPRUCE_LEAVES "spruce_leaves" SPEC_LEAVES,
    BIRCH_LEAVES "birch_leaves" SPEC_LEAVES,
    JUNGLE_LEAVES "jungle_leaves" SPEC_LEAVES,
    ACACIA_LEAVES "acacia_leaves" SPEC_LEAVES,
    DARK_OAK_LEAVES "dark_oak_leaves" SPEC_LEAVES,
    // LEAVES "leaves" SPEC_OVERWORLD_WOOD_TYPE, // Merged

    SPONGE "sponge",
    WET_SPONGE "wet_sponge",
    GLASS "glass",
    LAPIS_ORE "lapis_ore",
    LAPIS_BLOCK "lapis_block",
    DISPENSER "dispenser" SPEC_DISPENSER,
    SANDSTONE "sandstone",
    CHISELED_SANDSTONE "chiseled_sandstone",
    CUT_SANDSTONE "cut_sandstone",
    NOTE_BLOCK "note_block" SPEC_NOTE_BLOCK,
    BED "bed" SPEC_BED,
    POWERED_RAIL "powered_rail" SPEC_RAIL_SPECIAL,
    DETECTOR_RAIL "detector_rail" SPEC_RAIL_SPECIAL,
    PISTON "piston" SPEC_PISTON, // Merged the two piston type into one using a property "sticky".
    PISTON_HEAD "piston_head" SPEC_PISTON_HEAD, // 'sticky' property instead of 'type'
    MOVING_PISTON "moving_piston" SPEC_MOVING_PISTON, // 'sticky' property instead of 'type'
    COBWEB "cobweb",
    GRASS "grass",
    FERN "fern",
    DEAD_BUSH "dead_bush",
    SEAGRASS "seagrass",
    TALL_SEAGRASS "tall_seagrass" SPEC_DOUBLE_PLANT,
    WOOL "wool" SPEC_COLORED,
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
    SOUL_FIRE "fire" SPEC_FIRE,
    SPAWNER "spawner",
    WOODEN_STAIRS "wooden_stairs" SPEC_WOODEN_STAIRS, // Merged
    CHEST "chest" SPEC_CHEST,
    REDSTONE_WIRE "redstone_wire" SPEC_REDSTONE_WIRE,
    DIAMOND_ORE "diamond_ore",
    DIAMOND_BLOCK "diamond_block",
    CRAFTING_TABLE "crafting_table",
    WHEAT "wheat" SPEC_CROP,
    FARMLAND "farmland" SPEC_FARMLAND,
    FURNACE "furnace" SPEC_FURNACE_LIKE,
    SIGN "sign" SPEC_SIGN,
    LADDER "ladder" SPEC_LADDER,
    RAIL "rail" SPEC_RAIL,
    COBBLESTONE_STAIRS "cobblestone_stairs" SPEC_STAIRS,
    WALL_SIGN "wall_sign" SPEC_WALL_SIGN, // Merged
    LEVER "lever" SPEC_LEVER,
    STONE_PRESSURE_PLATE "stone_pressure_plate" SPEC_PRESSURE_PLATE,
    IRON_DOOR "iron_door" SPEC_DOOR,
    WOODEN_PRESSURE_PLATE "wooden_pressure_plate" SPEC_WOODEN_PRESSURE_PLATE, // Merged
    REDSTONE_ORE "redstone_ore" SPEC_LIT,
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
    STAINED_GLASS "stained_glass" SPEC_COLORED,
    WOODEN_TRAPDOOR "wooden_trapdoor" SPEC_WOODEN_TRAPDOOR, // Merged
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
    BROWN_MUSHROOM_BLOCK "brown_mushroom_block" SPEC_MUSHROOM_BLOCK,
    RED_MUSHROOM_BLOCK "red_mushroom_block" SPEC_MUSHROOM_BLOCK,
    MUSHROOM_STEM "mushroom_stem" SPEC_MUSHROOM_BLOCK,
    IRON_BARS "iron_bars" SPEC_BARS,
    CHAIN "chain" SPEC_AXIS_WATERLOGGED,
    GLASS_PANE "glass_pane" SPEC_BARS,
    MELON "melon",
    ATTACHED_PUMPKIN_STEM "attached_pumpkin_stem" SPEC_HORIZONTAL_FACING,
    ATTACHED_MELON_STEM "attached_melon_stem" SPEC_HORIZONTAL_FACING,
    PUMPKIN_STEM "pumpkin_stem" SPEC_CROP,
    MELON_STEM "melon_stem" SPEC_CROP,
    VINE "vine" SPEC_VINE,
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
    CAULDRON "cauldron" SPEC_CAULDRON,
    END_PORTAL "end_portal",
    END_PORTAL_FRAME "end_portal_frame" SPEC_END_PORTAL,
    END_STONE "end_stone",
    DRAGON_EGG "dragon_egg",
    REDSTONE_LAMP "redstone_lamp" SPEC_LIT,
    COCOA "cocoa" SPEC_COCOA,
    SANDSTONE_STAIRS "sandstone_stairs" SPEC_STAIRS,
    EMERALD_ORE "emerald_ore",
    ENDER_CHEST "ender_chest" SPEC_ENDER_CHEST,
    TRIPWIRE_HOOK "tripwire_hook" SPEC_TRIPWIRE_HOOK,
    TRIPWIRE "tripwire" SPEC_TRIPWIRE,
    EMERALD_BLOCK "emerald_block",
    COMMAND_BLOCK "command_block" SPEC_COMMAND_BLOCK, // Merged
    BEACON "beacon",
    COBBLESTONE_WALL "cobblestone_wall" SPEC_WALL,
    MOSSY_COBBLESTONE_WALL "mossy_cobblestone_wall" SPEC_WALL,
    // FLOWER_POT "flower_pot"         [PROP_POT_CONTENT], // Merged TODO
    CARROTS "carrots" SPEC_CROP,
    POTATOES "potatoes" SPEC_CROP,
    WOODEN_BUTTON "wooden_button" SPEC_WOODEN_BUTTON,
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
    COLORED_TERRACOTTA "colored_terracotta" SPEC_COLORED, // Merged
    STAINED_GLASS_PANE "stained_glass_pane" SPEC_BARS_COLORED, // Merged
    SLIME_BLOCK "slime_block",
    BARRIER "barrier",
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
    CARPET "carpet" SPEC_COLORED, // Merged
    COAL_BLOCK "coal_block",
    PACKED_ICE "packed_ice",
    SUNFLOWER "sunflower" SPEC_DOUBLE_PLANT,
    LILAC "lilac" SPEC_DOUBLE_PLANT,
    ROSE_BUSH "rose_bush" SPEC_DOUBLE_PLANT,
    PEONY "peony" SPEC_DOUBLE_PLANT,
    TALL_GRASS "tall_grass" SPEC_DOUBLE_PLANT,
    LARGE_FERN "large_fern" SPEC_DOUBLE_PLANT,
    BANNER "banner" SPEC_BANNER, // Merged
    WALL_BANNER "wall_banner" SPEC_WALL_BANNER, // Merged
    RED_SANDSTONE "red_sandstone",
    CHISELED_RED_SANDSTONE "chiseled_red_sandstone",
    CUT_RED_SANDSTONE "cut_red_sandstone",
    RED_SANDSTONE_STAIRS "red_sandstone_stairs" SPEC_STAIRS,
    WOODEN_SLAB "wooden_slab" SPEC_WOODEN_SLAB, // Merged
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
    BIRCH_FENCE_GATE "spruce_fence_gate" SPEC_FENCE_GATE,
    JUNGLE_FENCE_GATE "spruce_fence_gate" SPEC_FENCE_GATE,
    ACACIA_FENCE_GATE "spruce_fence_gate" SPEC_FENCE_GATE,
    DARK_OAK_FENCE_GATE "spruce_fence_gate" SPEC_FENCE_GATE,

    // WOODEN_FENCE_GATE "wooden_fence_gate" SPEC_WOODEN_FENCE_GATE, // Merged

    WOODEN_FENCE "wooden_fence" SPEC_WOODEN_FENCE, // Merged
    WOODEN_DOOR "wooden_door" SPEC_WOODEN_DOOR, // Merged
    END_ROD "end_rod" SPEC_FACING,
    CHORUS_PLANT "chorus_plant" SPEC_CHORUS_PLANT,
    CHORUS_FLOWER "chorus_flower" SPEC_CHORUS_FLOWER,
    PURPUR_BLOCK "purpur_block",
    PURPUR_PILLAR "purpur_pillar" SPEC_AXIS,
    PURPUR_STAIRS "purpur_stairs" SPEC_STAIRS,
    END_STONE_BRICKS "end_stone_bricks",
    BEETROOTS "beetroots" SPEC_BEETROOTS,
    GRASS_PATH "grass_path",
    END_GATEWAY "end_gateway",
    FROSTED_ICE "frosted_ice" SPEC_FROSTED_ICE,
    MAGMA_BLOCK "magma_block",
    NETHER_WART_BLOCK "nether_wart_block",
    RED_NETHER_BRICKS "red_nether_bricks",
    BONE_BLOCK "bone_block" SPEC_AXIS,
    STRUCTURE_VOID "structure_void",
    OBSERVER "observer" SPEC_OBSERVER,
    SHULKER_BOX "shulker_box" SPEC_SHULKER_BOX,
    COLORED_SHULKER_BOX "colored_shulker_box" SPEC_COLORED_SHULKER_BOX, // Merged
    GLAZED_TERRACOTTA "glazed_terracotta" SPEC_GLAZED_TERRACOTA,
    CONCRETE "concrete" SPEC_COLORED,
    CONCRETE_POWDER "concrete_powder" SPEC_COLORED,
    KELP "kelp" SPEC_KELP,
    KELP_PLANT "kelp_plant",
    DRIED_KELP_BLOCK "dried_kelp_block",
    TURTLE_EGG "turtle_egg" SPEC_TURTLE_EGG,
    CORAL_BLOCK "coral_block" SPEC_CORAL, // Merged
    DEAD_CORAL_BLOCK "dead_coral_block" SPEC_CORAL, // Merged
    CORAL "coral" SPEC_CORAL, // Merged
    DEAD_CORAL "dead_coral" SPEC_CORAL, // Merged
    CORAL_FAN "coral_fan" SPEC_CORAL, // Merged
    DEAD_CORAL_FAN "dead_coral_fan" SPEC_CORAL, // Merged
    SEA_PICKLE "sea_pickle" SPEC_SEA_PICKLE,
    BLUE_ICE "blue_ice",
    CONDUIT "conduit" SPEC_WATERLOGGED,
    BAMBOO_SAPLING "bamboo_sapling",
    BAMBOO "bamboo" SPEC_BAMBOO,
    // TODO: VOID_AIR, CAVE_AIR (skip them?)
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
    SOUL_LANTERN "lantern" SPEC_LANTERN,
    CAMPFIRE "campfire" SPEC_CAMPFIRE,
    SWEET_BERRY_BUSH "sweet_berry_bush" SPEC_SWEET_BERRY_BUSH,
    STEM "stem" SPEC_NETHER_WOOD_TYPE, // Merged
    STRIPPED_STEM "stripped_stem" SPEC_NETHER_WOOD_TYPE, // Merged
    HYPHAE "hyphae" SPEC_NETHER_WOOD_TYPE, // Merged
    STRIPPED_HYPHAE "stripped_hyphae" SPEC_NETHER_WOOD_TYPE, // Merged
    NYLIUM "nylium" SPEC_NETHER_WOOD_TYPE, // Merged
    FUNGUS "fungus" SPEC_NETHER_WOOD_TYPE, // Merged
    ROOTS "roots" SPEC_NETHER_WOOD_TYPE, // Merged
    WARPED_WART_BLOCK "warped_wart_block",
    NETHER_SPROUTS "nether_sprouts",
    SHROOMLIGHT "shroomlight",
    WEEPING_VINES "weeping_vines" SPEC_NETHER_VINE,
    WEEPING_VINES_PLANT "weeping_vines_plant",
    TWISTING_VINES "twisting_vines" SPEC_NETHER_VINE,
    TWISTING_VINES_PLANT "twisting_vines_plant",
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
]);


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BambooLeaves {
    None,
    Large,
    Small
}

impl_enum_serializable!(BambooLeaves {
    None: "none",
    Large: "large",
    Small: "small"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BedPart {
    Foot,
    Head
}

impl_enum_serializable!(BedPart {
    Foot: "foot",
    Head: "head"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Face {
    Ceiling,
    Floor,
    Wall,
    DoubleWall,
    SingleWall
}

impl_enum_serializable!(Face {
    Ceiling: "ceiling",
    Floor: "floor",
    Wall: "wall",
    DoubleWall: "double_wall",
    SingleWall: "single_wall"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instrument {
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
}

impl_enum_serializable!(Instrument {
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


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RailShape {
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
}

impl_enum_serializable!(RailShape {
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


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ComparatorMode {
    Compare,
    Subtract
}

impl_enum_serializable!(ComparatorMode {
    Compare: "compare",
    Subtract: "subtract"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RedstoneWireMode {
    None,
    Side,
    Up
}

impl_enum_serializable!(RedstoneWireMode {
    None: "none",
    Side: "side",
    Up: "up"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DoubleBlockHalf {
    Lower,
    Upper
}

impl_enum_serializable!(DoubleBlockHalf {
    Lower: "lower",
    Upper: "upper"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Half {
    Top,
    Bottom
}

impl_enum_serializable!(Half {
    Top: "top",
    Bottom: "bottom"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DoorHingeSide {
    Left,
    Right
}

impl_enum_serializable!(DoorHingeSide {
    Left: "left",
    Right: "right"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WoodType {
    Oak,
    Spruce,
    Birch,
    Jungle,
    Acacia,
    DarkOak,
    Crimson,
    Warped
}

impl_enum_serializable!(WoodType {
    Oak: "oak",
    Spruce: "spruce",
    Birch: "birch",
    Jungle: "jungle",
    Acacia: "acacia",
    DarkOak: "dark_oak",
    Crimson: "crimson",
    Warped: "warped"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChestType {
    Single,
    Left,
    Right
}

impl_enum_serializable!(ChestType {
    Single: "single",
    Left: "left",
    Right: "right"
});



#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StairsShape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight
}

impl_enum_serializable!(StairsShape {
    Straight: "straight",
    InnerLeft: "inner_left",
    InnerRight: "inner_right",
    OuterLeft: "outer_left",
    OuterRight: "outer_right"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WallSide {
    None,
    Low,
    Tall
}

impl_enum_serializable!(WallSide {
    None: "none",
    Low: "low",
    Tall: "tall"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PotContent {
    // TODO: PotContent is my worst idea, maybe "unmerge" pots.
    None,
    OakSapling,
    SpruceSapling,
    BirchSapling,
    JungleSapling,
    AcaciaSapling,
    DarkOakSapling,
    Fern,
    Dandelion,
    Poppy,
    BlueOrchid,
    Allium,
    AzureBluet,
    RedTulip,
    OrangeTulip,
    WhiteTulip,
    PinkTulip,
    OxeyeDaisy,
    Cornflower,
    LilyOfTheValley,
    WitherRose,
    RedMushroom,
    BrownMushroom,
    DeadBush,
    Cactus,
    Bamboo,
    CrimsonFungus,
    WarpedFungus,
    CrimsonRoots,
    WarpedRoots
}

impl_enum_serializable!(PotContent {
    None: "none",
    OakSapling: "oak_sapling",
    SpruceSapling: "spruce_sapling",
    BirchSapling: "birch_sapling",
    JungleSapling: "jungle_sapling",
    AcaciaSapling: "acacia_sapling",
    DarkOakSapling: "dark_oak_sapling",
    Fern: "fern",
    Dandelion: "dandelion",
    Poppy: "poppy",
    BlueOrchid: "blue_orchid",
    Allium: "allium",
    AzureBluet: "azure_bluet",
    RedTulip: "red_tulip",
    OrangeTulip: "orange_tulip",
    WhiteTulip: "white_tulip",
    PinkTulip: "pink_tulip",
    OxeyeDaisy: "oxeye_daisy",
    Cornflower: "cornflower",
    LilyOfTheValley: "lily_of_the_valley",
    WitherRose: "wither_rose",
    RedMushroom: "red_mushroom",
    BrownMushroom: "brown_mushroom",
    DeadBush: "dead_bush",
    Cactus: "cactus",
    Bamboo: "bamboo",
    CrimsonFungus: "crimson_fungus",
    WarpedFungus: "warped_fungus",
    CrimsonRoots: "crimson_roots",
    WarpedRoots: "warped_roots"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SlabType {
    Top,
    Bottom,
    Double
}

impl_enum_serializable!(SlabType {
    Top: "top",
    Bottom: "bottom",
    Double: "double"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CommandBlockType {
    Impulse,
    Repeating,
    Chain
}

impl_enum_serializable!(CommandBlockType {
    Impulse: "impulse",
    Repeating: "repeating",
    Chain: "chain"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CoralType {
    Tube,
    Brain,
    Bubble,
    Fire,
    Horn
}

impl_enum_serializable!(CoralType {
    Tube: "tube",
    Brain: "brain",
    Bubble: "bubble",
    Fire: "fire",
    Horn: "horn"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StructureMode {
    Save,
    Load,
    Corner,
    Data
}

impl_enum_serializable!(StructureMode {
    Save: "save",
    Load: "load",
    Corner: "corner",
    Data: "data"
});


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FrontAndTop {
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
}

impl_enum_serializable!(FrontAndTop {
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
