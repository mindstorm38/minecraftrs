use crate::{blocks, properties, properties_groups, impl_enum_serializable};
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

    pub PROP_DOWN: bool("down");
    pub PROP_EAST: bool("east");
    pub PROP_NORTH: bool("north");
    pub PROP_SOUTH: bool("south");
    pub PROP_UP: bool("up");
    pub PROP_WEST: bool("west");
    pub PROP_BOTTOM: bool("bottom");

    pub PROP_FACING: enum("facing", Direction, FACING, [
        East, North, South, West
    ]);

    pub PROP_FACING_ALL: enum("facing", Direction, FACING_ALL, [
        Down, East, North, South, Up, West
    ]);

    pub PROP_AXIS: enum("axis", Axis, AXIS, [X, Y, Z]);

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

    pub PROP_SKULL_TYPE: enum("type", SkullType, SKULL_TYPE, [
        Skeleton,
        WitherSkeleton,
        Zombie,
        Creeper,
        Dragon,
        Player
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

properties_groups! {
    pub PROPS_FIRE: [PROP_AGE_16, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST, PROP_UP];
    pub PROPS_STAIRS: [PROP_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED];
    pub PROPS_WALLS: [PROP_UP, PROP_WALL_EAST, PROP_WALL_NORTH, PROP_WALL_SOUTH, PROP_WALL_WEST, PROP_WATERLOGGED];
    pub PROPS_MUSHROOM_BLOCKS: [PROP_UP, PROP_DOWN, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST];
    pub PROPS_BARS: [PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED];
    pub PROPS_CESTS: [PROP_FACING, PROP_CHEST_TYPE, PROP_WATERLOGGED];
    pub PROPS_FENCE_GATES: [PROP_FACING, PROP_OPEN, PROP_POWERED, PROP_IN_WALL];
    pub PROPS_TRAPDOORS: [PROP_FACING, PROP_OPEN, PROP_HALF, PROP_POWERED, PROP_WATERLOGGED];
    pub PROPS_DOORS: [PROP_DOUBLE_BLOCK_HALF, PROP_FACING, PROP_OPEN, PROP_DOOR_HINGE, PROP_POWERED];
    pub PROPS_DOUBLE_PLANTS: [PROP_DOUBLE_BLOCK_HALF];
    pub PROPS_BUTTONS: [PROP_FACING, PROP_POWERED, PROP_FACE];
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
    GRASS_BLOCK "grass_block"       [PROP_SNOWY],
    PODZOL "podzol"                 [PROP_SNOWY],
    DIRT "dirt",
    COARSE_DIRT "coarse_dirt",
    COBBLESTONE "cobblestone",
    PLANKS "planks"                 [PROP_ALL_WOOD_TYPE], // Merged
    SAPLING "sapling"               [PROP_OVERWORLD_WOOD_TYPE], // Merged
    BEDROCK "bedrock",
    WATER "water"                   [PROP_LIQUID_FALLING],
    FLOWING_WATER "flowing_water"   [PROP_LIQUID_FALLING, PROP_LIQUID_LEVEL],
    LAVA "lava"                     [PROP_LIQUID_FALLING],
    FLOWING_LAVA "flowing_lava"     [PROP_LIQUID_FALLING, PROP_LIQUID_LEVEL],
    SAND "sand",
    RED_SAND "red_sand",
    GRAVEL "gravel",
    GOLD_ORE "gold_ore",
    IRON_ORE "iron_ore",
    COAL_ORE "coal_ore",
    NETHER_GOLD_ORE "nether_gold_ore",
    LOG "log"                       [PROP_OVERWORLD_WOOD_TYPE], // Merged
    STRIPPED_LOG "stripped_log"     [PROP_OVERWORLD_WOOD_TYPE], // Merged
    WOOD "wood"                     [PROP_OVERWORLD_WOOD_TYPE], // Merged
    STRIPPED_WOOD "stripped_wood"   [PROP_OVERWORLD_WOOD_TYPE], // Merged
    LEAVES "leaves"                 [PROP_OVERWORLD_WOOD_TYPE], // Merged
    SPONGE "sponge",
    WET_SPONGE "wet_sponge",
    GLASS "glass",
    LAPIS_ORE "lapis_ore",
    LAPIS_BLOCK "lapis_block",
    DISPENSER "dispenser"           [PROP_FACING_ALL, PROP_TRIGGERED],
    SANDSTONE "sandstone",
    CHISELED_SANDSTONE "chiseled_sandstone",
    CUT_SANDSTONE "cut_sandstone",
    NOTE_BLOCK "note_block"         [PROP_INSTRUMENT, PROP_NOTE, PROP_POWERED],
    BED "bed"                       [PROP_COLOR, PROP_FACING, PROP_BED_PART, PROP_OCCUPIED],
    POWERED_RAIL "powered_rail"     [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED],
    DETECTOR_RAIL "detector_rail"   [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED],
    PISTON "piston"                 [PROP_STICKY, PROP_FACING_ALL, PROP_EXTENDED], // Merged the two piston type into one using a property "sticky".
    PISTON_HEAD "piston_head"       [PROP_FACING_ALL, PROP_STICKY, PROP_SHORT], // 'sticky' property instead of 'type'
    MOVING_PISTON "moving_piston"   [PROP_FACING_ALL, PROP_STICKY], // 'sticky' property instead of 'type'
    COBWEB "cobweb",
    GRASS "grass",
    FERN "fern",
    DEAD_BUSH "dead_bush",
    SEAGRASS "seagrass",
    TALL_SEAGRASS "tall_seagrass"   [(...PROPS_DOUBLE_PLANTS)],
    WOOL "wool"                     [PROP_COLOR],
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
    TNT "tnt"                       [PROP_UNSTABLE],
    BOOKSHELF "bookshelf",
    MOSSY_COBBLESTONE "mossy_cobblestone",
    OBSIDIAN "obsidian",
    TORCH "torch",
    WALL_TORCH "wall_torch"         [PROP_FACING],
    FIRE "fire"                     [(...PROPS_FIRE)],
    SOUL_FIRE "fire"                [(...PROPS_FIRE)],
    SPAWNER "spawner",
    WOODEN_STAIRS "wooden_stairs"   [PROP_ALL_WOOD_TYPE, (...PROPS_STAIRS)], // Merged
    CHEST "chest"                   [(...PROPS_CESTS)],
    REDSTONE_WIRE "redstone_wire"   [PROP_REDSTONE_POWER, PROP_REDSTONE_EAST, PROP_REDSTONE_NORTH, PROP_REDSTONE_SOUTH, PROP_REDSTONE_WEST],
    DIAMOND_ORE "diamond_ore",
    DIAMOND_BLOCK "diamond_block",
    CRAFTING_TABLE "crafting_table",
    WHEAT "wheat"                   [PROP_AGE_8],
    FARMLAND "farmland"             [PROP_FARMLAND_MOISTURE],
    FURNACE "furnace"               [PROP_FACING, PROP_LIT],
    SIGN "sign"                     [PROP_ALL_WOOD_TYPE, PROP_ROTATION, PROP_WATERLOGGED],
    LADDER "ladder"                 [PROP_FACING, PROP_WATERLOGGED],
    RAIL "rail"                     [PROP_RAIL_SHAPE],
    COBBLESTONE_STAIRS "cobblestone_stairs" [(...PROPS_STAIRS)],
    WALL_SIGN "wooden_wall_sign"    [PROP_ALL_WOOD_TYPE, PROP_FACING, PROP_WATERLOGGED], // Merged
    LEVER "lever"                   [PROP_FACE, PROP_FACING, PROP_POWERED],
    STONE_PRESSURE_PLATE "stone_pressure_plate" [PROP_POWERED],
    IRON_DOOR "iron_door"           [(...PROPS_DOORS)],
    WOODEN_PRESSURE_PLATE "wooden_pressure_plate" [PROP_ALL_WOOD_TYPE, PROP_POWERED], // Merged
    REDSTONE_ORE "redstone_ore"     [PROP_LIT],
    REDSTONE_TORCH "redstone_torch" [PROP_LIT],
    REDSTONE_WALL_TORCH "redstone_wall_torch" [PROP_FACING, PROP_LIT],
    STONE_BUTTON "stone_button"     [(...PROPS_BUTTONS)],
    SNOW "snow"                     [PROP_SNOW_LAYERS],
    ICE "ice",
    SNOW_BLOCK "snow_block",
    CACTUS "cactus"                 [PROP_AGE_16],
    CLAY "clay",
    SUGAR_CANE "sugar_cane"         [PROP_AGE_16],
    JUKEBOX "jukebox"               [PROP_HAS_RECORD],
    PUMPKIN "pumpkin",
    NETHERRACK "netherrack",
    SOUL_SAND "soul_sand",
    SOUL_SOIL "soul_soil",
    BASALT "basalt"                 [PROP_AXIS],
    POLISHED_BASALT "polished_basalt" [PROP_AXIS],
    SOUL_TORCH "soul_torch",
    SOUL_WALL_TORCH "soul_wall_torch" [PROP_FACING],
    GLOWSTONE "glowstone",
    NETHER_PORTAL "nether_portal"   [PROP_AXIS],
    CARVED_PUMPKIN "carved_pumpkin" [PROP_FACING],
    JACK_O_LANTERN "jack_o_lantern" [PROP_FACING],
    CAKE "cake"                     [PROP_CAKE_BITES],
    REPEATER "repeater"             [PROP_REPEATER_DELAY, PROP_FACING, PROP_LOCKED, PROP_POWERED],
    STAINED_GLASS "stained_glass"   [PROP_COLOR],
    WOODEN_TRAPDOOR "wooden_trapdoor" [PROP_ALL_WOOD_TYPE, (...PROPS_TRAPDOORS)], // Merged
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
    BROWN_MUSHROOM_BLOCK "brown_mushroom_block" [(...PROPS_MUSHROOM_BLOCKS)],
    RED_MUSHROOM_BLOCK "red_mushroom_block" [(...PROPS_MUSHROOM_BLOCKS)],
    MUSHROOM_STEM "mushroom_stem"   [(...PROPS_MUSHROOM_BLOCKS)],
    IRON_BARS "iron_bars"           [(...PROPS_BARS)],
    CHAIN "chain"                   [PROP_WATERLOGGED, PROP_AXIS],
    GLASS_PANE "glass_pane"         [(...PROPS_BARS)],
    MELON "melon",
    ATTACHED_PUMPKIN_STEM "attached_pumpkin_stem" [PROP_FACING],
    ATTACHED_MELON_STEM "attached_melon_stem" [PROP_FACING],
    PUMPKIN_STEM "pumpkin_stem"     [PROP_AGE_8],
    MELON_STEM "melon_stem"         [PROP_AGE_8],
    VINE "vine"                     [PROP_UP, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST],
    BRICK_STAIRS "brick_stairs"     [(...PROPS_STAIRS)],
    STONE_BRICK_STAIRS "stone_brick_stairs" [(...PROPS_STAIRS)],
    MYCELIUM "mycelium"             [PROP_SNOWY],
    LILY_PAD "lily_pad",
    NETHER_BRICKS "nether_bricks",
    NETHER_BRICK_FENCE "nether_brick_fence" [(...PROPS_BARS)],
    NETHER_BRICK_STAIRS "nether_brick_stairs" [(...PROPS_STAIRS)],
    NETHER_WART "nether_wart"       [PROP_AGE_4],
    ENCHANTING_TABLE "enchanting_table",
    BREWING_STAND "brewing_stand"   [PROP_HAS_BOTTLE_0, PROP_HAS_BOTTLE_1, PROP_HAS_BOTTLE_2],
    CAULDRON "cauldron"             [PROP_CAULDRON_LEVEL],
    END_PORTAL "end_portal",
    END_PORTAL_FRAME "end_portal_frame" [PROP_END_PORTAL_EYE, PROP_FACING],
    END_STONE "end_stone",
    DRAGON_EGG "dragon_egg",
    REDSTONE_LAMP "redstone_lamp"   [PROP_LIT],
    COCOA "cocoa"                   [PROP_AGE_3, PROP_FACING],
    SANDSTONE_STAIRS "sandstone_stairs" [(...PROPS_STAIRS)],
    EMERALD_ORE "emerald_ore",
    ENDER_CHEST "ender_chest"       [PROP_FACING, PROP_WATERLOGGED],
    TRIPWIRE_HOOK "tripwire_hook"   [PROP_ATTACHED, PROP_FACING, PROP_POWERED],
    TRIPWIRE "tripwire"             [PROP_ATTACHED, PROP_DISARMED, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_WEST, PROP_POWERED],
    EMERALD_BLOCK "emerald_block",
    COMMAND_BLOCK "command_block"   [PROP_COMMAND_BLOCK_TYPE, PROP_FACING, PROP_CONDITIONAL], // Merged
    BEACON "beacon",
    COBBLESTONE_WALL "cobblestone_wall" [(...PROPS_WALLS)],
    MOSSY_COBBLESTONE_WALL "mossy_cobblestone_wall" [(...PROPS_WALLS)],
    FLOWER_POT "flower_pot"         [PROP_POT_CONTENT], // Merged
    CARROTS "carrots"               [PROP_AGE_8],
    POTATOES "potatoes"             [PROP_AGE_8],
    WOODEN_BUTTON "wooden_button"   [PROP_ALL_WOOD_TYPE, (...PROPS_BUTTONS)],
    SKULL "skull"                   [PROP_SKULL_TYPE, PROP_ROTATION],
    WALL_SKULL "skull"              [PROP_SKULL_TYPE, PROP_FACING],
    ANVIL "anvil"                   [PROP_FACING],
    CHIPPED_ANVIL "chipped_anvil"   [PROP_FACING],
    DAMAGED_ANVIL "damaged_anvil"   [PROP_FACING],
    TRAPPED_CHEST "trapped_chest"   [(...PROPS_CESTS)],
    LIGHT_WEIGHTED_PRESSURE_PLATE "light_weighted_pressure_plate" [PROP_REDSTONE_POWER],
    HEAVY_WEIGHTED_PRESSURE_PLATE "heavy_weighted_pressure_plate" [PROP_REDSTONE_POWER],
    COMPARATOR "comparator"         [PROP_FACING, PROP_COMPARATOR_MODE, PROP_POWERED],
    DAYLIGHT_DETECTOR "daylight_detector" [PROP_INVERTED, PROP_REDSTONE_POWER],
    REDSTONE_BLOCK "redstone_block",
    NETHER_QUARTZ_ORE "nether_quartz_ore",
    HOPPER "hopper"                 [PROP_FACING, PROP_ENABLED],
    QUARTZ_BLOCK "quartz_block",
    CHISELED_QUARTZ_BLOCK "chiseled_quartz_block",
    QUARTZ_PILLAR "quartz_pillar"   [PROP_AXIS],
    QUARTZ_STAIRS "quartz_stairs"   [(...PROPS_STAIRS)],
    ACTIVATOR_RAIL "activator_rail" [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED],
    DROPPER "dropper"               [PROP_FACING_ALL, PROP_TRIGGERED],
    TERRACOTTA "terracotta",
    COLORED_TERRACOTTA "colored_terracotta" [PROP_COLOR], // Merged
    STAINED_GLASS_PANE "stained_glass_pane" [(...PROPS_BARS), PROP_COLOR], // Merged
    SLIME_BLOCK "slime_block",
    BARRIER "barrier",
    IRON_TRAPDOOR "iron_trapdoor"   [(...PROPS_TRAPDOORS)],
    PRISMARINE "prismarine",
    PRISMARINE_BRICKS "prismarine_bricks",
    DARK_PRISMARINE "dark_prismarine",
    PRISMARINE_STAIRS "prismarine_stairs" [(...PROPS_STAIRS)],
    PRISMARINE_BRICK_STAIRS "prismarine_brick_stairs" [(...PROPS_STAIRS)],
    DARK_PRISMARINE_STAIRS "dark_prismarine_stairs" [(...PROPS_STAIRS)],
    PRISMARINE_SLAB "prismarine_slab" [PROP_SLAB_TYPE],
    PRISMARINE_BRICK_SLAB "prismarine_brick_slab" [PROP_SLAB_TYPE],
    DARK_PRISMARINE_SLAB "dark_prismarine_slab" [PROP_SLAB_TYPE],
    SEA_LANTERN "sea_lantern",
    HAY_BLOCK "hay_block"           [PROP_AXIS],
    CARPET "carpet"                 [PROP_COLOR], // Merged
    COAL_BLOCK "coal_block",
    PACKED_ICE "packed_ice",
    SUNFLOWER "sunflower"           [(...PROPS_DOUBLE_PLANTS)],
    LILAC "lilac"                   [(...PROPS_DOUBLE_PLANTS)],
    ROSE_BUSH "rose_bush"           [(...PROPS_DOUBLE_PLANTS)],
    PEONY "peony"                   [(...PROPS_DOUBLE_PLANTS)],
    TALL_GRASS "tall_grass"         [(...PROPS_DOUBLE_PLANTS)],
    LARGE_FERN "large_fern"         [(...PROPS_DOUBLE_PLANTS)],
    BANNER "banner"                 [PROP_ROTATION, PROP_COLOR], // Merged
    WALL_BANNER "wall_banner"       [PROP_FACING, PROP_COLOR], // Merged
    RED_SANDSTONE "red_sandstone",
    CHISELED_RED_SANDSTONE "chiseled_red_sandstone",
    CUT_RED_SANDSTONE "cut_red_sandstone",
    RED_SANDSTONE_STAIRS "red_sandstone_stairs" [(...PROPS_STAIRS)],
    WOODEN_SLAB "wooden_slab"       [PROP_ALL_WOOD_TYPE, PROP_SLAB_TYPE], // Merged
    STONE_SLAB "stone_slab"         [PROP_SLAB_TYPE],
    SMOOTH_STONE_SLAB "smooth_stone_slab" [PROP_SLAB_TYPE],
    SANDSTONE_SLAB "sandstone_slab" [PROP_SLAB_TYPE],
    CUT_SANDSTONE_SLAB "cut_sandstone_slab" [PROP_SLAB_TYPE],
    PETRIFIED_OAK_SLAB "petrified_oak_slab" [PROP_SLAB_TYPE],
    COBBLESTONE_SLAB "cobblestone_slab" [PROP_SLAB_TYPE],
    BRICK_SLAB "brick_slab"         [PROP_SLAB_TYPE],
    STONE_BRICK_SLAB "stone_brick_slab" [PROP_SLAB_TYPE],
    NETHER_BRICK_SLAB "nether_brick_slab" [PROP_SLAB_TYPE],
    QUARTZ_SLAB "quartz_slab"       [PROP_SLAB_TYPE],
    RED_SANDSTONE_SLAB "red_sandstone_slab" [PROP_SLAB_TYPE],
    CUT_RED_SANDSTONE_SLAB "cut_red_sandstone_slab" [PROP_SLAB_TYPE],
    PURPUR_SLAB "purpur_slab"       [PROP_SLAB_TYPE],
    SMOOTH_STONE "smooth_stone",
    SMOOTH_SANDSTONE "smooth_sandstone",
    SMOOTH_QUARTZ "smooth_quartz",
    SMOOTH_RED_SANDSTONE "smooth_red_sandstone",
    WOODEN_FENCE_GATE "wooden_fence_gate" [PROP_ALL_WOOD_TYPE, (...PROPS_FENCE_GATES)], // Merged
    WOODEN_FENCE "wooden_fence"     [PROP_ALL_WOOD_TYPE, (...PROPS_BARS)], // Merged
    WOODEN_DOOR "wooden_door"       [PROP_ALL_WOOD_TYPE, (...PROPS_DOORS)], // Merged
    END_ROD "end_rod"               [PROP_FACING_ALL],
    CHORUS_PLANT "chorus_plant"     [PROP_DOWN, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_UP, PROP_WEST],
    CHORUS_FLOWER "chorus_flower"   [PROP_AGE_6],
    PURPUR_BLOCK "purpur_block",
    PURPUR_PILLAR "purpur_pillar"   [PROP_AXIS],
    PURPUR_STAIRS "purpur_stairs"   [(...PROPS_STAIRS)],
    END_STONE_BRICKS "end_stone_bricks",
    BEETROOTS "beetroots"           [PROP_AGE_4],
    GRASS_PATH "grass_path",
    END_GATEWAY "end_gateway",
    FROSTED_ICE "frosted_ice"       [PROP_AGE_4],
    MAGMA_BLOCK "magma_block",
    NETHER_WART_BLOCK "nether_wart_block",
    RED_NETHER_BRICKS "red_nether_bricks",
    BONE_BLOCK "bone_block"         [PROP_AXIS],
    STRUCTURE_VOID "structure_void",
    OBSERVER "observer"             [PROP_FACING_ALL, PROP_POWERED],
    SHULKER_BOX "shulker_box"       [PROP_FACING_ALL],
    COLORED_SHULKER_BOX "colored_shulker_box" [PROP_FACING_ALL], // Merged
    GLAZED_TERRACOTTA "glazed_terracotta" [PROP_COLOR, PROP_FACING],
    CONCRETE "concrete"             [PROP_COLOR],
    CONCRETE_POWDER "concrete_powder" [PROP_COLOR],
    KELP "kelp"                     [PROP_AGE_26],
    KELP_PLANT "kelp_plant",
    DRIED_KELP_BLOCK "dried_kelp_block",
    TURTLE_EGG "turtle_egg"         [PROP_EGGS, PROP_HATCH],
    CORAL_BLOCK "coral_block"       [PROP_CORAL_TYPE], // Merged
    DEAD_CORAL_BLOCK "dead_coral_block" [PROP_CORAL_TYPE], // Merged
    CORAL "coral"                   [PROP_CORAL_TYPE], // Merged
    DEAD_CORAL "dead_coral"         [PROP_CORAL_TYPE], // Merged
    CORAL_FAN "coral_fan"           [PROP_CORAL_TYPE], // Merged
    DEAD_CORAL_FAN "dead_coral_fan" [PROP_CORAL_TYPE], // Merged
    SEA_PICKLE "sea_pickle"         [PROP_PICKLES, PROP_WATERLOGGED],
    BLUE_ICE "blue_ice",
    CONDUIT "conduit"               [PROP_WATERLOGGED],
    BAMBOO_SAPLING "bamboo_sapling",
    BAMBOO "bamboo"                 [PROP_BAMBOO_AGE, PROP_BAMBOO_LEAVES, PROP_BAMBOO_STAGE],
    // TODO: VOID_AIR, CAVE_AIR (skip them?)
    BUBBLE_COLUMN "bubble_column"   [PROP_DRAG],
    POLISHED_GRANITE_STAIRS "polished_granite_stairs" [(...PROPS_STAIRS)],
    SMOOTH_RED_SANDSTONE_STAIRS "smooth_red_sandstone_stairs" [(...PROPS_STAIRS)],
    MOSSY_STONE_BRICK_STAIRS "mossy_stone_brick_stairs" [(...PROPS_STAIRS)],
    POLISHED_DIORITE_STAIRS "polished_diorite_stairs" [(...PROPS_STAIRS)],
    MOSSY_COBBLESTONE_STAIRS "mossy_cobblestone_stairs" [(...PROPS_STAIRS)],
    END_STONE_BRICK_STAIRS "end_stone_brick_stairs" [(...PROPS_STAIRS)],
    STONE_STAIRS "stone_stairs"     [(...PROPS_STAIRS)],
    SMOOTH_SANDSTONE_STAIRS "smooth_sandstone_stairs" [(...PROPS_STAIRS)],
    SMOOTH_QUARTZ_STAIRS "smooth_quartz_stairs" [(...PROPS_STAIRS)],
    GRANITE_STAIRS "granite_stairs" [(...PROPS_STAIRS)],
    ANDESITE_STAIRS "andesite_stairs" [(...PROPS_STAIRS)],
    RED_NETHER_BRICK_STAIRS "red_nether_brick_stairs" [(...PROPS_STAIRS)],
    POLISHED_ANDESITE_STAIRS "polished_andesite_stairs" [(...PROPS_STAIRS)],
    DIORITE_STAIRS "diorite_stairs" [(...PROPS_STAIRS)],
    POLISHED_GRANITE_SLAB "polished_granite_slab" [PROP_SLAB_TYPE],
    SMOOTH_RED_SANDSTONE_SLAB "smooth_red_sandstone_slab" [PROP_SLAB_TYPE],
    MOSSY_STONE_BRICK_SLAB "mossy_stone_brick_slab" [PROP_SLAB_TYPE],
    POLISHED_DIORITE_SLAB "polished_diorite_slab" [PROP_SLAB_TYPE],
    MOSSY_COBBLESTONE_SLAB "mossy_cobblestone_slab" [PROP_SLAB_TYPE],
    END_STONE_BRICK_SLAB "end_stone_brick_slab" [PROP_SLAB_TYPE],
    SMOOTH_SANDSTONE_SLAB "smooth_sandstone_slab" [PROP_SLAB_TYPE],
    SMOOTH_QUARTZ_SLAB "smooth_quartz_slab" [PROP_SLAB_TYPE],
    GRANITE_SLAB "granite_slab"     [PROP_SLAB_TYPE],
    ANDESITE_SLAB "andesite_slab"   [PROP_SLAB_TYPE],
    RED_NETHER_BRICK_SLAB "red_nether_brick_slab" [PROP_SLAB_TYPE],
    POLISHED_ANDESITE_SLAB "polished_andesite_slab" [PROP_SLAB_TYPE],
    DIORITE_SLAB "diorite_slab"     [PROP_SLAB_TYPE],
    BRICK_WALL "brick_wall"         [(...PROPS_WALLS)],
    PRISMARINE_WALL "prismarine_wall" [(...PROPS_WALLS)],
    RED_SANDSTONE_WALL "red_sandstone_wall" [(...PROPS_WALLS)],
    MOSSY_STONE_BRICK_WALL "mossy_stone_brick_wall" [(...PROPS_WALLS)],
    GRANITE_WALL "granite_wall"     [(...PROPS_WALLS)],
    STONE_BRICK_WALL "stone_brick_wall" [(...PROPS_WALLS)],
    NETHER_BRICK_WALL "nether_brick_wall" [(...PROPS_WALLS)],
    ANDESITE_WALL "andesite_wall"   [(...PROPS_WALLS)],
    RED_NETHER_BRICK_WALL "red_nether_brick_wall" [(...PROPS_WALLS)],
    SANDSTONE_WALL "sandstone_wall" [(...PROPS_WALLS)],
    END_STONE_BRICK_WALL "end_stone_brick_wall" [(...PROPS_WALLS)],
    DIORITE_WALL "diorite_wall"     [(...PROPS_WALLS)],
    SCAFFOLDING "scaffolding"       [PROP_BOTTOM, PROP_SCAFFOLDING_DISTANCE, PROP_WATERLOGGED],
    LOOM "loom"                     [PROP_FACING],
    BARREL "barrel"                 [PROP_FACING_ALL, PROP_OPEN],
    SMOKER "smoker"                 [PROP_FACING, PROP_LIT],
    BLAST_FURNACE "blast_furnace"   [PROP_FACING, PROP_LIT],
    CARTOGRAPHY_TABLE "cartography_table",
    FLETCHING_TABLE "fletching_table",
    GRINDSTONE "grindstone"         [PROP_FACE, PROP_FACING],
    LECTERN "lectern"               [PROP_FACING, PROP_HAS_BOOK, PROP_POWERED],
    SMITHING_TABLE "smithing_table",
    STONECUTTER "stonecutter"       [PROP_FACING],
    BELL "bell"                     [PROP_BELL_ATTACHMENT, PROP_FACING, PROP_POWERED],
    LANTERN "lantern"               [PROP_HANGING],
    SOUL_LANTERN "lantern"          [PROP_HANGING],
    CAMPFIRE "campfire"             [PROP_FACING, PROP_LIT, PROP_SIGNAL_FIRE, PROP_WATERLOGGED],
    SWEET_BERRY_BUSH "sweet_berry_bush" [PROP_AGE_4],
    STEM "stem"                     [PROP_NETHER_WOOD_TYPE], // Merged
    STRIPPED_STEM "stripped_stem"   [PROP_NETHER_WOOD_TYPE], // Merged
    HYPHAE "hyphae"                 [PROP_NETHER_WOOD_TYPE], // Merged
    STRIPPED_HYPHAE "stripped_hyphae" [PROP_NETHER_WOOD_TYPE], // Merged
    NYLIUM "nylium"                 [PROP_NETHER_WOOD_TYPE], // Merged
    FUNGUS "fungus"                 [PROP_NETHER_WOOD_TYPE], // Merged
    ROOTS "roots"                   [PROP_NETHER_WOOD_TYPE], // Merged
    WARPED_WART_BLOCK "warped_wart_block",
    NETHER_SPROUTS "nether_sprouts",
    SHROOMLIGHT "shroomlight",
    WEEPING_VINES "weeping_vines"   [PROP_AGE_26],
    WEEPING_VINES_PLANT "weeping_vines_plant",
    TWISTING_VINES "twisting_vines" [PROP_AGE_26],
    TWISTING_VINES_PLANT "twisting_vines_plant",
    STRUCTURE_BLOCK "structure_block" [PROP_STRUCTURE_MODE],
    JIGSAW "jigsaw"                 [PROP_JIGSAW_ORIENTATION],
    COMPOSTER "composter"           [PROP_COMPOSTER_LEVEL],
    TARGET "target"                 [PROP_REDSTONE_POWER],
    BEE_NEST "bee_nest"             [PROP_FACING, PROP_HONEY_LEVEL],
    BEEHIVE "beehive"               [PROP_FACING, PROP_HONEY_LEVEL],
    HONEY_BLOCK "honey_block",
    HONEYCOMB_BLOCK "honeycomb_block",
    NETHERITE_BLOCK "netherite_block",
    ANCIENT_DEBRIS "ancient_debris",
    CRYING_OBSIDIAN "crying_obsidian",
    RESPAWN_ANCHOR "respawn_anchor" [PROP_CHARGES],
    LODESTONE "lodestone",
    BLACKSTONE "blackstone",
    BLACKSTONE_STAIRS "blackstone_stairs" [(...PROPS_STAIRS)],
    BLACKSTONE_WALL "blackstone_wall" [(...PROPS_WALLS)],
    BLACKSTONE_SLAB "blackstone_slab" [PROP_SLAB_TYPE],
    POLISHED_BLACKSTONE "polished_blackstone",
    POLISHED_BLACKSTONE_BRICKS "polished_blackstone_bricks",
    CRACKED_POLISHED_BLACKSTONE_BRICKS "cracked_polished_blackstone_bricks",
    CHISELED_POLISHED_BLACKSTONE "chiseled_polished_blackstone",
    POLISHED_BLACKSTONE_BRICK_SLAB "polished_blackstone_brick_slab" [PROP_SLAB_TYPE],
    POLISHED_BLACKSTONE_BRICK_STAIRS "polished_blackstone_brick_stairs" [(...PROPS_STAIRS)],
    POLISHED_BLACKSTONE_BRICK_WALL "polished_blackstone_brick_wall" [(...PROPS_WALLS)],
    GILDED_BLACKSTONE "gilded_blackstone" [(...PROPS_WALLS)],
    POLISHED_BLACKSTONE_STAIRS "polished_blackstone_stairs" [(...PROPS_STAIRS)],
    POLISHED_BLACKSTONE_SLAB "polished_blackstone_slab" [PROP_SLAB_TYPE],
    POLISHED_BLACKSTONE_PRESSURE_PLATE "polished_blackstone_pressure_plate" [PROP_POWERED],
    POLISHED_BLACKSTONE_BUTTON "polished_blackstone_button" [(...PROPS_BUTTONS)],
    POLISHED_BLACKSTONE_WALL "polished_blackstone_wall" [(...PROPS_WALLS)],
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
pub enum SkullType {
    Skeleton,
    WitherSkeleton,
    Zombie,
    Creeper,
    Dragon,
    Player
}

impl_enum_serializable!(SkullType {
    Skeleton: "skeleton",
    WitherSkeleton: "wither_skeleton",
    Zombie: "zombie",
    Creeper: "creeper",
    Dragon: "dragon",
    Player: "player"
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
