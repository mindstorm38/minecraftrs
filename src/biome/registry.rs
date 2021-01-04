use crate::biome::Biome;
use crate::version::Version;
use crate::res::Registry;
use derive_more::{Deref, DerefMut};


/// For registering biomes.
#[derive(Deref, DerefMut)]
pub struct BiomeRegistry(pub Registry<u8, Biome>);


#[allow(non_snake_case)]
pub mod def {

    /// Define Biome.
    macro_rules! db {
        ($mod_id:ident, $name:literal, $id:expr) => {
            pub mod $mod_id {
                pub const NAME: &'static str = $name;
                pub const ID: u8 = $id;
            }
        };
    }

    db!(OCEAN, "ocean", 0);
    db!(PLAINS, "plains", 1);
    db!(DESERT, "desert", 2);
    db!(EXTREME_HILLS, "extreme_hills", 3);
    db!(FOREST, "forest", 4);
    db!(TAIGA, "taiga", 5);
    db!(SWAMPLAND, "swampland", 6);
    db!(RIVER, "river", 7);

    db!(FROZEN_OCEAN, "frozen_ocean", 10);
    db!(FROZEN_RIVER, "frozen_river", 11);
    db!(ICE_PLAINS, "ice_plains", 12);
    db!(ICE_MOUNTAINS, "ice_mountains", 13);

    db!(MUSHROOM_ISLAND, "mushroom_island", 14);
    db!(MUSHROOM_ISLAND_SHORE, "mushroom_island_shore", 15);

    db!(BEACH, "beach", 16);

    db!(DESERT_HILLS, "desert_hills", 17);
    db!(FOREST_HILLS, "forest_hills", 18);
    db!(TAIGA_HILLS, "taiga_hills", 19);
    db!(EXTREME_HILLS_EDGE, "extreme_hills_edge", 20);

    db!(JUNGLE, "jungle", 21);
    db!(JUNGLE_HILLS, "jungle_hills", 22);

}


macro_rules! biome {
    ($mod_id:ident) => (Biome::new(def::$mod_id::NAME, def::$mod_id::ID));
}


impl From<Version> for BiomeRegistry {

    fn from(v: Version) -> Self {

        let mut reg = Registry::new();

        reg.register(biome!(OCEAN).set_height_range(-1.0, 0.4));
        reg.register(biome!(PLAINS).set_temp_rainfall(0.8, 0.4));
        reg.register(biome!(DESERT).set_temp_rainfall(2.0, 0.0).set_height_range(0.1, 0.2));
        reg.register(biome!(EXTREME_HILLS).set_temp_rainfall(0.2, 0.3).set_height_range(0.2, 1.3));
        reg.register(biome!(FOREST).set_temp_rainfall(0.7, 0.8));
        reg.register(biome!(TAIGA).set_temp_rainfall(0.05, 0.8).set_height_range(0.1, 0.4));
        reg.register(biome!(SWAMPLAND).set_temp_rainfall(0.8, 0.9).set_height_range(-0.2, 0.1));
        reg.register(biome!(RIVER).set_height_range(-0.5, 0.0));

        reg.register(biome!(FROZEN_OCEAN).set_temp_rainfall(0.0, 0.5).set_height_range(-1.0, 0.5));
        reg.register(biome!(FROZEN_RIVER).set_temp_rainfall(0.0, 0.5).set_height_range(-0.5, 0.0));
        reg.register(biome!(ICE_PLAINS).set_temp_rainfall(0.0, 0.5));
        reg.register(biome!(ICE_MOUNTAINS).set_temp_rainfall(0.0, 0.5));

        reg.register(biome!(MUSHROOM_ISLAND).set_temp_rainfall(0.9, 1.0).set_height_range(0.2, 1.0));
        reg.register(biome!(MUSHROOM_ISLAND_SHORE).set_temp_rainfall(0.9, 1.0).set_height_range(-1.0, 0.1));

        reg.register(biome!(BEACH).set_temp_rainfall(0.8, 0.4).set_height_range(0.0, 0.1));

        reg.register(biome!(DESERT_HILLS).set_temp_rainfall(2.0, 0.0).set_height_range(0.2, 0.7));
        reg.register(biome!(FOREST_HILLS).set_temp_rainfall(0.7, 0.8).set_height_range(0.2, 0.6));
        reg.register(biome!(TAIGA_HILLS).set_temp_rainfall(0.05, 0.8).set_height_range(0.2, 0.7));
        reg.register(biome!(EXTREME_HILLS_EDGE).set_temp_rainfall(0.2, 0.3).set_height_range(0.2, 0.8));

        if v >= Version::RELEASE_1_2 {
            reg.register(biome!(JUNGLE).set_temp_rainfall(1.2, 0.9).set_height_range(0.2, 0.4));
            reg.register(biome!(JUNGLE_HILLS).set_temp_rainfall(1.2, 0.9).set_height_range(1.8, 0.2));
        }

        BiomeRegistry(reg)

    }

}