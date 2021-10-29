use mc_core::util::{Rect, Cube};
use mc_core::rand::JavaRandom;
use mc_core::math::lerp;

use std::mem::MaybeUninit;


pub type NoiseRect = Rect<f64>;
pub type NoiseCube = Cube<f64>;


fn grad(value: u16, x: f64, y: f64, z: f64) -> f64 {

    let i = value & 0xf;

    let a = if i >= 8 { y } else { x };
    let b = if i >= 4 {
        if i != 12 && i != 14 { z } else { x }
    } else { y };

    (if (i & 1) != 0 { -a } else { a }) +
    (if (i & 2) != 0 { -b } else { b })

}

fn grad2(value: u16, x: f64, z: f64) -> f64 {

    let i = value & 0xf;

    let a = (1.0 - ((i & 8) >> 3) as f64) * x;
    let b = if i >= 4 {
        if i != 12 && i != 14 { z } else { x }
    } else { 0.0 };

    (if (i & 1) != 0 { -a } else { a }) +
    (if (i & 2) != 0 { -b } else { b })

}


/// Perlin noise generator.
///
/// Valid for: 1.2.5
pub struct PerlinNoise {
    permutations: Box<[u16; 512]>,
    x: f64,
    y: f64,
    z: f64
}

impl PerlinNoise {

    /// Construct a new perlin noise generator, the given RNG is used to construct the
    /// internal permutation table and is not kept into the structure. Internally,
    /// `next_double` is called 3 times and then `next_int_bounded` is called 256 times.
    pub fn new(rand: &mut JavaRandom) -> Self {

        let mut permutations = [0; 512];

        let x = rand.next_double() * 256.0;
        let y = rand.next_double() * 256.0;
        let z = rand.next_double() * 256.0;

        for i in 0..256usize {
            permutations[i] = i as u16;
        }

        for i in 0..256usize {
            let n = rand.next_int_bounded(256 - i as i32) as usize + i as usize;
            let t = permutations[i];
            permutations[i] = permutations[n];
            permutations[n] = t;
            permutations[i + 256] = permutations[i];
        }

        Self {
            permutations: Box::new(permutations),
            x, y, z
        }

    }

    #[inline]
    fn calc_coord_data(mut coord: f64) -> (f64, f64, usize) {

        let coord_floor = coord.floor();
        let permutation_index = (coord_floor as i32 & 0xff) as usize;
        coord -= coord_floor;
        let lerp_factor = coord * coord * coord * (coord * (coord * 6.0 - 15.0) + 10.0);

        (coord, lerp_factor, permutation_index)

    }

    /// Generate a noise field in a 2D rectangle.
    pub fn generate_2d(&self, rect: &mut NoiseRect, x: f64, z: f64, x_scale: f64, z_scale: f64, scale: f64) {

        let scale_inverted = 1.0 / scale;

        for cx in 0..rect.x_size {
            let (rx, x_lerp_factor, x_permutation) = Self::calc_coord_data(x + cx as f64 * x_scale + self.x);
            for cz in 0..rect.z_size {
                let (rz, z_lerp_factor, z_permutation) = Self::calc_coord_data(z + cz as f64 * z_scale + self.z);

                let a = self.permutations[x_permutation] as usize;
                let b = self.permutations[a] as usize + z_permutation;
                let c = self.permutations[x_permutation + 1] as usize;
                let d = self.permutations[c] as usize + z_permutation;

                let p1 = lerp(x_lerp_factor,
                              grad2(self.permutations[b], rx, rz),
                              grad(self.permutations[d], rx - 1.0, 0.0, rz));

                let p2 = lerp(x_lerp_factor,
                              grad(self.permutations[b + 1], rx, 0.0, rz - 1.0),
                              grad(self.permutations[d + 1], rx - 1.0, 0.0, rz - 1.0));

                let p3 = lerp(z_lerp_factor, p1, p2);

                rect.add(cx, cz, p3 * scale_inverted);

            }
        }

    }

