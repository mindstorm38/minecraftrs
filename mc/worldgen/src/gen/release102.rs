//!
//! Generator for release 1.2
//!

use std::collections::HashMap;
use std::num::Wrapping;
use std::sync::Arc;

use once_cell::sync::Lazy;

use mc_core::world::source::{LevelGenerator, ProtoChunk, ChunkInfo, LevelSourceError};
use mc_core::world::chunk::Chunk;
use mc_core::biome::{Biome, BiomeKey};
use mc_core::block::{BlockState, Block};
use mc_core::rand::JavaRandom;
use mc_core::util::Rect;

use mc_vanilla::biome::*;
use mc_vanilla::block::*;

use crate::noise::{PerlinNoiseOctaves, NoiseCube, NoiseRect};
use crate::layer::{LayerBuilder, BoxLayer, Layer};
use crate::layer::zoom::VoronoiLayer;


/// The LevelGenerator for Minecraft 1.2 release.
// #[derive(Clone)]
pub struct LevelGenRelease102 {

    shared: Arc<Shared>,
    rand: JavaRandom,

    noise1_cache: NoiseCube,
    noise2_cache: NoiseCube,
    noise3_cache: NoiseCube,
    noise4_cache: NoiseRect,
    noise5_cache: NoiseRect,
    noise_surface_cache: NoiseCube,

    noise_field: NoiseCube,
    layer_voronoi: VoronoiLayer<BoxLayer<&'static Biome>>

    //ravine_carver: Carver,
    //cave_carver: Carver

}

struct Shared {
    noise1: PerlinNoiseOctaves<16>,
    noise2: PerlinNoiseOctaves<16>,
    noise3: PerlinNoiseOctaves<8>,
    noise4: PerlinNoiseOctaves<10>,
    noise5: PerlinNoiseOctaves<16>,
    noise_surface: PerlinNoiseOctaves<4>,
}

impl LevelGenRelease102 {

