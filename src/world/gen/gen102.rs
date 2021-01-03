//!
//! Generation for release 1.2
//!

use crate::rand::jrand::JavaRandom;
use crate::rand::noise::{NoiseCube, FixedOctavesPerlinNoise};
use crate::world::provider::{ChunkLoader, ChunkError};
use crate::world::chunk::Chunk;
use crate::world::WorldInfo;
use std::cell::RefCell;
use std::num::Wrapping;
use std::rc::Rc;


struct ChunkGeneratorInternal {

    world_info: Rc<WorldInfo>,
    rand: JavaRandom,

    noise_main1: FixedOctavesPerlinNoise,
    noise_main2: FixedOctavesPerlinNoise,
    noise_main3: FixedOctavesPerlinNoise,
    noise_main4: FixedOctavesPerlinNoise,
    noise_main5: FixedOctavesPerlinNoise,
    noise_surface: FixedOctavesPerlinNoise,

    noise_field: NoiseCube

}

impl ChunkGeneratorInternal {

    fn new(world_info: Rc<WorldInfo>) -> Self {

        const WIDTH: usize = 5;
        const HEIGHT: usize = 17;

        let mut rand = JavaRandom::new(world_info.seed);

        ChunkGeneratorInternal {
            world_info,
            noise_main1: FixedOctavesPerlinNoise::new(&mut rand, 16, WIDTH, HEIGHT, WIDTH),
            noise_main2: FixedOctavesPerlinNoise::new(&mut rand, 16, WIDTH, HEIGHT, WIDTH),
            noise_main3: FixedOctavesPerlinNoise::new(&mut rand, 8, WIDTH, HEIGHT, WIDTH),
            noise_surface: FixedOctavesPerlinNoise::new(&mut rand, 4, 16, 16, 1),
            noise_main4: FixedOctavesPerlinNoise::new(&mut rand, 10, WIDTH, 1, WIDTH),
            noise_main5: FixedOctavesPerlinNoise::new(&mut rand, 16, WIDTH, 1, WIDTH),
            noise_field: NoiseCube::new(WIDTH, HEIGHT, WIDTH),
            rand
        }

    }

    fn initialize_noise_field(&mut self, x: i32, y: i32, z: i32) {

        const WIDTH_SCALE: f64 = 684.41200000000003;
        const HEIGHT_SCALE: f64 = 684.41200000000003;

        self.noise_main4.generate(x, 10, z, 1.121, 1.0, 1.121);
        self.noise_main5.generate(x, 10, z, 200.0, 1.0, 200.0);
        self.noise_main3.generate(x, y, z, WIDTH_SCALE / 80.0, HEIGHT_SCALE / 160.0, WIDTH_SCALE / 80.0);
        self.noise_main1.generate(x, y, z, WIDTH_SCALE, HEIGHT_SCALE, WIDTH_SCALE);
        self.noise_main2.generate(x, y, z, WIDTH_SCALE, HEIGHT_SCALE, WIDTH_SCALE);

        for x in 0..self.noise_field.x_size {
            for z in 0..self.noise_field.z_size {

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

        const X_MUL: Wrapping<i64> = Wrapping(0x4f9939f508);
        const Z_MUL: Wrapping<i64> = Wrapping(0x1ef1565bd5);

        //self.rand.set_seed((Wrapping(cx as i64) * X_MUL + Wrapping(cz as i64) * Z_MUL).0);

        Err(ChunkError::IllegalPosition(cx, cz))

    }

}