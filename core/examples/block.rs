use mc_core::block::{Block, Blocks, StaticBlocks};
use mc_core::block::vanilla::*;
use std::mem::size_of;
use std::time::Instant;

fn main() {

    let start = Instant::now();

    let mut blocks = Blocks::new();
    blocks.register(&*VanillaBlocks);

    let block = &VanillaBlocks.NOTE_BLOCK;
    let state = block.get_default_state();

    println!("State: {:?}", state);
    println!("State uid in reg: {}", blocks.get_state_uid(&*state));
    println!("State with uid 1 in reg: {:?}", blocks.get_state(1));
    println!("State with uid 2 in reg: {:?}", blocks.get_state(2));
    println!("State with uid 54 in reg: {:?}", blocks.get_state(54));
    println!("State sizeof: {}", size_of::<Block>());
    println!("States count: {}", VanillaBlocks.get_last_uid());

    println!("Example {}s", start.elapsed().as_secs_f32());

}