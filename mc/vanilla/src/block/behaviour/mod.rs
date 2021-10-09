use super::*;

pub mod redstone;

use redstone::{REDSTONE_BEHAVIOURS, RedstoneBehaviour, RedstoneConstant};
use mc_core::block::BlockState;


pub fn register_behaviours() {

    // Redstone behaviours
    REDSTONE_BEHAVIOURS.register(&REDSTONE_BLOCK, &RedstoneConstant(15));
    REDSTONE_BEHAVIOURS.register(&STONE_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&OAK_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&SPRUCE_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&BIRCH_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&JUNGLE_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&DARK_OAK_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&CRIMSON_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&WARPED_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&POLISHED_BLACKSTONE_PRESSURE_PLATE, &BinaryPressurePlate);
    REDSTONE_BEHAVIOURS.register(&LIGHT_WEIGHTED_PRESSURE_PLATE, &AnalogPressurePlate);
    REDSTONE_BEHAVIOURS.register(&HEAVY_WEIGHTED_PRESSURE_PLATE, &AnalogPressurePlate);

}


pub struct BinaryPressurePlate;
impl RedstoneBehaviour for BinaryPressurePlate {

    fn is_signal_source(&self, _state: &BlockState) -> bool {
        true
    }

    fn get_signal(&self, state: &BlockState, _direction: Direction) -> u8 {
        if let Some(true) = state.get(&PROP_POWERED) {
            15
        } else {
            0
        }
    }

    fn get_direct_signal(&self, state: &BlockState, direction: Direction) -> u8 {
        match direction {
            Direction::Up => self.get_signal(state, direction),
            _ => 0
        }
    }

}

pub struct AnalogPressurePlate;
impl RedstoneBehaviour for AnalogPressurePlate {

    fn is_signal_source(&self, _state: &BlockState) -> bool {
        true
    }

    fn get_signal(&self, state: &BlockState, _direction: Direction) -> u8 {
        state.get(&PROP_REDSTONE_POWER).unwrap_or(0)
    }

    fn get_direct_signal(&self, state: &BlockState, direction: Direction) -> u8 {
        match direction {
            Direction::Up => self.get_signal(state, direction),
            _ => 0
        }
    }

}
