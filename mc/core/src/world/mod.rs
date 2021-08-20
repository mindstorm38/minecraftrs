use std::collections::HashMap;
use std::any::{TypeId, Any};

pub mod level;
pub mod chunk;
pub mod source;

pub mod anvil;

pub mod system;
use system::SystemComponents;



pub struct World {
    components: SystemComponents
}

impl World {

    #[inline]
    pub fn get_components(&self) -> &SystemComponents {
        &self.components
    }

    #[inline]
    pub fn get_components_mut(&mut self) -> &mut SystemComponents {
        &mut self.components
    }

}
