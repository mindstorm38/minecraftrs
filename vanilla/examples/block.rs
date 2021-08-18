use mc_core::block::{Block, GlobalBlocks, Property, PropertySerializable, BlockState};
use mc_vanilla::block::*;
use std::time::Instant;
use std::mem::size_of;


fn main() {

    println!("==== LOADING STATES ====");
    let start = Instant::now();
    let mut blocks = GlobalBlocks::new();
    blocks.register_static(&VANILLA_BLOCKS).unwrap();
    println!("Vanilla blocks registered in {}us", start.elapsed().as_micros());
    let blocks_count = blocks.blocks_count();
    let states_count = blocks.states_count();
    println!("Vanilla blocks count: {}", blocks_count);
    println!("Vanilla states count: {}", states_count);
    println!("========================");
    println!();

    println!("====== TEST BLOCK ======");
    let block = &BREWING_STAND;
    println!("Testing block {}...", block.get_name());
    let state = block.get_default_state();

    println!("Duration 'with' (same): {}ns", time_average_with(&*state, &PROP_HAS_BOTTLE_0, false));
    println!("Duration 'with' (diff): {}ns", time_average_with(&*state, &PROP_HAS_BOTTLE_0, true));

    println!("State: {:?}", state);
    println!("State with: {:?}", state.with(&PROP_HAS_BOTTLE_0, true));
    let start = Instant::now();
    let state_sid = blocks.get_sid_from(&*state).unwrap();
    let elapsed = start.elapsed().as_nanos();
    println!("State uid in reg: {:?} (in: {}ns)", state_sid, elapsed);
    println!("State from its sid: {:?}", blocks.get_state_from(state_sid));
    println!("========================");
    println!();

    println!("===== OTHER BLOCKS =====");
    let start = Instant::now();
    let state = blocks.get_state_from(1);
    let elapsed = start.elapsed().as_nanos();
    println!("State with uid 1 in reg: {:?} (in: {}ns)", state, elapsed);
    println!("State with uid 2 in reg: {:?}", blocks.get_state_from(2));
    println!("State with uid 54 in reg: {:?}", blocks.get_state_from(54));
    println!("========================");
    println!();

    println!("===== MEMORY USAGE =====");
    let block_sizeof = size_of::<Block>();
    let state_sizeof = size_of::<BlockState>();
    println!("Block sizeof: {} (total: {}ko)", block_sizeof, (block_sizeof * blocks_count) as f32 / 1000.0);
    println!("State sizeof: {} (total: {}ko)", state_sizeof, (state_sizeof * states_count) as f32 / 1000.0);
    println!("========================");

}


fn time_average_with<T, P>(state: &BlockState, prop: &P, value: T) -> u32
where
    T: PropertySerializable,
    P: Property<T>
{

    let mut total_time = 0;
    let total_count = 1000000;

    for _ in 0..total_count {
        let start = Instant::now();
        state.with(prop, value);
        total_time += start.elapsed().as_nanos();
    }

    (total_time / total_count) as u32

}
