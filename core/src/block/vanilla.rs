use crate::util::{Direction, Axis, DyeColor};
use crate::{blocks, properties, impl_enum_serializable};


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


static REDSTONE_WIRE_MODE: [RedstoneWireMode; 3] = [RedstoneWireMode::None, RedstoneWireMode::Side, RedstoneWireMode::Up];


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

    pub PROP_DOWN: bool("down");
    pub PROP_EAST: bool("east");
    pub PROP_NORTH: bool("north");
    pub PROP_SOUTH: bool("south");
    pub PROP_UP: bool("up");
    pub PROP_WEST: bool("west");
    pub PROP_BOTTOM: bool("bottom");

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

    pub PROP_DOUBLE_BLOCK_HALF: enum<DoubleBlockHalf>("half", DOUBLE_BLOCK_HALF, [
        DoubleBlockHalf::Lower, DoubleBlockHalf::Upper
    ]);

    pub PROP_DOOR_HINGE: enum<DoorHingeSide>("hinge", DOOR_HINGE, [
        DoorHingeSide::Left, DoorHingeSide::Right
    ]);

    pub PROP_HALF: enum<Half>("half", HALF, [
        Half::Top, Half::Bottom
    ]);

    pub PROP_COLOR: enum<DyeColor>("color", COLOR, [
        DyeColor::White,
        DyeColor::Orange,
        DyeColor::Magenta,
        DyeColor::LightBlue,
        DyeColor::Yellow,
        DyeColor::Lime,
        DyeColor::Pink,
        DyeColor::Gray,
        DyeColor::LightGray,
        DyeColor::Cyan,
        DyeColor::Purple,
        DyeColor::Blue,
        DyeColor::Brown,
        DyeColor::Green,
        DyeColor::Red,
        DyeColor::Black
    ]);

    pub PROP_INSTRUMENT: enum<Instrument>("instrument", INSTRUMENT, [
        Instrument::Banjo,
        Instrument::BassDrum,
        Instrument::Bass,
        Instrument::Bell,
        Instrument::Bit,
        Instrument::Chime,
        Instrument::CowBell,
        Instrument::Didjeridoo,
        Instrument::Flute,
        Instrument::Guitar,
        Instrument::Harp,
        Instrument::Hat,
        Instrument::IronXylophone,
        Instrument::Pling,
        Instrument::Snare,
        Instrument::Xylophone
    ]);

    pub PROP_RAIL_SHAPE: enum<RailShape>("shape", RAIL_SHAPE, [
        RailShape::EastWest,
        RailShape::NorthEast,
        RailShape::NorthSouth,
        RailShape::NorthWest,
        RailShape::SouthEast,
        RailShape::SouthWest,
        RailShape::AscendingEast,
        RailShape::AscendingNorth,
        RailShape::AscendingSouth,
        RailShape::AscendingWest
    ]);

    pub PROP_RAIL_SHAPE_SPECIAL: enum<RailShape>("shape", RAIL_SHAPE_SPECIAL, [
        RailShape::EastWest,
        RailShape::NorthSouth,
        RailShape::AscendingEast,
        RailShape::AscendingNorth,
        RailShape::AscendingSouth,
        RailShape::AscendingWest
    ]);

    pub PROP_COMPARATOR_MODE: enum<ComparatorMode>("mode", COMPARATOR_MODE, [
        ComparatorMode::Compare,
        ComparatorMode::Subtract
    ]);

    pub PROP_WOOD_TYPE: enum<WoodType>("wood_type", WOOD_TYPE, [
        WoodType::Oak,
        WoodType::Spruce,
        WoodType::Birch,
        WoodType::Jungle,
        WoodType::Acacia,
        WoodType::DarkOak,
        WoodType::Crimson,
        WoodType::Warped
    ]);

    pub PROP_NETHER_WOOD_TYPE: enum<NetherWoodType>("wood_type", NETHER_WOOD_TYPE, [
        NetherWoodType::Crimson,
        NetherWoodType::Warped
    ]);

    pub PROP_REDSTONE_WIRE_EAST: enum<RedstoneWireMode>("east", REDSTONE_WIRE_MODE);
    pub PROP_REDSTONE_WIRE_NORTH: enum<RedstoneWireMode>("north", REDSTONE_WIRE_MODE);
    pub PROP_REDSTONE_WIRE_SOUTH: enum<RedstoneWireMode>("south", REDSTONE_WIRE_MODE);
    pub PROP_REDSTONE_WIRE_WEST: enum<RedstoneWireMode>("west", REDSTONE_WIRE_MODE);

    pub PROP_CHEST_TYPE: enum<ChestType>("type", CHEST_TYPE, [
        ChestType::Single,
        ChestType::Left,
        ChestType::Right
    ]);

    pub PROP_STAIRS_SHAPE: enum<StairsShape>("shape", STAIRS_SHAPE, [
        StairsShape::Straight,
        StairsShape::InnerLeft,
        StairsShape::InnerRight,
        StairsShape::OuterLeft,
        StairsShape::OuterRight
    ]);

}