    /// Generate a noise field in a 3D cube.
    pub fn generate_3d(&self, cube: &mut NoiseCube, x: f64, y: f64, z: f64, x_scale: f64, y_scale: f64, z_scale: f64, scale: f64) {

        let scale_inverted = 1.0 / scale;
        let mut last_y_permutation = -1;

        let mut p1 = 0.0;
        let mut p2 = 0.0;
        let mut p3 = 0.0;
        let mut p4 = 0.0;

        for cx in 0..cube.x_size {
            let (rx, x_lerp_factor, x_permutation) = Self::calc_coord_data(x + cx as f64 * x_scale + self.x);
            for cz in 0..cube.z_size {
                let (rz, z_lerp_factor, z_permutation) = Self::calc_coord_data(z + cz as f64 * z_scale + self.z);
                for cy in 0..cube.y_size {
                    let (ry, y_lerp_factor, y_permutation) = Self::calc_coord_data(y + cy as f64 * y_scale + self.y);

                    if cy == 0 || y_permutation as isize != last_y_permutation {
                        last_y_permutation = y_permutation as isize;

                        let a = self.permutations[x_permutation] as usize + y_permutation;
                        let b = self.permutations[a] as usize + z_permutation;
                        let c = self.permutations[a + 1] as usize + z_permutation;

                        let d = self.permutations[x_permutation + 1] as usize + y_permutation;
                        let e = self.permutations[d] as usize + z_permutation;
                        let f = self.permutations[d + 1] as usize + z_permutation;

                        p1 = lerp(x_lerp_factor,
                                  grad(self.permutations[b], rx, ry, rz),
                                  grad(self.permutations[e], rx - 1.0, ry, rz));

                        p2 = lerp(x_lerp_factor,
                                  grad(self.permutations[c], rx, ry - 1.0, rz),
                                  grad(self.permutations[f], rx - 1.0, ry - 1.0, rz));

                        p3 = lerp(x_lerp_factor,
                                  grad(self.permutations[b + 1], rx, ry, rz - 1.0),
                                  grad(self.permutations[e + 1], rx - 1.0, ry, rz - 1.0));

                        p4 = lerp(x_lerp_factor,
                                  grad(self.permutations[c + 1], rx, ry - 1.0, rz - 1.0),
                                  grad(self.permutations[f + 1], rx - 1.0, ry - 1.0, rz - 1.0));
                    }

                    let p5 = lerp(y_lerp_factor, p1, p2);
                    let p6 = lerp(y_lerp_factor, p3, p4);
                    let p7 = lerp(z_lerp_factor, p5, p6);

                    cube.add(cx, cy, cz, p7 * scale_inverted);

                }
            }
        }

    }

}


/// Octaves perlin noise generator with variable octaves count.
///
/// Valid for: 1.2.5
pub struct PerlinNoiseOctaves<const OCTAVES: usize> {
    generators: [PerlinNoise; OCTAVES]
}

impl<const OCTAVES: usize> PerlinNoiseOctaves<OCTAVES> {

    /// Construct new perlin noise octaves, the required RNG is used to make permutation
    /// tables of each layer. This random is not kept into the structure.
    pub fn new(rand: &mut JavaRandom) -> Self {

        assert!(OCTAVES > 0);

        let mut uninit_generators: [MaybeUninit<PerlinNoise>; OCTAVES] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for i in 0..OCTAVES {
            uninit_generators[i] = MaybeUninit::new(PerlinNoise::new(rand));
        }

        Self {
            generators: unsafe {
                std::mem::transmute_copy(&uninit_generators)
            }
        }

    }

    pub fn generate_2d(&self, rect: &mut NoiseRect, x: i32, z: i32, x_scale: f64, z_scale: f64) {
        rect.fill(0.0);
        let mut scale = 1.0;
        for generator in &self.generators {

            let mut rx = x as f64 * scale * x_scale;
            let mut rz = z as f64 * scale * z_scale;

            let rx_floor = rx.floor() as i64;
            let rz_floor = rz.floor() as i64;

            rx -= rx_floor as f64;
            rz -= rz_floor as f64;
            rx += (rx_floor % 0x1000000) as f64;
            rz += (rz_floor % 0x1000000) as f64;

            generator.generate_2d(rect, rx, rz, x_scale * scale, z_scale * scale, scale);
            scale /= 2.0;

        }
    }

    pub fn generate_3d(&self, cube: &mut NoiseCube, x: i32, y: i32, z: i32, x_scale: f64, y_scale: f64, z_scale: f64) {
        cube.fill(0.0);
        let mut scale = 1.0;
        for generator in &self.generators {

            let mut rx = x as f64 * scale * x_scale;
            let     ry = y as f64 * scale * y_scale;
            let mut rz = z as f64 * scale * z_scale;

            let rx_floor = rx.floor() as i64;
            let rz_floor = rz.floor() as i64;

            rx -= rx_floor as f64;
            rz -= rz_floor as f64;
            rx += (rx_floor % 0x1000000) as f64;
            rz += (rz_floor % 0x1000000) as f64;

            generator.generate_3d(cube, rx, ry, rz, x_scale * scale, y_scale * scale, z_scale * scale, scale);
            scale /= 2.0;

        }
    }

}


/*/// A `PerlinNoiseOctaves` wrapped with a noise cube, it allows you to generate .
pub struct CachedPerlinNoiseOctaves(PerlinNoiseOctaves, NoiseCube);

impl CachedPerlinNoiseOctaves {

    pub fn new(rand: &mut JavaRandom, octaves: usize, x_size: usize, y_size: usize, z_size: usize) -> Self {
        Self(
            PerlinNoiseOctaves::new(rand, octaves),
            NoiseCube::new_default(x_size, y_size, z_size)
        )
    }

    #[inline]
    pub fn generate(&mut self, x: i32, y: i32, z: i32, x_scale: f64, y_scale: f64, z_scale: f64) {
        self.0.generate(&mut self.1, x, y, z, x_scale, y_scale, z_scale);
    }

    #[inline]
    pub fn get_noise(&self, x: usize, y: usize, z: usize) -> f64 {
        *self.1.get(x, y, z)
    }

}
*/