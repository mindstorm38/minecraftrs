//! # Generator for release 1.2
//! A module implementing world generation for Minecraft 1.2.5.
//!
//! ## Known MC issues
//! To be as exact as possible, we also need to implement known issues of Minecraft 1.2, these
//! issues that are well known are that we discovered are listed below:
//!
//! ### Heightmap miscalculation
//! Right after world gen, the whole heightmap of the chunk is refreshed, however the algorithm
//! is wrong in this version for blocks on the max Y coordinate of a "top filled sub chunk" (this
//! mean that there is no more initialized sub chunk above it).
//!
//! For example, if in a chunk the top most filled chunk is at Y=3 (chunk coords), and if the
//! block at Y=63 is solid, we expect the heightmap to be set to 64, but the game set 63. This
//! can be even worse if there is a caveat below this solid block, in this case the height will
//! be even lower.
//!
//! This issue impact `MOTION_BLOCKING` heightmaps but not the `OCEAN_FLOOR` ones
//!
//! ### Non-deterministic big trees
//! The big tree feature is determined by the first big tree generated. There is one big tree
//! feature instance for each biome, so each biome have a different big tree instance that has
//! a different base height depending on the first generate big tree for this biome.

use std::num::Wrapping;
use std::sync::Arc;

use once_cell::sync::Lazy;

use mc_core::heightmap::HeightmapType;
use mc_core::world::chunk::Chunk;
use mc_core::rand::JavaRandom;
use mc_core::biome::Biome;
use mc_core::perf;
use mc_core::util::Rect;
use mc_core::world::source::ProtoChunk;

use mc_vanilla::heightmap::*;
use mc_vanilla::biome::*;
use mc_vanilla::block::*;

use crate::noise::{PerlinNoiseOctaves, NoiseCube, NoiseRect};
use crate::layer::{LayerBuilder, BoxLayer, Layer};
use crate::layer::zoom::VoronoiLayer;

use crate::structure::ravine::RavineStructure;
use crate::structure::cave::CaveStructure;
use crate::structure::Structure;

use crate::feature::{FeatureChain, Feature};
use crate::feature::distrib::{Distrib, HeightmapDistrib, LavaLakeDistrib};
use crate::feature::vein::{WaterCircleFeature, VeinFeature};
use crate::feature::branch::RepeatCount;
use crate::feature::dungeon::DungeonFeature;
use crate::feature::tree::{TreeFeature, BigTreeFeature};
use crate::feature::lake::LakeFeature;
use crate::view::LevelView;

use super::legacy::{GeneratorProvider, FeatureGenerator, TerrainGenerator, LegacyProtoChunk, QuadLevelView};
use super::biome::{BiomePropertyMap, BiomeProperty};


/// Base implementation of `GeneratorProvider` for release 1.2 generation.
pub struct R102Provider {
    shared: Arc<Shared>
}

impl R102Provider {

    /// Construct a new provider with the given seed.
    pub fn new(seed: i64) -> Self {
        let mut rand = JavaRandom::new(seed);
        Self {
            shared: Arc::new(Shared {
                seed,
                noise1: PerlinNoiseOctaves::new(&mut rand),
                noise2: PerlinNoiseOctaves::new(&mut rand),
                noise3: PerlinNoiseOctaves::new(&mut rand),
                noise_surface: PerlinNoiseOctaves::new(&mut rand),
                noise4: PerlinNoiseOctaves::new(&mut rand),
                noise5: PerlinNoiseOctaves::new(&mut rand),
            })
        }
    }

}

impl GeneratorProvider for R102Provider {

    type Terrain = R102TerrainGenerator;
    type Feature = R102FeatureGenerator;

    fn build_terrain(&self) -> Self::Terrain {
        R102TerrainGenerator::new(Arc::clone(&self.shared))
    }

    fn build_feature(&self) -> Self::Feature {
        R102FeatureGenerator::new(Arc::clone(&self.shared))
    }

}

/// Terrain generator for release 1.2
pub struct R102TerrainGenerator {
    shared: Arc<Shared>,
    noise1_cache: NoiseCube,
    noise2_cache: NoiseCube,
    noise3_cache: NoiseCube,
    noise4_cache: NoiseRect,
    noise5_cache: NoiseRect,
    noise_surface_cache: NoiseCube,
    noise_field: NoiseCube,
    layer_voronoi: VoronoiLayer<BoxLayer<&'static Biome>>,
}