    pub fn new(seed: i64) -> Self {

        const WIDTH: usize = 5;
        const HEIGHT: usize = 17;

        let mut rand = JavaRandom::new(seed);

        let shared = Arc::new(Shared {
            noise1: PerlinNoiseOctaves::new(&mut rand),
            noise2: PerlinNoiseOctaves::new(&mut rand),
            noise3: PerlinNoiseOctaves::new(&mut rand),
            noise_surface: PerlinNoiseOctaves::new(&mut rand),
            noise4: PerlinNoiseOctaves::new(&mut rand),
            noise5: PerlinNoiseOctaves::new(&mut rand),
        });

        Self {
            shared,
            noise1_cache: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            noise2_cache: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            noise3_cache: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            noise4_cache: NoiseRect::new_default(WIDTH, WIDTH),
            noise5_cache: NoiseRect::new_default(WIDTH, WIDTH),
            noise_surface_cache: NoiseCube::new_default(16, 16, 1),
            noise_field: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            layer_voronoi: Self::new_layers(seed),
            // ravine_carver: Carver::new_ravine(),
            // cave_carver: Carver::new_cave(),
            rand,
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
            .then_zoom_voronoi(100)
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

    fn generate_surface(&mut self, chunk: &mut Chunk, biomes: &Rect<&'static Biome>) {

        let (cx, cz) = chunk.get_position();

        let block_air = AIR.get_default_state();
        let block_stone = STONE.get_default_state();
        let block_bedrock = BEDROCK.get_default_state();
        let block_sand = SAND.get_default_state();
        let block_sandstone = SANDSTONE.get_default_state();

        const SCALE: f64 = 0.03125 * 2.0;
        self.shared.noise_surface.generate_3d(&mut self.noise_surface_cache, cx * 16, cz * 16, 0, SCALE, SCALE, SCALE);

        for z in 0..16u8 {
            for x in 0..16u8 {

                let biome = *biomes.get(x as usize, z as usize);
                let biome_prop = BIOMES_PROPERTIES.get(biome).unwrap();

                // x/z are inverted
                let noise_val = (*self.noise_surface_cache.get(x as usize, z as usize, 0) / 3.0 + 3.0 + self.rand.next_double() * 0.25) as i32;

                let biome_top_block = biome_prop.top_block;
                let biome_filler_block = biome_prop.filler_block;

                let mut top_block = biome_top_block;
                let mut filler_block = biome_filler_block;

                let mut depth = -1;

                for y in (0..128i32).rev() {

                    if y <= self.rand.next_int_bounded(5) {
                        chunk.set_block(x, y, z, block_bedrock).unwrap();
                    } else {

                        let block = chunk.get_block(x, y, z).unwrap();

                        if block == block_air {
                            depth = -1;
                        } else if block == block_stone {

                            if depth == -1 {

                                if noise_val <= 0 {
                                    // This block is used to generate places where there is no grass but
                                    // stone at the layer behind de surface.
                                    top_block = block_air;
                                    filler_block = block_stone;
                                } else if y >= 59 && y <= 64 {
                                    top_block = biome_top_block;
                                    filler_block = biome_filler_block;
                                }

                                if y < 63 && top_block == block_air {
                                    if biome_prop.temperature < 0.15 {
                                        top_block = ICE.get_default_state();
                                    } else {
                                        top_block = WATER.get_default_state();
                                    }
                                }

                                depth = noise_val;

                                chunk.set_block(x, y, z, if y >= 62 {
                                    top_block
                                } else {
                                    filler_block
                                }).unwrap();

                            }

                            if depth > 0 {

                                depth -= 1;
                                chunk.set_block(x, y, z, filler_block).unwrap();

                                if depth == 0 && filler_block == block_sand {
                                    // This block is used to generate the sandstone behind the sand in
                                    // the desert.
                                    depth = self.rand.next_int_bounded(4);
                                    filler_block = block_sandstone;
                                }

                            }

                        }

                    }

                }

            }
        }

    }

}

impl LevelGenerator for LevelGenRelease102 {

    fn generate(&mut self, info: ChunkInfo) -> Result<ProtoChunk, (LevelSourceError, ChunkInfo)> {

        const POS_LIMIT: i32 = 1_875_004;
        const X_MUL: Wrapping<i64> = Wrapping(0x4f9939f508);
        const Z_MUL: Wrapping<i64> = Wrapping(0x1ef1565bd5);

        if info.cx < -POS_LIMIT || info.cz < -POS_LIMIT || info.cx >= POS_LIMIT || info.cz >= POS_LIMIT {
            // In order to return position, we need a better LevelGenerator trait
            return Err((LevelSourceError::UnsupportedChunkPosition, info));
        }

        self.rand.set_seed((Wrapping(info.cx as i64) * X_MUL + Wrapping(info.cz as i64) * Z_MUL).0);

        let mut chunk = info.build_proto_chunk();
        let biomes = self.initialize_biomes(&mut *chunk);
        self.generate_terrain(&mut *chunk);
        self.generate_surface(&mut *chunk, &biomes);

        Ok(chunk)

    }

}


struct BiomePropertyMap {
    data: HashMap<BiomeKey, BiomeProperty>
}

struct BiomeProperty {
    min_height: f32,
    max_height: f32,
    temperature: f32,
    top_block: &'static BlockState,
    filler_block: &'static BlockState
}

struct BiomePropertyBuilder {
    biome: &'static Biome,
    inner: BiomeProperty,
    map: BiomePropertyMap,
}

impl BiomePropertyBuilder {

    pub fn height(mut self, min: f32, max: f32) -> Self {
        self.inner.min_height = min;
        self.inner.max_height = max;
        self
    }

    pub fn temp(mut self, temp: f32) -> Self {
        self.inner.temperature = temp;
        self
    }

    pub fn blocks(mut self, top: &'static Block, filler: &'static Block) -> Self {
        self.inner.top_block = top.get_default_state();
        self.inner.filler_block = filler.get_default_state();
        self
    }

    pub fn build(mut self) -> BiomePropertyMap {
        self.map.data.insert(self.biome.get_key(), self.inner);
        self.map
    }

}

impl BiomePropertyMap {

    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn insert(self, biome: &'static Biome) -> BiomePropertyBuilder {
        BiomePropertyBuilder {
            biome,
            inner: BiomeProperty {
                min_height: 0.1,
                max_height: 0.3,
                temperature: 0.5,
                top_block: GRASS.get_default_state(),
                filler_block: DIRT.get_default_state()
            },
            map: self
        }
    }

    pub fn get(&self, biome: &'static Biome) -> Option<&BiomeProperty> {
        self.data.get(&biome.get_key())
    }

    pub fn get_height(&self, biome: &'static Biome) -> Option<(f32, f32)> {
        self.get(biome).map(|prop| (prop.min_height, prop.max_height))
    }

}

static BIOMES_PROPERTIES: Lazy<BiomePropertyMap> = Lazy::new(|| {
    BiomePropertyMap::new()
        .insert(&OCEAN).height(-1.0, 0.4).build()
        .insert(&PLAINS).height(0.1, 0.3).temp(0.8).build()
        .insert(&DESERT).height(0.1, 0.2).temp(2.0).blocks(&SAND, &SANDSTONE).build()
        .insert(&MOUNTAINS).height(0.2, 1.3).temp(0.2).build()
        .insert(&FOREST).temp(0.7).build()
        .insert(&TAIGA).height(0.1, 0.4).temp(0.05).build()
        .insert(&SWAMP).height(-0.2, 0.1).temp(0.8).build()
        .insert(&RIVER).height(-0.5, 0.0).build()
        .insert(&FROZEN_OCEAN).height(-1.0, 0.5).temp(0.0).build()
        .insert(&FROZEN_RIVER).height(-0.5, 0.0).temp(0.0).build()
        .insert(&SNOWY_TUNDRA).temp(0.0).build()
        .insert(&SNOWY_MOUNTAINS).height(0.2, 1.2).temp(0.0).build()
        .insert(&MUSHROOM_FIELDS).height(0.2, 1.0).temp(0.9).blocks(&MYCELIUM, &DIRT).build()
        .insert(&MUSHROOM_FIELD_SHORE).height(-1.0, 0.1).temp(0.9).blocks(&MYCELIUM, &DIRT).build()
        .insert(&BEACH).height(0.0, 0.1).temp(0.8).blocks(&SAND, &SAND).build()
        .insert(&DESERT_HILLS).height(0.2, 0.7).temp(2.0).blocks(&SAND, &SANDSTONE).build()
        .insert(&WOODED_HILLS).height(0.2, 0.6).temp(0.7).build()
        .insert(&TAIGA_HILLS).height(0.2, 0.7).temp(0.05).build()
        .insert(&MOUNTAIN_EDGE).height(0.2, 0.8).temp(0.2).build()
        .insert(&JUNGLE).height(0.2, 0.4).temp(1.2).build()
        .insert(&JUNGLE_HILLS).height(1.8, 0.2).temp(1.2).build()
});






/*struct ChunkGeneratorInternal {

    world_info: Rc<WorldInfo>,
    rand: JavaRandom,

    voronoi_layer: Layer,

    noise_main1: FixedPerlinNoiseOctaves,
    noise_main2: FixedPerlinNoiseOctaves,
    noise_main3: FixedPerlinNoiseOctaves,
    noise_main4: FixedPerlinNoiseOctaves,
    noise_main5: FixedPerlinNoiseOctaves,
    noise_surface: FixedPerlinNoiseOctaves,

    noise_field: NoiseCube,

    ravine_carver: Carver,
    cave_carver: Carver

}

impl ChunkGeneratorInternal {

    fn new(world_info: Rc<WorldInfo>) -> Self {

        const WIDTH: usize = 5;
        const HEIGHT: usize = 17;

        let mut rand = JavaRandom::new(world_info.seed);FixedPerlinNoiseOctaves

        ChunkGeneratorInternal {
            voronoi_layer: Self::new_layers(world_info.seed),
            noise_main1: FixedPerlinNoiseOctaves::new(&mut rand, 16, WIDTH, HEIGHT, WIDTH),
            noise_main2: FixedPerlinNoiseOctaves::new(&mut rand, 16, WIDTH, HEIGHT, WIDTH),
            noise_main3: FixedPerlinNoiseOctaves::new(&mut rand, 8, WIDTH, HEIGHT, WIDTH),
            noise_surface: FixedPerlinNoiseOctaves::new(&mut rand, 4, 16, 16, 1),
            noise_main4: FixedPerlinNoiseOctaves::new(&mut rand, 10, WIDTH, 1, WIDTH),
            noise_main5: FixedPerlinNoiseOctaves::new(&mut rand, 16, WIDTH, 1, WIDTH),
            noise_field: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            ravine_carver: Carver::new_ravine(),
            cave_carver: Carver::new_cave(),
            rand,
            world_info
        }

    }

    fn new_layers(seed: i64) -> Layer {

        // Common layers
        let mut common = Layer::new_island(1);
        common = Layer::new_fuzzy_zoom(2000, common);
        common = Layer::new_add_island(1, common);
        common = Layer::new_zoom(2001, common);
        common = Layer::new_add_island(2, common);
        common = Layer::new_add_snow(2, common);
        common = Layer::new_zoom(2002, common);
        common = Layer::new_add_island(3, common);
        common = Layer::new_zoom(2003, common);
        common = Layer::new_add_island(4, common);
        common = Layer::new_add_mushroom_island(5, common);

        // River layers
        // Cloning all the common hierarchy
        let mut river = Layer::new_river_init(100, common.clone());
        river = Layer::new_zoom_multiple(1000, river, 6);
        river = Layer::new_river(1, river);
        river = Layer::new_smooth(1000, river);

        // Definitive biomes layers
        let mut biome = Layer::new_biome_1_2(200, common);
        biome = Layer::new_zoom_multiple(1000, biome, 2);
        biome = Layer::new_hills(1000, biome);
        for i in 0..4 {
            biome = Layer::new_zoom(1000 + i, biome);
            match i {
                0 => biome = Layer::new_add_island(3, biome),
                1 => {
                    biome = Layer::new_shore(1000, biome);
                    biome = Layer::new_biome_rivers(1000, biome);
                },
                _ => {}
            }
        }
        biome = Layer::new_smooth(1000, biome);

        let mixed = Layer::new_mix_biome_river(100, biome, river);
        let mut voronoi = Layer::new_voronoi(10, mixed);

        voronoi.init_world_seed(seed);
        voronoi

    }

    /// Entry point for chunk generation.
    fn generate_chunk(&mut self, cx: i32, cz: i32) -> Result<Chunk, ChunkError> {

        const POS_LIMIT: i32 = 1_875_004;
        const X_MUL: Wrapping<i64> = Wrapping(0x4f9939f508);
        const Z_MUL: Wrapping<i64> = Wrapping(0x1ef1565bd5);

        if cx < -POS_LIMIT || cz < -POS_LIMIT || cx >= POS_LIMIT || cz >= POS_LIMIT {
            return Err(ChunkError::IllegalPosition(cx, cz))
        }

        self.rand.set_seed((Wrapping(cx as i64) * X_MUL + Wrapping(cz as i64) * Z_MUL).0);

        let mut chunk = Chunk::new(Rc::clone(&self.world_info), cx, cz, 8);

        self.initialize_biomes(&mut chunk);
        self.generate_terrain(&mut chunk);
        self.generate_surface(&mut chunk);
        self.cave_carver.generate(&mut chunk);
        self.ravine_carver.generate(&mut chunk);

        Ok(chunk)

    }

    fn populate_chunk(&mut self, world: &mut WorldAccess, cx: i32, cz: i32) {



    }

    fn initialize_biomes(&mut self, chunk: &mut Chunk) {

        let (cx, cz) = chunk.get_position();
        let biomes = self.voronoi_layer.generate(cx * 16, cz * 16, 16, 16);

        for x in 0..16 {
            for z in 0..16 {
                chunk.set_biome_2d(x, z, self.world_info.biome_registry.0.expect_from_id(biomes.get(x, z).expect_biome()));
            }
        }

        chunk.set_biome_3d_auto();

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

        let stone_block = self.world_info.block_registry.0.expect_from_name("stone").get_id();
        let water_block = self.world_info.block_registry.0.expect_from_name("water").get_id();

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
                    let mut n_0y0 = self.noise_field.get(dx + 0, dy + 0, dz + 0);
                    let mut n_0y1 = self.noise_field.get(dx + 0, dy + 0, dz + 1);
                    let mut n_1y0 = self.noise_field.get(dx + 1, dy + 0, dz + 0);
                    let mut n_1y1 = self.noise_field.get(dx + 1, dy + 0, dz + 1);

                    // println!("[{}/{}/{}] n_0y0={}, n_0y1={}, n_1y0={}, n_1y1={}", dx, dy, dz, n_0y0, n_0y1, n_1y0, n_1y1);

                    // Mul by 0.125 because it equals 1/8, 8 is the number of blocks in the half sub chunk.
                    let ns_010 = (self.noise_field.get(dx + 0, dy + 1, dz + 0) - n_0y0) * 0.125;
                    let ns_011 = (self.noise_field.get(dx + 0, dy + 1, dz + 1) - n_0y1) * 0.125;
                    let ns_110 = (self.noise_field.get(dx + 1, dy + 1, dz + 0) - n_1y0) * 0.125;
                    let ns_111 = (self.noise_field.get(dx + 1, dy + 1, dz + 1) - n_1y1) * 0.125;

                    // Get the current sub-chunk.
                    let sub_chunk = chunk.get_sub_chunk_mut(dy >> 1);

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
                                    stone_block
                                } else if block_real_y < 63 {
                                    water_block
                                } else {
                                    0
                                };

                                // println!("[{:02}/{:02}/{:02}] noise: {}, block: {:?}", block_x, block_real_y, block_z, n_xyz, block_to_set);

                                n_xyz += ns_xy1;
                                sub_chunk.set_block_id(block_x, block_y, block_z, block_to_set);

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

        // Terrain biomes don't expect to be "voronoi-ed"
        let biome_layer = self.voronoi_layer.expect_parent();
        let biome_layer_data = biome_layer.generate(cx * 4 - 2, cz * 4 - 2, 10, 10);
        let biome_rect = build_biome_rect(biome_layer_data, &self.world_info.biome_registry);

        const WIDTH_SCALE: f64 = 684.41200000000003;
        const HEIGHT_SCALE: f64 = 684.41200000000003;

        self.noise_main4.generate(x, 10, z, 1.121, 1.0, 1.121);
        self.noise_main5.generate(x, 10, z, 200.0, 1.0, 200.0);
        self.noise_main3.generate(x, y, z, WIDTH_SCALE / 80.0, HEIGHT_SCALE / 160.0, WIDTH_SCALE / 80.0);
        self.noise_main1.generate(x, y, z, WIDTH_SCALE, HEIGHT_SCALE, WIDTH_SCALE);
        self.noise_main2.generate(x, y, z, WIDTH_SCALE, HEIGHT_SCALE, WIDTH_SCALE);

        // dx/dz/dy are the position in the noise field

        for dx in 0..self.noise_field.x_size {
            for dz in 0..self.noise_field.z_size {

                let biome = biome_rect.get(dx + 2, dz + 2);
                let mut average_max_height = 0.0;
                let mut average_min_height = 0.0;
                let mut total_weight = 0.0;

                for neighbour_dx in -2..=2 {
                    for neighbour_dz in -2..=2 {

                        let neighbour_biome = biome_rect.get(dx + (neighbour_dx + 2) as usize, dz + (neighbour_dz + 2) as usize);
                        let mut weight = 10.0 / ((neighbour_dx * neighbour_dx + neighbour_dz * neighbour_dz) as f32 + 0.2).sqrt();
                        weight /= neighbour_biome.get_min_height() + 2.0;

                        if neighbour_biome.get_min_height() > biome.get_min_height() {
                            weight /= 2.0;
                        }

                        average_max_height += neighbour_biome.get_max_height() * weight;
                        average_min_height += neighbour_biome.get_min_height() * weight;
                        total_weight += weight;

                    }
                }

                average_max_height = (average_max_height / total_weight) * 0.9 + 0.1;
                average_min_height = ((average_min_height / total_weight) * 4.0 - 1.0) / 8.0;

                let mut val = self.noise_main5.get_noise(dx, 0, dz) / 8000.0;

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

                    let val1 = self.noise_main1.get_noise(dx, dy, dz) / 512.0;
                    let val2 = self.noise_main2.get_noise(dx, dy, dz) / 512.0;
                    let val3 = (self.noise_main3.get_noise(dx, dy, dz) / 10.0 + 1.0) / 2.0;

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

    fn generate_surface(&mut self, chunk: &mut Chunk) {

        let (cx, cz) = chunk.get_position();

        let stone_block = self.world_info.block_registry.0.expect_from_name("stone").get_id();
        let bedrock_block = self.world_info.block_registry.0.expect_from_name("bedrock").get_id();
        let water_block = self.world_info.block_registry.0.expect_from_name("water").get_id();
        let ice_block = self.world_info.block_registry.0.expect_from_name("ice").get_id();
        let sand_block = self.world_info.block_registry.0.expect_from_name("sand").get_id();
        let sand_stone_block = self.world_info.block_registry.0.expect_from_name("sand_stone").get_id();

        const SCALE: f64 = 0.03125 * 2.0;
        self.noise_surface.generate(cx * 16, cz * 16, 0, SCALE, SCALE, SCALE);

        for z in 0..16 {
            for x in 0..16 {

                //let biome = biome_rect.get(x, z);
                let biome = chunk.get_biome_2d(x, z);
                let temp = biome.temperature;

                // x/z are inverted
                let noise_val = (self.noise_surface.get_noise(x, z, 0) / 3.0 + 3.0 + self.rand.next_double() * 0.25) as i32;

                let biome_top_block = self.world_info.block_registry.0.expect_from_name(biome.top_block).get_id();
                let biome_filler_block = self.world_info.block_registry.0.expect_from_name(biome.filler_block).get_id();

                let mut top_block = biome_top_block;
                let mut filler_block = biome_filler_block;

                let mut depth = -1;

                for y in (0..128).rev() {

                    if y <= self.rand.next_int_bounded(5) as usize {
                        chunk.set_block_id(x, y, z, bedrock_block);
                    } else {

                        let block = chunk.get_block_id(x, y, z);

                        if block == 0 {
                            depth = -1;
                        } else if block == stone_block {

                            if depth == -1 {

                                if noise_val <= 0 {
                                    // This block is used to generate places where there is no grass but
                                    // stone at the layer behind de surface.
                                    top_block = 0;
                                    filler_block = stone_block;
                                } else if y >= 59 && y <= 64 {
                                    top_block = biome_top_block;
                                    filler_block = biome_filler_block;
                                }

                                if y < 63 && top_block == 0 {
                                    if temp < 0.15 {
                                        top_block = ice_block;
                                    } else {
                                        top_block = water_block;
                                    }
                                }

                                depth = noise_val;

                                chunk.set_block_id(x, y, z, if y >= 62 {
                                    top_block
                                } else {
                                    filler_block
                                });

                            }

                            if depth > 0 {

                                depth -= 1;
                                chunk.set_block_id(x, y, z, filler_block);

                                if depth == 0 && filler_block == sand_block {
                                    // This block is used to generate the sandstone behind the sand in
                                    // the desert.
                                    depth = self.rand.next_int_bounded(4);
                                    filler_block = sand_stone_block;
                                }

                            }

                        }

                    }

                }

            }
        }

    }

}


pub struct ChunkGenerator102(RefCell<ChunkGeneratorInternal>);

impl ChunkGenerator102 {
    pub fn new(world_info: Rc<WorldInfo>) -> Self {
        ChunkGenerator102(RefCell::new(ChunkGeneratorInternal::new(world_info)))
    }
}

impl ChunkLoader for ChunkGenerator102 {

    fn load_chunk(&self, cx: i32, cz: i32) -> Result<Chunk, ChunkError> {
        self.0.borrow_mut().generate_chunk(cx, cz)
    }

    fn populate_chunk(&self, world: &mut WorldAccess, cx: i32, cz: i32) {
        self.0.borrow_mut().populate_chunk(world, cx, cz);
    }

}
*/