use crate::res::Registrable;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};


/// A basic block.
#[derive(Debug)]
pub struct Block {
    name: &'static str,
    id: u16,
    hardness: f32,
    resistance: f32
}

impl Registrable<u16> for Block {
    fn get_name(&self) -> &'static str { self.name }
    fn get_id(&self) -> u16 { self.id }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("{}/{}", self.id, self.name))
    }
}

impl Block {

    pub fn new(name: &'static str, id: u16) -> Self {
        Block {
            name,
            id,
            hardness: 0.0,
            resistance: 0.0
        }
    }

    pub fn set_hardness(mut self, hardness: f32) -> Self {
        self.hardness = hardness;
        self
    }

    pub fn set_resistance(mut self, resistance: f32) -> Self {
        self.resistance = resistance * 3.0;
        self
    }

}

mod registry;
pub use registry::*;