impl TerrainGenerator for R102TerrainGenerator {
    type Chunk = LegacyProtoChunk;
    fn generate(&mut self, mut chunk: ProtoChunk) -> Self::Chunk {

        perf::push("r102_gen_terrain");

        const X_MUL: Wrapping<i64> = Wrapping(0x4f9939f508);
        const Z_MUL: Wrapping<i64> = Wrapping(0x1ef1565bd5);

        let (cx, cz) = chunk.get_position();
        let mut rand = JavaRandom::new((Wrapping(cx as i64) * X_MUL + Wrapping(cz as i64) * Z_MUL).0);

        perf::push("init_biomes");
        let biomes = self.initialize_biomes(&mut *chunk);
        perf::pop_push("terrain");
        self.generate_terrain(&mut *chunk);
        perf::pop();

        let mut chunk = LegacyProtoChunk {
            inner: chunk,
            legacy_biomes: biomes
        };

        perf::push("surface");
        self.generate_surface(&mut chunk, &mut rand/*&mut *chunk, &mut rand, &biomes*/);
        perf::pop_push("structures");
        self.generate_structures(&mut chunk/*&mut *chunk, &biomes*/);
        perf::pop();

        perf::pop();

        chunk

    }
}

impl R102TerrainGenerator {

    fn new(shared: Arc<Shared>) -> Self {
        const WIDTH: usize = 5;
        const HEIGHT: usize = 17;
        Self {
            noise1_cache: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            noise2_cache: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            noise3_cache: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            noise4_cache: NoiseRect::new_default(WIDTH, WIDTH),
            noise5_cache: NoiseRect::new_default(WIDTH, WIDTH),
            noise_surface_cache: NoiseCube::new_default(16, 16, 1),
            noise_field: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            layer_voronoi: Self::new_layers(shared.seed),
            shared,
        }
    }

    fn new_layers(seed: i64) -> VoronoiLayer<BoxLayer<&'static Biome>> {

        let (
            river,
            biome
        ) = LayerBuilder::with_island(1)
            .then_zoom_fuzzy(2000)
            .then_add_island(1)
            .then_zoom_smart(2001)
            .then_add_island(2)
            .then_add_snow(2)
            .then_zoom_smart(2002)
            .then_add_island(3)
            .then_zoom_smart(2003)
            .then_add_island(4)
            .then_add_mushroom_island(5)
            .into_box()
            .into_shared_split();

        let river = river
            .then_init_river(100)
            .then_zoom_smart(1000)
            .then_zoom_smart(1001)
            .then_zoom_smart(1002)
            .then_zoom_smart(1003)
            .then_zoom_smart(1004)
            .then_zoom_smart(1005)
            .then_add_river()
            .then_smooth(1000)
            .into_box()
            .build();

        let biome = biome
            .then_biome(200, (1, 2)).unwrap()
            .then_zoom_smart(1000)
            .then_zoom_smart(1001)
            .then_hills(1000)
            .then_zoom_smart(1000)
            .then_add_island(3)
            .then_zoom_smart(1001)
            .then_shore()
            .then_biome_river(1000)
            .then_zoom_smart(1002)
            .then_zoom_smart(1003)
            .then_smooth(1000)
            .into_box()
            .build();

        let mut voronoi = LayerBuilder::with_biome_and_river_mix(biome, river)
            .into_box()
            .then_zoom_voronoi(10)
            .build();

