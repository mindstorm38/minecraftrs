//! This module must be used to find the biome numeric ID from its static object.
//! In fact, these IDs are not yet "legacy".
//!
//! Reference: https://minecraft.fandom.com/wiki/Biome/ID

use super::vanilla::VanillaBiomes;
use super::Biome;

use once_cell::sync::Lazy;
use std::collections::HashMap;


macro_rules! build_legacy {
    (
        $($biome_id:ident $id:literal),*
        $(,)?
    ) => {

        static CACHE: Lazy<HashMap<u32, u8>> = Lazy::new(|| {
            let mut cache = HashMap::new();
            $(cache.insert(VanillaBiomes.$biome_id.get_uid(), $id);)*
            cache
        });

        pub fn from_biome(biome: &Biome) -> Option<u8> {
            CACHE.get(&biome.get_uid()).copied()
        }

        pub fn from_id(id: u8) -> Option<&'static Biome> {
            match id {
                $($id => Some(&VanillaBiomes.$biome_id),)*
                _ => None
            }
        }

    };
}

build_legacy! {
    OCEAN 0,
    PLAINS 1,
    DESERT 2,
    MOUNTAINS 3,
    FOREST 4,
    TAIGA 5,
    SWAMP 6,
    RIVER 7,
    NETHER_WASTES 8,
    THE_END 9,
    FROZEN_OCEAN 10,
    FROZEN_RIVER 11,
    SNOWY_TUNDRA 12,
    SNOWY_MOUNTAINS 13,
    MUSHROOM_FIELDS 14,
    MUSHROOM_FIELD_SHORE 15,
    BEACH 16,
    DESERT_HILLS 17,
    WOODED_HILLS 18,
    TAIGA_HILLS 19,
    MOUNTAIN_EDGE 20,
    JUNGLE 21,
    JUNGLE_HILLS 22,
    JUNGLE_EDGE 23,
    DEEP_OCEAN 24,
    STONE_SHORE 25,
    SNOWY_BEACH 26,
    BIRCH_FOREST 27,
    BIRCH_FOREST_HILLS 28,
    DARK_FOREST 29,
    SNOWY_TAIGA 30,
    SNOWY_TAIGA_HILLS 31,
    GIANT_TREE_TAIGA 32,
    GIANT_TREE_TAIGA_HILLS 33,
    WOODED_MOUNTAINS 34,
    SAVANNA 35,
    SAVANNA_PLATEAU 36,
    BADLANDS 37,
    WOODED_BADLANDS_PLATEAU 38,
    BADLANDS_PLATEAU 39,
    SMALL_END_ISLANDS 40,
    END_MIDLANDS 41,
    END_HIGHLANDS 42,
    END_BARRENS 43,
    WARM_OCEAN 44,
    LUKEWARM_OCEAN 45,
    COLD_OCEAN 46,
    DEEP_WARM_OCEAN 47,
    DEEP_LUKEWARM_OCEAN 48,
    DEEP_COLD_OCEAN 49,
    DEEP_FROZEN_OCEAN 50,
    THE_VOID 127,
    SUNFLOWER_PLAINS 129,
    DESERT_LAKES 130,
    GRAVELLY_MOUNTAINS 131,
    FLOWER_FOREST 132,
    TAIGA_MOUNTAINS 133,
    SWAMP_HILLS 134,
    ICE_SPIKES 140,
    MODIFIED_JUNGLE 149,
    MODIFIED_JUNGLE_EDGE 151,
    TALL_BIRCH_FOREST 155,
    TALL_BIRCH_HILLS 156,
    DARK_FOREST_HILLS 157,
    SNOWY_TAIGA_MOUNTAINS 158,
    GIANT_SPRUCE_TAIGA 160,
    GIANT_SPRUCE_TAIGA_HILLS 161,
    MODIFIED_GRAVELLY_MOUNTAINS 162,
    SHATTERED_SAVANNA 163,
    SHATTERED_SAVANNA_PLATEAU 164,
    ERODED_BADLANDS 165,
    MODIFIED_WOODED_BADLANDS_PLATEAU 166,
    MODIFIED_BADLANDS_PLATEAU 167,
    BAMBOO_JUNGLE 168,
    BAMBOO_JUNGLE_HILLS 169,
    SOUL_SAND_VALLEY 170,
    CRIMSON_FOREST 171,
    WARPED_FOREST 172,
    BASALT_DELTAS 173,
    DRIPSTONE_CAVES 174,
    LUSH_CAVES 175,
}