// TODO:
//  - Banners
//  - Bubble Column
//  - Buttons
//  [OK] Chests
//  - Command Blocks
//  [OK] Doors
//  [OK] Fences
//  [OK] Fence Gates
//  - Glass Panes
//  - Glazed Terracotta
//  - Jigsaw Block
//  - Large Flowers
//  - Fluids
//  [OK] Logs
//  - Mob heads
//  [OK] Mushroom Blocks
//  - Wooden Pressure Plates
//  [OK] Saplings
//  - Shulker Boxes
//  [OK] Signs
//  - Slabs
//  [OK] Stairs
//  - Structure Blocks / Void
//  - Tall Grass / Tall Plants / Large Fern / Seagrass
//  [OK] Trapdoors
//  - Walls
//  [OK] Wood


// Same order as defined in MC code
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
    PLANKS "planks"                 [PROP_WOOD_TYPE], // Merged
    SAPLING "sapling"               [PROP_WOOD_TYPE], // Merged
    BEDROCK "bedrock",
    // Here: WATER/LAVA
    SAND "sand",
    RED_SAND "red_sand",
    GRAVEL "gravel",
    GOLD_ORE "gold_ore",
    IRON_ORE "iron_ore",
    COAL_ORE "coal_ore",
    NETHER_GOLD_ORE "nether_gold_ore",
    LOG "log"                       [PROP_WOOD_TYPE], // Merged
    STRIPPED_LOG "stripped_log"     [PROP_WOOD_TYPE], // Merged
    WOOD "wood"                     [PROP_WOOD_TYPE], // Merged
    STRIPPED_WOOD "stripped_wood"   [PROP_WOOD_TYPE], // Merged
    LEAVES "leaves"                 [PROP_WOOD_TYPE], // Merged
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
    PISTON_HEAD "piston_head"       [PROP_FACING_ALL, PROP_STICKY, PROP_SHORT],
    COBWEB "cobweb",
    GRASS "grass",
    FERN "fern",
    DEAD_BUSH "dead_bush",
    SEAGRASS "seagrass",
    TALL_SEAGRASS "tall_seagrass"   [PROP_DOUBLE_BLOCK_HALF],
    WOOL "wool"                     [PROP_COLOR],
    // Here: MOVING_PISTON
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
    FIRE "fire"                     [PROP_AGE_16, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST, PROP_UP],
    SOUL_FIRE "fire"                [PROP_AGE_16, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST, PROP_UP],
    SPAWNER "spawner",
    WOODEN_STAIRS "wooden_stairs"   [PROP_WOOD_TYPE, PROP_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED], // Merged
    CHEST "chest"                   [PROP_FACING, PROP_CHEST_TYPE, PROP_WATERLOGGED],
    REDSTONE_WIRE "redstone_wire"   [PROP_REDSTONE_POWER, PROP_REDSTONE_WIRE_EAST, PROP_REDSTONE_WIRE_NORTH, PROP_REDSTONE_WIRE_SOUTH, PROP_REDSTONE_WIRE_WEST],
    DIAMOND_ORE "diamond_ore",
    DIAMOND_BLOCK "diamond_block",
    CRAFTING_TABLE "crafting_table",
    WHEAT "wheat"                   [PROP_AGE_8],
    FARMLAND "farmland"             [PROP_FARMLAND_MOISTURE],
    FURNACE "furnace"               [PROP_FACING, PROP_LIT],
    SIGN "sign"                     [PROP_WOOD_TYPE, PROP_ROTATION, PROP_WATERLOGGED],
    WOODEN_DOOR "wooden_door"       [PROP_WOOD_TYPE, PROP_DOUBLE_BLOCK_HALF, PROP_FACING, PROP_OPEN, PROP_DOOR_HINGE, PROP_POWERED], // Merged
    LADDER "ladder"                 [PROP_FACING, PROP_WATERLOGGED],
    RAIL "rail"                     [PROP_RAIL_SHAPE],
    COBBLESTONE_STAIRS "cobblestone_stairs" [PROP_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED],
    WOODEN_WALL_SIGN "wooden_wall_sign"     [PROP_WOOD_TYPE, PROP_FACING, PROP_WATERLOGGED], // Merged
    LEVER "lever"                   [PROP_FACE, PROP_FACING, PROP_POWERED],
    STONE_PRESSURE_PLATE "stone_pressure_plate" [PROP_POWERED],
    IRON_DOOR "iron_door"           [PROP_WOOD_TYPE, PROP_DOUBLE_BLOCK_HALF, PROP_FACING, PROP_OPEN, PROP_DOOR_HINGE, PROP_POWERED],
    WOODEN_PRESSURE_PLATE "wooden_pressure_plate" [PROP_WOOD_TYPE, PROP_POWERED], // Merged
    REDSTONE_ORE "redstone_ore"     [PROP_LIT],
    REDSTONE_TORCH "redstone_torch" [PROP_LIT],
    REDSTONE_WALL_TORCH "redstone_wall_torch" [PROP_FACING, PROP_LIT],
    STONE_BUTTON "stone_button"     [PROP_FACING, PROP_POWERED, PROP_FACE],
    SNOW "snow"                     [PROP_SNOW_LAYERS],
    ICE "ice",
    SNOW_BLOCK "snow_block",
    CACTUS "cactus"                 [PROP_AGE_16],
    CLAY "clay",
    SUGAR_CANE "sugar_cane"         [PROP_AGE_16],
    JUKEBOX "jukebox"               [PROP_HAS_RECORD],
    WOODEN_FENCE "wooden_fence"     [PROP_WOOD_TYPE, PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED], // Merged
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
    WOODEN_TRAPDOOR "wooden_trapdoor" [PROP_WOOD_TYPE, PROP_FACING, PROP_OPEN, PROP_HALF, PROP_POWERED, PROP_WATERLOGGED], // Merged
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
    BROWN_MUSHROOM_BLOCK "brown_mushroom_block" [PROP_UP, PROP_DOWN, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST],
    RED_MUSHROOM_BLOCK "red_mushroom_block"     [PROP_UP, PROP_DOWN, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST],
    MUSHROOM_STEM "mushroom_stem"               [PROP_UP, PROP_DOWN, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST],
    IRON_BARS "iron_bars"           [PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED],
    CHAIN "chain"                   [PROP_WATERLOGGED, PROP_AXIS],
    GLASS_PANE "glass_pane"         [PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED],
    MELON "melon",
    ATTACHED_PUMPKIN_STEM "attached_pumpkin_stem"   [PROP_FACING],
    ATTACHED_MELON_STEM "attached_melon_stem"       [PROP_FACING],
    PUMPKIN_STEM "pumpkin_stem"                     [PROP_AGE_8],
    MELON_STEM "melon_stem"                         [PROP_AGE_8],
    VINE "vine"                     [PROP_UP, PROP_NORTH, PROP_EAST, PROP_SOUTH, PROP_WEST],
    WOODEN_FENCE_GATE "fence_gate"  [PROP_WOOD_TYPE, PROP_FACING, PROP_OPEN, PROP_POWERED, PROP_IN_WALL], // Merged
    BRICK_STAIRS "brick_stairs"                 [PROP_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED],
    STONE_BRICK_STAIRS "stone_brick_stairs"     [PROP_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED],
    MYCELIUM "mycelium"                         [PROP_SNOWY],
    LILY_PAD "lily_pad",
    NETHER_BRICKS "nether_bricks",
    NETHER_BRICK_FENCE "nether_brick_fence"     [PROP_NORTH, PROP_EAST, PROP_WEST, PROP_SOUTH, PROP_WATERLOGGED],
    NETHER_BRICK_STAIRS "nether_brick_stairs"   [PROP_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED],
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
    SANDSTONE_STAIRS "sandstone_stairs" [PROP_FACING, PROP_HALF, PROP_STAIRS_SHAPE, PROP_WATERLOGGED],
    EMERALD_ORE "emerald_ore",
    ENDER_CHEST "ender_chest"       [PROP_FACING, PROP_WATERLOGGED],
    TRIPWIRE_HOOK "tripwire_hook"   [PROP_ATTACHED, PROP_FACING, PROP_POWERED],
    TRIPWIRE "tripwire"             [PROP_ATTACHED, PROP_DISARMED, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_WEST, PROP_POWERED],
    EMERALD_BLOCK "emerald_block",
    // Here: COMMAND_BLOCK
    BEACON "beacon",
    COBBLESTONE_WALL "cobblestone_wall" [PROP_UP], // TODO: Finish, line 524 in Blocks.java


    BAMBOO "bamboo"                 [PROP_BAMBOO_AGE, PROP_BAMBOO_LEAVES, PROP_BAMBOO_STAGE],
    CARROTS "carrots"               [PROP_AGE_8],
    BEETROOTS "beetroots"           [PROP_AGE_4],
    ANVIL "anvil"                   [PROP_FACING],
    BARREL "barrel"                 [PROP_FACING_ALL, PROP_OPEN],
    BEEHIVE "beehive"               [PROP_FACING, PROP_HONEY_LEVEL],
    BEE_NEST "bee_nest"             [PROP_FACING, PROP_HONEY_LEVEL],
    BELL "bell"                     [PROP_BELL_ATTACHMENT, PROP_FACING, PROP_POWERED],
    BLAST_FURNACE "blast_furnace"   [PROP_FACING, PROP_LIT],
    BONE_BLOCK "bone_block"         [PROP_AXIS],
    CAMPFIRE "campfire"             [PROP_FACING, PROP_LIT, PROP_SIGNAL_FIRE, PROP_WATERLOGGED],
    CHORUS_FLOWER "chorus_flower"   [PROP_AGE_6],
    CHORUS_PLANT "chorus_plant"     [PROP_DOWN, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_UP, PROP_WEST],
    COMPOSTER "composter"           [PROP_COMPOSTER_LEVEL],
    CONDUIT "conduit"               [PROP_WATERLOGGED],
    DAYLIGHT_DETECTOR "daylight_detector" [PROP_INVERTED, PROP_REDSTONE_POWER],
    DROPPER "dropper"               [PROP_FACING_ALL, PROP_TRIGGERED],
    END_ROD "end_rod"               [PROP_FACING_ALL],
    FROSTED_ICE "frosted_ice"       [PROP_AGE_4],
    GRINDSTONE "grindstone"         [PROP_FACE, PROP_FACING],
    HAY_BLOCK "hay_block"           [PROP_AXIS],
    HOPPER "hopper"                 [PROP_ENABLED, PROP_FACING],
    KELP "kelp"                     [PROP_AGE_26],
    LANTERN "lantern"               [PROP_HANGING],
    LECTERN "lectern"               [PROP_FACING, PROP_HAS_BOOK, PROP_POWERED],
    LOOM "loom"                     [PROP_FACING],
    OBSERVER "observer"             [PROP_FACING, PROP_POWERED],
    POTATOES "potatoes"             [PROP_AGE_8],
    POLISHED_BLACKSTONE_PRESSURE_PLATE "polished_blackstone_pressure_plate" [PROP_POWERED],
    LIGHT_WEIGHTED_PRESSURE_PLATE "light_weighted_pressure_plate" [PROP_REDSTONE_POWER],
    HEAVY_WEIGHTED_PRESSURE_PLATE "heavy_weighted_pressure_plate" [PROP_REDSTONE_POWER],
    PURPUR_PILLAR "purpur_pillar"   [PROP_AXIS],
    QUARTZ_PILLAR "quartz_pillar"   [PROP_AXIS],
    ACTIVATOR_RAIL "activator_rail" [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED],
    COMPARATOR "comparator"         [PROP_FACING, PROP_COMPARATOR_MODE, PROP_POWERED],
    RESPAWN_ANCHOR "respawn_anchor" [PROP_CHARGES],
    SCAFFOLDING "scaffolding"       [PROP_BOTTOM, PROP_SCAFFOLDING_DISTANCE, PROP_WATERLOGGED],
    SEA_PICKLE "sea_pickle"         [PROP_PICKLES, PROP_WATERLOGGED],
    SMOKER "smoker"                 [PROP_FACING, PROP_LIT],
    STONECUTTER "stonecutter"       [PROP_FACING],
    SWEET_BERRY_BUSH "sweet_berry_bush" [PROP_AGE_4],
    TURTLE_EGG "turtle_egg"         [PROP_EGGS, PROP_HATCH],
    WATER "water"                   [PROP_LIQUID_FALLING],
    FLOWING_WATER "flowing_water"   [PROP_LIQUID_FALLING, PROP_LIQUID_LEVEL]
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
pub enum NetherWoodType {
    Crimson,
    Warped
}

impl_enum_serializable!(NetherWoodType {
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
