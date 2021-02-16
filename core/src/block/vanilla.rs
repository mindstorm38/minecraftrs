use crate::util::{Direction, Axis};
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

    pub PROP_HALF: enum<PlantHalf>("half", HALF, [
        PlantHalf::Lower, PlantHalf::Upper
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

    pub PROP_REDSTONE_WIRE_EAST: enum<RedstoneWireMode>("east", REDSTONE_WIRE_MODE);
    pub PROP_REDSTONE_WIRE_NORTH: enum<RedstoneWireMode>("north", REDSTONE_WIRE_MODE);
    pub PROP_REDSTONE_WIRE_SOUTH: enum<RedstoneWireMode>("south", REDSTONE_WIRE_MODE);
    pub PROP_REDSTONE_WIRE_WEST: enum<RedstoneWireMode>("west", REDSTONE_WIRE_MODE);

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
//  - Logs
//  - Mob heads
//  - Mushroom Blocks
//  - Wooden Pressure Plates
//  - Saplings
//  - Shulker Boxes
//  - Signs
//  - Slabs
//  - Stairs
//  - Structure Blocks / Void
//  - Tall Grass / Tall Plants / Large Fern / Seagrass
//  - Trapdoors
//  - Walls
//  - Wood


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
    JUKEBOX "jukebox"               [PROP_HAS_RECORD],
    KELP "kelp"                     [PROP_AGE_26],
    LADDER "ladder"                 [PROP_FACING, PROP_WATERLOGGED],
    LANTERN "lantern"               [PROP_HANGING],
    LECTERN "lectern"               [PROP_FACING, PROP_HAS_BOOK, PROP_POWERED],
    LEVER "lever"                   [PROP_FACE, PROP_FACING, PROP_POWERED],
    LOOM "loom"                     [PROP_FACING],
    MELON_STEM "melon_stem"         [PROP_AGE_8],
    PUMPKIN_STEM "pumpkin_stem"     [PROP_AGE_8],
    ATTACHED_MELON_STEM "attached_melon_stem" [PROP_FACING],
    ATTACHED_PUMPKIN_STEM "attached_pumpkin_stem" [PROP_FACING],
    NETHER_WART "nether_wart"       [PROP_AGE_4],
    NETHER_PORTAL "nether_portal"   [PROP_AXIS],
    NOTE_BLOCK "note_block"         [PROP_INSTRUMENT, PROP_NOTE, PROP_POWERED],
    OBSERVER "observer"             [PROP_FACING, PROP_POWERED],
    PISTON "piston"                 [PROP_STICKY, PROP_EXTENDED, PROP_FACING_ALL], // Merged the two piston type into one using a property "sticky".
    PISTON_HEAD "piston_head"       [PROP_STICKY, PROP_SHORT, PROP_FACING_ALL],
    POTATOES "potatoes"             [PROP_AGE_8],
    STONE_PRESSURE_PLATE "stone_pressure_plate" [PROP_POWERED],
    POLISHED_BLACKSTONE_PRESSURE_PLATE "polished_blackstone_pressure_plate" [PROP_POWERED],
    LIGHT_WEIGHTED_PRESSURE_PLATE "light_weighted_pressure_plate" [PROP_REDSTONE_POWER],
    HEAVY_WEIGHTED_PRESSURE_PLATE "heavy_weighted_pressure_plate" [PROP_REDSTONE_POWER],
    PURPUR_PILLAR "purpur_pillar"   [PROP_AXIS],
    QUARTZ_PILLAR "quartz_pillar"   [PROP_AXIS],
    RAIL "rail"                     [PROP_RAIL_SHAPE],
    POWERED_RAIL "powered_rail"     [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED],
    DETECTOR_RAIL "detector_rail"   [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED],
    ACTIVATOR_RAIL "activator_rail" [PROP_RAIL_SHAPE_SPECIAL, PROP_POWERED],
    COMPARATOR "comparator"         [PROP_FACING, PROP_COMPARATOR_MODE, PROP_POWERED],
    REDSTONE_WIRE "redstone_wire"   [PROP_REDSTONE_POWER, PROP_REDSTONE_WIRE_EAST, PROP_REDSTONE_WIRE_NORTH, PROP_REDSTONE_WIRE_SOUTH, PROP_REDSTONE_WIRE_WEST],
    REDSTONE_LAMP "redstone_lamp"   [PROP_LIT],
    REDSTONE_ORE "redstone_ore"     [PROP_LIT],
    REPEATER "repeater"             [PROP_REPEATER_DELAY, PROP_FACING, PROP_LOCKED, PROP_POWERED],
    REDSTONE_TORCH "redstone_torch" [PROP_LIT],
    REDSTONE_WALL_TORCH "redstone_wall_torch" [PROP_FACING, PROP_LIT],
    RESPAWN_ANCHOR "respawn_anchor" [PROP_CHARGES],
    SCAFFOLDING "scaffolding"       [PROP_BOTTOM, PROP_SCAFFOLDING_DISTANCE, PROP_WATERLOGGED],
    SEA_PICKLE "sea_pickle"         [PROP_PICKLES, PROP_WATERLOGGED],
    SMOKER "smoker"                 [PROP_FACING, PROP_LIT],
    SNOW "snow"                     [PROP_SNOW_LAYERS],
    STONECUTTER "stonecutter"       [PROP_FACING],
    SUGAR_CANE "sugar_cane"         [PROP_AGE_16],
    SWEET_BERRY_BUSH "sweet_berry_bush" [PROP_AGE_4],
    TNT "tnt"                       [PROP_UNSTABLE],
    TRIPWIRE "tripwire"             [PROP_ATTACHED, PROP_DISARMED, PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_WEST, PROP_POWERED],
    TRIPWIRE_HOOK "tripwire_hook"   [PROP_ATTACHED, PROP_FACING, PROP_POWERED],
    TURTLE_EGG "turtle_egg"         [PROP_EGGS, PROP_HATCH],
    VINE "vine"                     [PROP_EAST, PROP_NORTH, PROP_SOUTH, PROP_UP, PROP_WEST],
    WALL_TORCH "wall_torch"         [PROP_FACING],
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
pub enum PlantHalf {
    Lower,
    Upper
}

impl_enum_serializable!(PlantHalf {
    Lower: "lower",
    Upper: "upper"
});