        voronoi.seed(seed);
        voronoi

    }

    fn initialize_biomes(&mut self, chunk: &mut Chunk) -> Rect<&'static Biome> {
        let (cx, cz) = chunk.get_position();
        let biomes = self.layer_voronoi.next_grid(cx * 16, cz * 16, 16, 16);
        chunk.set_biomes_2d(&biomes).expect("The biome layer returned invalid biomes.");
        biomes
    }

    /// Generate base terrain and return the first stage chunk.
    fn generate_terrain(&mut self, chunk: &mut Chunk) {

        // Generate terrain only generate 8 sub-chunks in height,
        // the construction limit is 16 chunks in height.
        //
        // 17 noises values are used for the whole 8 sub-chunks,
        // ignoring the last noise layer, there are 2 noise layers
        // for each sub-chunk.
        //
        // Only 4 values are used for each x & z axis, then a single
        // noise point in the noise field represent 4*4*8 blocks.
        //
        // This function just apply linear interpolation between
        // noise points.

        let (cx, cz) = chunk.get_position();
        self.initialize_noise_field(cx, cz);

        // dx/dz/dy are the noise field coordinates
        for dx in 0..4 {
            for dz in 0..4 {
                for dy in 0..16 {

                    // These lines of code will be called 2 times for each sub-chunk (8 sub chunk in total).

                    // Pattern for variables: [n|ns]_<x><y><z>
                    // The prefix "n" stands for "noise [at]"
                    // The prefix "ns" stands for "noise step"
                    // Using "x", "y" or "z" as coordinate means that the value will vary according to
                    // the specified axis.
                    let mut n_0y0 = *self.noise_field.get(dx + 0, dy + 0, dz + 0);
                    let mut n_0y1 = *self.noise_field.get(dx + 0, dy + 0, dz + 1);
                    let mut n_1y0 = *self.noise_field.get(dx + 1, dy + 0, dz + 0);
                    let mut n_1y1 = *self.noise_field.get(dx + 1, dy + 0, dz + 1);

                    // println!("[{}/{}/{}] n_0y0={}, n_0y1={}, n_1y0={}, n_1y1={}", dx, dy, dz, n_0y0, n_0y1, n_1y0, n_1y1);

                    // Mul by 0.125 because it equals 1/8, 8 is the number of blocks in the half sub chunk.
                    let ns_010 = (*self.noise_field.get(dx + 0, dy + 1, dz + 0) - n_0y0) * 0.125;
                    let ns_011 = (*self.noise_field.get(dx + 0, dy + 1, dz + 1) - n_0y1) * 0.125;
                    let ns_110 = (*self.noise_field.get(dx + 1, dy + 1, dz + 0) - n_1y0) * 0.125;
                    let ns_111 = (*self.noise_field.get(dx + 1, dy + 1, dz + 1) - n_1y1) * 0.125;

                    // Get the current sub-chunk.
                    let sub_chunk = chunk.ensure_sub_chunk((dy >> 1) as i8/*, None*/).unwrap();

                    // Iterating over the 8 blocks in the half sub-chunk.
                    for half_sub_chunk_dy in 0..8 {

                        let mut n_xy0 = n_0y0;
                        let mut n_xy1 = n_0y1;

                        // Mul by 0.25 because it equals 1/4, 4 is the number of blocks for a single
                        // axis of a single noise point.
                        let ns_1y0 = (n_1y0 - n_0y0) * 0.25;
                        let ns_1y1 = (n_1y1 - n_0y1) * 0.25;

                        // Using "& 1" to alternate between 0*8 and 1*8 for each sub-chunk.
                        let block_real_y = dy * 8 + half_sub_chunk_dy;
                        let block_y = block_real_y & 15;

                        // Iterating over the 4*4 for each value of 'half_sub_chunk_dy'
                        for sub_block_dx in 0..4 {

                            let block_x = dx * 4 + sub_block_dx;

                            let mut n_xyz = n_xy0;
                            let ns_xy1 = (n_xy1 - n_xy0) * 0.25;

                            for sub_block_dz in 0..4 {

                                let block_z = dz * 4 + sub_block_dz;

                                let block_to_set = if n_xyz > 0.0 {
                                    Some(STONE.get_default_state())
                                } else if block_real_y < 63 {
                                    Some(WATER.get_default_state())
                                } else {
                                    None
                                };

                                if let Some(block) = block_to_set {
                                    sub_chunk.set_block(block_x as u8, block_y as u8, block_z as u8, block).unwrap();
                                }

                                // println!("[{:02}/{:02}/{:02}] noise: {}, block: {:?}", block_x, block_real_y, block_z, n_xyz, block_to_set);

                                n_xyz += ns_xy1;

                            }

                            n_xy0 += ns_1y0;
                            n_xy1 += ns_1y1;

                        }

                        n_0y0 += ns_010;
                        n_0y1 += ns_011;
                        n_1y0 += ns_110;
                        n_1y1 += ns_111;

                    }

                }
            }
        }

    }

    fn initialize_noise_field(&mut self, cx: i32, cz: i32) {

        let x = cx * 4;
        let y = 0;
        let z = cz * 4;

        let gen_biomes = self.layer_voronoi.parent
            .next_grid(cx * 4 - 2, cz * 4 - 2, 10, 10);

        const WIDTH_SCALE: f64 = 684.41200000000003;
        const HEIGHT_SCALE: f64 = 684.41200000000003;

        self.shared.noise4.generate_2d(&mut self.noise4_cache, x, z, 1.121, 1.121);
        self.shared.noise5.generate_2d(&mut self.noise5_cache, x, z, 200.0, 200.0);
        self.shared.noise3.generate_3d(&mut self.noise3_cache, x, y, z, WIDTH_SCALE / 80.0, HEIGHT_SCALE / 160.0, WIDTH_SCALE / 80.0);
        self.shared.noise1.generate_3d(&mut self.noise1_cache, x, y, z, WIDTH_SCALE, HEIGHT_SCALE, WIDTH_SCALE);
        self.shared.noise2.generate_3d(&mut self.noise2_cache, x, y, z, WIDTH_SCALE, HEIGHT_SCALE, WIDTH_SCALE);

        // dx/dz/dy are the position in the noise field

        for dx in 0..self.noise_field.x_size {
            for dz in 0..self.noise_field.z_size {

                let biome = *gen_biomes.get(dx + 2, dz + 2);

                let (
                    min_height,
                    _max_height
                ) = BIOMES_PROPERTIES.get_height(biome).unwrap();

                let mut average_max_height = 0.0;
                let mut average_min_height = 0.0;
                let mut total_weight = 0.0;

                for neighbour_dx in -2..=2 {
                    for neighbour_dz in -2..=2 {

                        let neighbour_biome = *gen_biomes.get(
                            dx + (neighbour_dx + 2) as usize,
                            dz + (neighbour_dz + 2) as usize
                        );

                        let (
                            neighbour_min_height,
                            neighbour_max_height
                        ) = BIOMES_PROPERTIES.get_height(neighbour_biome).unwrap();

                        let mut weight = 10.0 / ((neighbour_dx * neighbour_dx + neighbour_dz * neighbour_dz) as f32 + 0.2).sqrt();
                        weight /= neighbour_min_height + 2.0;

                        if neighbour_min_height > min_height {
                            weight /= 2.0;
                        }

                        average_max_height += neighbour_max_height * weight;
                        average_min_height += neighbour_min_height * weight;
                        total_weight += weight;

                    }
                }

                average_max_height = (average_max_height / total_weight) * 0.9 + 0.1;
                average_min_height = ((average_min_height / total_weight) * 4.0 - 1.0) / 8.0;


                let mut val = *self.noise5_cache.get(dx, dz) / 8000.0;

                if val < 0.0 {
                    val = -val * 0.29999999999999999;
                }

                val = val * 3.0 - 2.0;

                if val < 0.0 {
                    val /= 2.0;
                    if val < -1.0 {
                        val = -1.0
                    }
                    val /= 1.3999999999999999;
                    val /= 2.0;
                } else {
                    if val > 1.0 {
                        val = 1.0;
                    }
                    val /= 8.0;
                }

                //println!("Main Noise 5 {}/{} = {} (biome: {}, averageMaxHeight: {}, averageMinHeight: {})", dx, dz, val, biome.get_id(), average_max_height, average_min_height);

                let y_size = self.noise_field.y_size as f64;
                for dy in 0..self.noise_field.y_size {

                    let a = (average_min_height as f64 + val * 0.20000000000000001) * y_size / 16.0;
                    let b = y_size / 2.0 + a * 4.0;
                    let mut c;
                    let mut d = ((dy as f64 - b) * 12.0 * 128.0) / 128.0 / average_max_height as f64;

                    //println!(" => [{}] a: {}, b: {}, d: {}", dy, a, b, d);

                    if d < 0.0 {
                        d *= 4.0;
                    }

                    // println!("  y: {}, noise1: {}", dy, self.noise_main1.get_noise(dx, dy, dz));

                    let val1 = *self.noise1_cache.get(dx, dy, dz) / 512.0;
                    let val2 = *self.noise2_cache.get(dx, dy, dz) / 512.0;
                    let val3 = (*self.noise3_cache.get(dx, dy, dz) / 10.0 + 1.0) / 2.0;

                    if val3 < 0.0 {
                        c = val1;
                    } else if val3 > 1.0 {
                        c = val2;
                    } else {
                        c = val1 + (val2 - val1) * val3;
                    }

                    c -= d;

                    if dy > self.noise_field.y_size - 4 {
                        let e = ((dy - (self.noise_field.y_size - 4)) as f32 / 3.0) as f64;
                        c = c * (1.0 - e) + (e * -10.0);
                    }

                    //println!("Noise field {}/{}/{} = {} ({}/{}/{})", dx, dy, dz, c, val1, val2, val3);
                    self.noise_field.set(dx, dy, dz, c);

                }

            }
        }

    }

    fn generate_surface(&mut self, chunk: &mut LegacyProtoChunk, rand: &mut JavaRandom/*, biomes: &Rect<&'static Biome>*/) {

        let (cx, cz) = chunk.inner.get_position();

        let block_air = AIR.get_default_state();
        let block_stone = STONE.get_default_state();
        let block_bedrock = BEDROCK.get_default_state();
        let block_sand = SAND.get_default_state();
        let block_sandstone = SANDSTONE.get_default_state();

        perf::push("surface_noise");
        const SCALE: f64 = 0.03125 * 2.0;
        self.shared.noise_surface.generate_3d(&mut self.noise_surface_cache, cx * 16, cz * 16, 0, SCALE, SCALE, SCALE);
        perf::pop_push("loop_over_columns");
        for z in 0..16u8 {
            for x in 0..16u8 {

                perf::push("get_biome_prop");
                let biome = chunk.get_legacy_biome(x, z);
                let biome_prop = BIOMES_PROPERTIES.get(biome).unwrap();
                perf::pop();

                // x/z are inverted
                let noise_val = (*self.noise_surface_cache.get(x as usize, z as usize, 0) / 3.0 + 3.0 + rand.next_double() * 0.25) as i32;

                let biome_top_block = biome_prop.top_block;
                let biome_filler_block = biome_prop.filler_block;

                let mut top_block = biome_top_block;
                let mut filler_block = biome_filler_block;

                let mut depth = -1;

                perf::push("loop_in_column");
                for cy in (0..8).rev() {
                    if let Some(sub_chunk) = chunk.inner.get_sub_chunk_mut(cy) {
                        for y in (0..16u8).rev() {

                            let y_real = (cy as i32) * 16 + y as i32;

                            if y_real <= rand.next_int_bounded(5) {
                                sub_chunk.set_block(x, y, z, block_bedrock).unwrap();
                            } else {

                                let block = sub_chunk.get_block(x, y, z);

                                if block == block_air {
                                    depth = -1;
                                } else if block == block_stone {

                                    if depth == -1 {

                                        if noise_val <= 0 {
                                            // This block is used to generate places where there is no grass but
                                            // stone at the layer behind de surface.
                                            top_block = block_air;
                                            filler_block = block_stone;
                                        } else if y_real >= 59 && y_real <= 64 {
                                            top_block = biome_top_block;
                                            filler_block = biome_filler_block;
                                        }

                                        if y_real < 63 && top_block == block_air {
                                            if biome_prop.temperature < 0.15 {
                                                top_block = ICE.get_default_state();
                                            } else {
                                                top_block = WATER.get_default_state();
                                            }
                                        }

                                        depth = noise_val;

                                        sub_chunk.set_block(x, y, z, if y_real >= 62 {
                                            top_block
                                        } else {
                                            filler_block
                                        }).unwrap();

                                    } else if depth > 0 {

                                        depth -= 1;
                                        sub_chunk.set_block(x, y, z, filler_block).unwrap();

                                        if depth == 0 && filler_block == block_sand {
                                            // This block is used to generate the sandstone behind the sand in
                                            // the desert.
                                            depth = rand.next_int_bounded(4);
                                            filler_block = block_sandstone;
                                        }

                                    }

                                }

                            }

                        }
                    }
                }

                perf::pop_push("recompute_heightmap_column");
                chunk.inner.recompute_heightmap_column(x, z);
                perf::pop();

            }
        }
        perf::pop();

    }

    fn generate_structures(&mut self, chunk: &mut LegacyProtoChunk/*, biomes: &Rect<&'static Biome>*/) {
        CaveStructure::new(&*BIOMES_PROPERTIES).generate_in(self.shared.seed, chunk, 8);
        RavineStructure::new(&*BIOMES_PROPERTIES).generate_in(self.shared.seed, chunk, 8);
    }

}

