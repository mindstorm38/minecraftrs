use super::java::JavaRandom;
use crate::math::{lerp, Cube};
use std::cell::RefCell;


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
    permutations: Vec<u16>,
    x: f64,
    y: f64,
    z: f64
}

impl PerlinNoise {

    pub fn new(rand: &mut JavaRandom) -> Self {

        let mut permutations = vec![0; 512];
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

        PerlinNoise { permutations, x, y, z }

    }

    #[inline]
    fn calc_coord_data(mut coord: f64) -> (f64, f64, usize) {

        let coord_floor = coord.floor();
        let permutation_index = (coord_floor as i32 & 0xff) as usize;
        coord -= coord_floor;
        let lerp_factor = coord * coord * coord * (coord * (coord * 6.0 - 15.0) + 10.0);

        (coord, lerp_factor, permutation_index)

    }

    pub fn generate(&self, cube: &mut NoiseCube, x: f64, y: f64, z: f64, x_scale: f64, y_scale: f64, z_scale: f64, scale: f64) {

        let only_2d = cube.y_size == 1;
        let scale_inverted = 1.0 / scale;

        let mut last_y_permutation = -1;
        let (mut p1, mut p2, mut p3, mut p4) = (0.0, 0.0, 0.0, 0.0);

        for cx in 0..cube.x_size {
            let (rx, x_lerpf, x_permutation) = Self::calc_coord_data(x + cx as f64 * x_scale + self.x);
            for cz in 0..cube.z_size {
                let (rz, z_lerpf, z_permutation) = Self::calc_coord_data(z + cz as f64 * z_scale + self.z);
                if only_2d {

                    let a = self.permutations[x_permutation] as usize;
                    let b = self.permutations[a] as usize + z_permutation;
                    let c = self.permutations[x_permutation + 1] as usize;
                    let d = self.permutations[c] as usize + z_permutation;

                    p1 = lerp(x_lerpf,
                              grad2(self.permutations[b], rx, rz),
                              grad(self.permutations[d], rx - 1.0, 0.0, rz));

                    p2 = lerp(x_lerpf,
                              grad(self.permutations[b + 1], rx, 0.0, rz - 1.0),
                              grad(self.permutations[d + 1], rx - 1.0, 0.0, rz - 1.0));

                    p3 = lerp(z_lerpf, p1, p2);

                    cube.add(cx, 0, cz, p3 * scale_inverted);

                } else {
                    for cy in 0..cube.y_size {

                        let (ry, y_lerpf, y_permutation) = Self::calc_coord_data(y + cy as f64 * y_scale + self.y);

                        if cy == 0 || y_permutation as isize != last_y_permutation {
                            last_y_permutation = y_permutation as isize;

                            let a = self.permutations[x_permutation] as usize + y_permutation;
                            let b = self.permutations[a] as usize + z_permutation;
                            let c = self.permutations[a + 1] as usize + z_permutation;

                            let d = self.permutations[x_permutation + 1] as usize + y_permutation;
                            let e = self.permutations[d] as usize + z_permutation;
                            let f = self.permutations[d + 1] as usize + z_permutation;

                            p1 = lerp(x_lerpf,
                                      grad(self.permutations[b], rx, ry, rz),
                                      grad(self.permutations[e], rx - 1.0, ry, rz));

                            p2 = lerp(x_lerpf,
                                      grad(self.permutations[c], rx, ry - 1.0, rz),
                                      grad(self.permutations[f], rx - 1.0, ry - 1.0, rz));

                            p3 = lerp(x_lerpf,
                                      grad(self.permutations[b + 1], rx, ry, rz - 1.0),
                                      grad(self.permutations[e + 1], rx - 1.0, ry, rz - 1.0));

                            p4 = lerp(x_lerpf,
                                      grad(self.permutations[c + 1], rx, ry - 1.0, rz - 1.0),
                                      grad(self.permutations[f + 1], rx - 1.0, ry - 1.0, rz - 1.0));
                        }

                        let p5 = lerp(y_lerpf, p1, p2);
                        let p6 = lerp(y_lerpf, p3, p4);
                        let p7 = lerp(z_lerpf, p5, p6);

                        cube.add(cx, cy, cz, p7 * scale_inverted);

                    }
                }
            }
        }

    }

}


/// Octaves perlin noise generator with variable octaves count.
///
/// Valid for: 1.2.5
pub struct OctavesPerlinNoise {
    generators: Vec<PerlinNoise>
}

impl OctavesPerlinNoise {

    pub fn new(rand: &mut JavaRandom, octaves: usize) -> Self {
        assert!(octaves > 0);
        OctavesPerlinNoise {
            generators: (0..octaves).map(|_| PerlinNoise::new(rand)).collect()
        }
    }

    pub fn generate(&self, cube: &mut NoiseCube, x: i32, y: i32, z: i32, x_scale: f64, y_scale: f64, z_scale: f64) {

        // println!("Generating octaves perlin noise at {}/{}/{} scales: {}/{}/{}", x, y, z, x_scale, y_scale, z_scale);
        cube.reset(0.0);

        //println!("Generate noise octaves, x: {}, y: {}, z: {}, xSize: {}, ySize: {}, zSize: {}, xScale: {}, yScale: {}, zScale: {}", x, y, z, cube.x_size, cube.y_size, cube.z_size, x_scale, y_scale, z_scale);

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

            //println!(" => rx: {}, ry: {}, rz: {}, xScale: {}, yScale: {}, zScale: {}, scale: {}", rx, ry, rz, x_scale, y_scale, z_scale, scale);

            generator.generate(cube, rx, ry, rz, x_scale * scale, y_scale * scale, z_scale * scale, scale);
            scale /= 2.0;

        }

    }

}


pub struct FixedOctavesPerlinNoise(OctavesPerlinNoise, RefCell<NoiseCube>);

impl FixedOctavesPerlinNoise {

    pub fn new(rand: &mut JavaRandom, octaves: usize, x_size: usize, y_size: usize, z_size: usize) -> Self {
        FixedOctavesPerlinNoise(
            OctavesPerlinNoise::new(rand, octaves),
            RefCell::new(NoiseCube::new_default(x_size, y_size, z_size))
        )
    }

    // &mut self just to known that mutations happens, but it's not mandatory
    pub fn generate(&mut self, x: i32, y: i32, z: i32, x_scale: f64, y_scale: f64, z_scale: f64) {
        self.0.generate(&mut self.1.borrow_mut(), x, y, z, x_scale, y_scale, z_scale);
    }

    #[inline]
    pub fn get_noise(&self, x: usize, y: usize, z: usize) -> f64 {
        self.1.borrow().get(x, y, z)
    }

}
