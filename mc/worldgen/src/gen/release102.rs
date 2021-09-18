//!
//! Generator for release 1.2
//!

use std::num::Wrapping;

use mc_core::world::source::{LevelGenerator, ProtoChunk, ChunkInfo, LevelSourceError};
use mc_core::rand::JavaRandom;

use crate::noise::{FixedPerlinNoiseOctaves, NoiseCube};

use crate::layer_new::{LayerSystem};
use crate::layer_new::island::{IslandLayer, AddIslandLayer, AddMushroomIsland};
use crate::layer_new::zoom::ZoomLayer;
use crate::layer_new::snow::AddSnowLayer;
use crate::layer_new::river::{InitRiverLayer, AddRiverLayer};
use crate::layer_new::smooth::SmoothLayer;
use crate::layer_new::biome::{BiomeLayer, HillsLayer, ShoreLayer, BiomeRiversLayer, MixBiomeAndRiverLayer};
use crate::layer_new::voronoi::VoronoiLayer;


/// The LevelGenerator for Minecraft 1.2 release.
// #[derive(Clone)]
pub struct LevelGenRelease102 {

    rand: JavaRandom,

    noise_main1: FixedPerlinNoiseOctaves,
    noise_main2: FixedPerlinNoiseOctaves,
    noise_main3: FixedPerlinNoiseOctaves,
    noise_main4: FixedPerlinNoiseOctaves,
    noise_main5: FixedPerlinNoiseOctaves,
    noise_surface: FixedPerlinNoiseOctaves,

    noise_field: NoiseCube,

    layer_system: LayerSystem

    //ravine_carver: Carver,
    //cave_carver: Carver

}

impl LevelGenRelease102 {

    pub fn new(seed: i64) -> Self {

        const WIDTH: usize = 5;
        const HEIGHT: usize = 17;

        let mut rand = JavaRandom::new(seed);

        Self {
            noise_main1: FixedPerlinNoiseOctaves::new(&mut rand, 16, WIDTH, HEIGHT, WIDTH),
            noise_main2: FixedPerlinNoiseOctaves::new(&mut rand, 16, WIDTH, HEIGHT, WIDTH),
            noise_main3: FixedPerlinNoiseOctaves::new(&mut rand, 8, WIDTH, HEIGHT, WIDTH),
            noise_surface: FixedPerlinNoiseOctaves::new(&mut rand, 4, 16, 16, 1),
            noise_main4: FixedPerlinNoiseOctaves::new(&mut rand, 10, WIDTH, 1, WIDTH),
            noise_main5: FixedPerlinNoiseOctaves::new(&mut rand, 16, WIDTH, 1, WIDTH),
            noise_field: NoiseCube::new_default(WIDTH, HEIGHT, WIDTH),
            layer_system: Self::new_layers(seed),
            // ravine_carver: Carver::new_ravine(),
            // cave_carver: Carver::new_cave(),
            rand,
        }

    }

    fn new_layers(seed: i64) -> LayerSystem {

        let mut system = LayerSystem::new();

        system.push(IslandLayer::new(1));
        system.push(ZoomLayer::new_fuzzy(2000));
        system.push(AddIslandLayer::new(1));
        system.push(ZoomLayer::new_smart(2001));
        system.push(AddIslandLayer::new(2));
        system.push(AddSnowLayer::new(2));
        system.push(ZoomLayer::new_smart(2002));
        system.push(AddIslandLayer::new(3));
        system.push(ZoomLayer::new_smart(2003));
        system.push(AddIslandLayer::new(4));
        system.push(AddMushroomIsland::new(5));
        let common_idx = system.last_index().unwrap();

        system.push(InitRiverLayer::new(100));
        system.push_iter((0..6).map(|i| ZoomLayer::new_smart(1000 + i as i64)));
        system.push(AddRiverLayer);
        system.push(SmoothLayer::new(1000));
        let river_idx = system.last_index().unwrap();

        system.push_with_parents(BiomeLayer::new_102(200), vec![common_idx]);
        system.push_iter((0..2).map(|i| ZoomLayer::new_smart(1000 + i as i64)));
        system.push(HillsLayer::new(1000));
        for i in 0..4 {
            system.push(ZoomLayer::new_smart(1000 + i));
            match i {
                0 => system.push(AddIslandLayer::new(3)),
                1 => {
                    system.push(ShoreLayer::new(1000));
                    system.push(BiomeRiversLayer::new(1000));
                },
                _ => {}
            }
        }
        system.push(SmoothLayer::new(1000));
        let biome_idx = system.last_index().unwrap();

        system.push_with_parents(MixBiomeAndRiverLayer, vec![biome_idx, river_idx]);
        system.push(VoronoiLayer::new(10));

        system.seed(seed);
        system

        /*// Common layers
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
        voronoi*/

    }

    /*fn initialize_biomes(&mut self, chunk: &mut Chunk) -> [&'static Biome; 256] {
        let (cx, cz) = chunk.get_position();
        let layer_biomes = self.layer_system.borrow_root().unwrap()
            .generate_size(cx * 16, cz * 16, 16, 16);
        let biomes = layer_into_biomes::<256>(layer_biomes).unwrap();
        chunk.set_biomes_2d(&biomes);
        biomes
    }*/

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

        let chunk = info.build_proto_chunk();
        // let biomes = self.initialize_biomes(&mut *chunk);

        Ok(chunk)

    }

}






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