pub struct R102FeatureGenerator {
    shared: Arc<Shared>
}

impl FeatureGenerator for R102FeatureGenerator {
    type Chunk = LegacyProtoChunk;
    fn decorate(&mut self, mut level: QuadLevelView<Self::Chunk>, cx: i32, cz: i32, x: i32, z: i32) {

        let mut rand = JavaRandom::new(self.shared.seed);
        let a = Wrapping(rand.next_long() / 2 * 2 + 1);
        let b = Wrapping(rand.next_long() / 2 * 2 + 1);
        rand.set_seed((Wrapping(cx as i64) * a + Wrapping(cz as i64) * b).0 ^ self.shared.seed);

        /*{  // Debug biomes
            for dx in x..(x + 16) {
                for dz in z..(z + 16) {
                    let biome = level.get_biome_at(dx, 0, dz).unwrap();
                    let block = if biome == &RIVER {
                        &DEEPSLATE_LAPIS_ORE
                    } /*else if biome == &FOREST {
                        &DEEPSLATE_EMERALD_ORE
                    } else if biome == &PLAINS {
                        &DEEPSLATE_IRON_ORE
                    }*/ else {
                        &AIR
                    };
                    let height = level.get_heightmap_column_at(&WORLD_SURFACE, dx, dz).unwrap();
                    level.set_block_at(dx, height, dz, block.get_default_state()).unwrap();
                }
            }
        }*/

        /*{  // Debug feature chunks pos and biome
            let height = level.get_heightmap_column_at(&OCEAN_FLOOR, x + 8, z + 8).unwrap();
            level.set_block_at(x + 8, height - 1, z + 8, EMERALD_BLOCK.get_default_state()).unwrap();
            let height = level.get_heightmap_column_at(&OCEAN_FLOOR, x, z).unwrap();
            level.set_block_at(x, height - 1, z, EMERALD_ORE.get_default_state()).unwrap();
            level.set_block_at(x + 1, height - 1, z, EMERALD_ORE.get_default_state()).unwrap();
            level.set_block_at(x, height - 1, z + 1, EMERALD_ORE.get_default_state()).unwrap();
        }*/

        let biome = level.get_biome_at(x + 8, 0, z + 8).unwrap();
        let biome_prop = BIOMES_PROPERTIES.get(biome).unwrap();
        biome_prop.features.generate(&mut level, &mut rand, x, 0, z);

    }
}

