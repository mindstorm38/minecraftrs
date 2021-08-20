use mc_core::block::{BlockState, BlockBehaviourRegister};
use mc_core::pos::Direction;
use crate::block::REDSTONE_BLOCK;


pub static REDSTONE_BEHAVIOURS: BlockBehaviourRegister<&'static dyn RedstoneBehaviour> = BlockBehaviourRegister::new();


pub trait RedstoneBehaviour: Sync {
    fn is_signal_source(&self, state: &BlockState) -> bool;
    fn get_signal(&self, state: &BlockState, direction: Direction) -> u8;
    fn get_direct_signal(&self, state: &BlockState, direction: Direction) -> u8;
}


pub struct RedstoneConstant(pub u8);

impl RedstoneBehaviour for RedstoneConstant {
    fn is_signal_source(&self, _state: &BlockState) -> bool { true }
    fn get_signal(&self, _state: &BlockState, _direction: Direction) -> u8 { self.0 }
    fn get_direct_signal(&self, _state: &BlockState, _direction: Direction) -> u8 { 0 }
}


pub struct RedstoneEngine {
    // TODO
}
