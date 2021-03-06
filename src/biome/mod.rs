use crate::res::Registrable;

/// Basic biome structure, storage for all settings of a biome type.
pub struct Biome {
    name: &'static str,
    id: u8,
    height_range: (f32, f32),
    pub temperature: f32,
    pub rainfall: f32,
    pub top_block: &'static str,
    pub filler_block: &'static str
}

impl Registrable<u8> for Biome {
    fn get_name(&self) -> &'static str { self.name }
    fn get_id(&self) -> u8 { self.id }
}

impl Biome {

    pub fn new(name: &'static str, id: u8) -> Self {
        Biome {
            name,
            id,
            height_range: (0.1, 0.3),
            temperature: 0.5,
            rainfall: 0.5,
            top_block: "grass",
            filler_block: "dirt"
        }
    }

    pub fn set_height_range(mut self, min: f32, max: f32) -> Self {
        self.height_range = (min, max);
        self
    }

    pub fn get_min_height(&self) -> f32 {
        self.height_range.0
    }

    pub fn get_max_height(&self) -> f32 {
        self.height_range.1
    }

    pub fn set_temp(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    pub fn set_rainfall(mut self, rainfall: f32) -> Self {
        self.rainfall = rainfall;
        self
    }

    pub fn set_temp_rainfall(mut self, temp: f32, rainfall: f32) -> Self {
        self.temperature = temp;
        self.rainfall = rainfall;
        self
    }

    pub fn set_blocks(mut self, top_block: &'static str, filler_block: &'static str) -> Self {
        self.top_block = top_block;
        self.filler_block = filler_block;
        self
    }

}

mod registry;
pub use registry::*;