impl R102FeatureGenerator {
    fn new(shared: Arc<Shared>) -> Self {
        Self {
            shared
        }
    }
}


/// Internal shared structure among terrain and feature generators.
struct Shared {
    seed: i64,
    noise1: PerlinNoiseOctaves<16>,
    noise2: PerlinNoiseOctaves<16>,
    noise3: PerlinNoiseOctaves<8>,
    noise4: PerlinNoiseOctaves<10>,
    noise5: PerlinNoiseOctaves<16>,
    noise_surface: PerlinNoiseOctaves<4>,
}

static BIOMES_PROPERTIES: Lazy<BiomePropertyMap> = Lazy::new(|| {

    struct BiomeConfig {
        sand_count_1: u16,
        sand_count_2: u16,
        clay_count: u16,
        tree_count: Option<u16>,
        big_mushroom_count: u16,
        flower_count: u16,
        grass_count: u16,
        dead_bush_count: u16,
        lily_pad_count: u16,
        mushroom_count: u16,
        sugar_cane_count: u16,
        cactus_count: u16,
        tree_feature_type: TreeFeatureType
    }

    enum TreeFeatureType {
        Default,
        Forest,
        Jungle,
        Swamp,
        Taiga
    }

    impl BiomeConfig {

        pub fn new() -> Self {
            Self {
                sand_count_1: 3,
                sand_count_2: 1,
                clay_count: 1,
                tree_count: Some(0),
                big_mushroom_count: 0,
                flower_count: 2,
                grass_count: 1,
                dead_bush_count: 0,
                lily_pad_count: 0,
                mushroom_count: 0,
                sugar_cane_count: 0,
                cactus_count: 0,
                tree_feature_type: TreeFeatureType::Default
            }
        }

        pub fn with<F: FnOnce(&mut Self)>(cb: F) -> Self {
            let mut conf = Self::new();
            cb(&mut conf);
            conf
        }

        pub fn build(&self) -> BiomeProperty {
            BiomeProperty {
                min_height: 0.1,
                max_height: 0.3,
                temperature: 0.5,
                top_block: GRASS_BLOCK.get_default_state(),
                filler_block: DIRT.get_default_state(),
                features: {

                    let mut chain = FeatureChain::new();
                    chain.push(LakeFeature::new(WATER.get_default_state(), &BIOMES_PROPERTIES).distributed_uniform(0, 128).optional(4));
                    chain.push(LakeFeature::new(LAVA.get_default_state(), &BIOMES_PROPERTIES).distributed(LavaLakeDistrib).optional(8));
                    chain.push(DungeonFeature.distributed_uniform(0, 128).repeated(8));

                    chain.push(VeinFeature::new(DIRT.get_default_state(), 32).distributed_uniform(0, 128).repeated(20));
                    chain.push(VeinFeature::new(GRAVEL.get_default_state(), 32).distributed_uniform(0, 128).repeated(10));
                    chain.push(VeinFeature::new(COAL_ORE.get_default_state(), 16).distributed_uniform(0, 128).repeated(20));
                    chain.push(VeinFeature::new(IRON_ORE.get_default_state(), 8).distributed_uniform(0, 64).repeated(20));
                    chain.push(VeinFeature::new(GOLD_ORE.get_default_state(), 8).distributed_uniform(0, 32).repeated(2));
                    chain.push(VeinFeature::new(REDSTONE_ORE.get_default_state(), 7).distributed_uniform(0, 16).repeated(8));
                    chain.push(VeinFeature::new(DIAMOND_ORE.get_default_state(), 7).distributed_uniform(0, 16));
                    chain.push(VeinFeature::new(LAPIS_ORE.get_default_state(), 6).distributed_triangular(16, 16));

                    chain.push(WaterCircleFeature::new_sand(7).distributed(HeightmapDistrib::new(&OCEAN_FLOOR_WG)).repeated(self.sand_count_1));
                    chain.push(WaterCircleFeature::new_clay(4).distributed(HeightmapDistrib::new(&OCEAN_FLOOR_WG)).repeated(self.clay_count));
                    chain.push(WaterCircleFeature::new_sand(7).distributed(HeightmapDistrib::new(&OCEAN_FLOOR_WG)).repeated(self.sand_count_2));

                    if let Some(tree_count) = self.tree_count {

                        fn new_tree_feature<F: Feature>(tree_count: u16, feature: F) -> impl Feature {
                            // TODO: In real MC 1.2.5 the getHeight method is used and use the block opacity for the heightmap.
                            feature
                                .distributed(WrongHeightmapDistrib::new(&MOTION_BLOCKING))
                                .repeated(TreeRepeatCount(tree_count))
                        }

                        match self.tree_feature_type {
                            TreeFeatureType::Default => {
                                let oak_tree = TreeFeature::new_oak();
                                let big_tree = BigTreeFeature::new();
                                chain.push(new_tree_feature(tree_count, big_tree.optional_or(10, oak_tree)))
                            },
                            TreeFeatureType::Forest => {
                                let birch_tree = TreeFeature::new_forest_birch();
                                let oak_tree = TreeFeature::new_oak();
                                let big_tree = BigTreeFeature::new();
                                chain.push(new_tree_feature(tree_count, birch_tree.optional_or(5, big_tree.optional_or(10, oak_tree))));
                            },
                            TreeFeatureType::Jungle => todo!(),
                            TreeFeatureType::Swamp => {
                                chain.push(new_tree_feature(tree_count, TreeFeature::new_swamp()));
                            },
                            TreeFeatureType::Taiga => todo!(),
                        }

                    }

                    // chain.push(DebugChunkFeature);

                    chain

                }
            }
        }

    }

    let default_config = BiomeConfig::new();
    let plains_config = BiomeConfig::with(|c| c.tree_count = None);
    let desert_config = BiomeConfig::with(|c| c.tree_count = None);
    let forest_config = BiomeConfig::with(|c| {
        c.tree_count = Some(10);
        c.grass_count = 2;
        c.tree_feature_type = TreeFeatureType::Forest;
    });
    let taiga_config = BiomeConfig::with(|c| c.tree_count = Some(10));
    let swamp_config = BiomeConfig::with(|c| {
        c.tree_count = Some(2);
        c.tree_feature_type = TreeFeatureType::Swamp;
    });
    let beach_config = BiomeConfig::with(|c| c.tree_count = None);
    let jungle_config = BiomeConfig::with(|c| c.tree_count = Some(50));
    let mushroom_config = BiomeConfig::with(|c| {
        c.tree_count = None;
        c.flower_count = 0;
        c.grass_count = 0;
        c.mushroom_count = 1;
        c.big_mushroom_count = 1;
    });

    let mut map = BiomePropertyMap::new();
    map.insert(&OCEAN, default_config.build().height(-1.0, 0.4));
    map.insert(&PLAINS, plains_config.build().height(0.1, 0.3).temp(0.8));
    map.insert(&DESERT, desert_config.build().height(0.1, 0.2).temp(2.0).blocks(&SAND, &SAND));
    map.insert(&MOUNTAINS, default_config.build().height(0.2, 1.3).temp(0.2));
    map.insert(&FOREST, forest_config.build().temp(0.7));
    map.insert(&TAIGA, taiga_config.build().height(0.1, 0.4).temp(0.05));
    map.insert(&SWAMP, swamp_config.build().height(-0.2, 0.1).temp(0.8));
    map.insert(&RIVER, default_config.build().height(-0.5, 0.0));
    map.insert(&FROZEN_OCEAN, default_config.build().height(-1.0, 0.5).temp(0.0));
    map.insert(&FROZEN_RIVER, default_config.build().height(-0.5, 0.0).temp(0.0));
    map.insert(&SNOWY_TUNDRA, plains_config.build().temp(0.0));
    map.insert(&SNOWY_MOUNTAINS, default_config.build().height(0.2, 1.2).temp(0.0));
    map.insert(&MUSHROOM_FIELDS, default_config.build().height(0.2, 1.0).temp(0.9).blocks(&MYCELIUM, &DIRT));
    map.insert(&MUSHROOM_FIELD_SHORE, default_config.build().height(-1.0, 0.1).temp(0.9).blocks(&MYCELIUM, &DIRT));
    map.insert(&BEACH, beach_config.build().height(0.0, 0.1).temp(0.8).blocks(&SAND, &SAND));
    map.insert(&DESERT_HILLS, desert_config.build().height(0.2, 0.7).temp(2.0).blocks(&SAND, &SAND));
    map.insert(&WOODED_HILLS, forest_config.build().height(0.2, 0.6).temp(0.7));
    map.insert(&TAIGA_HILLS, taiga_config.build().height(0.2, 0.7).temp(0.05));
    map.insert(&MOUNTAIN_EDGE, default_config.build().height(0.2, 0.8).temp(0.2));
    map.insert(&JUNGLE, jungle_config.build().height(0.2, 0.4).temp(1.2));
    map.insert(&JUNGLE_HILLS, jungle_config.build().height(1.8, 0.2).temp(1.2));
    map

});


