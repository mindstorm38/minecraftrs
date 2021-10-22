use mc_core::rand::JavaRandom;
use mc_core::block::BlockState;
use mc_core::math::{mc_cos, mc_sin, JAVA_PI};
use mc_vanilla::block::*;

use super::{Feature, LevelView};


pub struct WaterCircleFeature {
    block: &'static BlockState,
    replace_grass: bool,
    depth: u16,
    radius: u16
}

impl WaterCircleFeature {

    pub fn new(block: &'static BlockState, replace_grass: bool, depth: u16, radius: u16) -> Self {
        Self {
            block,
            replace_grass,
            depth,
            radius
        }
    }

    pub fn new_sand(radius: u16) -> Self {
        Self::new(SAND.get_default_state(), true, 2, radius)
    }

    pub fn new_gravel(radius: u16) -> Self {
        Self::new(GRAVEL.get_default_state(), true, 2, radius)
    }

    pub fn new_clay(radius: u16) -> Self {
        Self::new(CLAY.get_default_state(), false, 1, radius)
    }

}

impl Feature for WaterCircleFeature {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        if level.get_block_at(x, y, z).unwrap().is_block(&WATER) {
            let radius = rand.next_int_bounded(self.radius as i32 - 2) + 2;
            for bx in (x - radius)..=(x + radius) {
                for bz in (z - radius)..=(z + radius) {
                    let dx = bx - x;
                    let dz = bz - z;
                    if dx * dx + dz * dz <= radius * radius {
                        for by in (y - self.depth as i32)..=(y + self.depth as i32) {
                            if let Ok(prev_state) = level.get_block_at(bx, by, bz) {
                                if prev_state.is_block(&DIRT) || (self.replace_grass && prev_state.is_block(&GRASS_BLOCK)) {
                                    level.set_block_at(bx, by, bz, self.block).unwrap();
                                }
                            }
                        }
                    }
                }
            }
            true
        } else {
            false
        }
    }

}


pub struct VeinFeature {
    block: &'static BlockState,
    count: u16
}

impl VeinFeature {
    pub fn new(block: &'static BlockState, count: u16) -> Self {
        Self {
            block,
            count
        }
    }
}

impl Feature for VeinFeature {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {

        let angle = rand.next_float() * JAVA_PI as f32;
        let angle_sin = mc_sin(angle);
        let angle_cos = mc_cos(angle);

        let count_f32 = self.count as f32;
        let count_f64 = self.count as f64;

        let x_line_start = (x as f32 + angle_sin * count_f32 / 8.0) as f64;
        let x_line_end = (x as f32 - angle_sin * count_f32 / 8.0) as f64;
        let z_line_start = (z as f32 + angle_cos * count_f32 / 8.0) as f64;
        let z_line_end = (z as f32 - angle_cos * count_f32 / 8.0) as f64;
        let y_line_start = (y + rand.next_int_bounded(3) - 2) as f64;
        let y_line_end = (y + rand.next_int_bounded(3) - 2) as f64;

        for i in 0..self.count {

            let x_center = x_line_start + ((x_line_end - x_line_start) * i as f64) / count_f64;
            let y_center = y_line_start + ((y_line_end - y_line_start) * i as f64) / count_f64;
            let z_center = z_line_start + ((z_line_end - z_line_start) * i as f64) / count_f64;

            let base_size = rand.next_double() * count_f64 / 16.0;
            let half_size = ((mc_sin(i as f32 * JAVA_PI as f32 / count_f32) + 1.0) as f64 * base_size + 1.0) / 2.0;

            let x_start = (x_center - half_size).floor() as i32;
            let y_start = (y_center - half_size).floor() as i32;
            let z_start = (z_center - half_size).floor() as i32;
            let x_end = (x_center + half_size).floor() as i32;
            let y_end = (y_center + half_size).floor() as i32;
            let z_end = (z_center + half_size).floor() as i32;

            for bx in x_start..=x_end {
                let bx_dist = (bx as f64 + 0.5 - x_center) / half_size;
                if bx_dist * bx_dist < 1.0 {
                    for by in y_start..=y_end {
                        let by_dist = (by as f64 + 0.5 - y_center) / half_size;
                        if bx_dist * bx_dist + by_dist * by_dist < 1.0 {
                            for bz in z_start..=z_end {
                                let bz_dist = (bz as f64 + 0.5 - z_center) / half_size;
                                if bx_dist * bx_dist + by_dist * by_dist + bz_dist * bz_dist < 1.0 {
                                    if let Ok(block) = level.get_block_at(bx, by, bz) {
                                        if block.is_block(&STONE) {
                                            level.set_block_at(bx, by, bz, self.block).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

        }

        true

    }
}