/// A special distribution that is wrong to fit the issue described in the module doc.
/// This heightmap distribution returns normal height most of the time but returns one
/// unit lower only if the height is located at the highest Y of the highest non-null
/// sub chunk (the highest sub chunk that contains non-null block(s)).
struct WrongHeightmapDistrib {
    heightmap_type: &'static HeightmapType
}

impl WrongHeightmapDistrib {
    fn new(heightmap_type: &'static HeightmapType) -> Self {
        Self {
            heightmap_type
        }
    }
}

impl Distrib for WrongHeightmapDistrib {
    fn pick_pos(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let rz = z + rand.next_int_bounded(16);
        // SAFETY: Unwrapping because +16/+16 should be valid.
        let chunk = level.get_chunk_at(rx, rz).unwrap();
        let ry = chunk.get_heightmap_column_at(self.heightmap_type, rx, rz).unwrap();
        if ry & 15 == 0 {
            // If the height is currently set to above the top sub chunk block.
            let highest_ry = ((chunk.get_highest_non_null_sub_chunk() as i32) << 4) + 16;
            if highest_ry == ry {
                return Some((rx, ry - 1, rz));
            }
        }
        Some((rx, ry, rz))
    }
}


/// A count provider for repeating feature specific to trees.
pub struct TreeRepeatCount(pub u16);

impl RepeatCount for TreeRepeatCount {
    fn get_count(&self, rand: &mut JavaRandom) -> u16 {
        // (self.0 + (rand.next_int_bounded(10) == 0) as u16).min(4)
        self.0 + (rand.next_int_bounded(10) == 0) as u16
    }
}
