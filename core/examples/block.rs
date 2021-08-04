use mc_core::block::{Block, WorkBlocks, Property, PropertySerializable, BlockState};
use mc_core::block::vanilla::*;
// use mc_core::block::legacy::setup_legacy_ids;
use std::time::Instant;
use std::mem::size_of;


fn main() {

    println!();
    println!("==== LOADING STATES ====");
    let start = Instant::now();
    &*VanillaBlocks;
    println!("Vanilla states loaded in {}ms", start.elapsed().as_secs_f32() * 1000.0);

    // setup_legacy_ids();

    let start = Instant::now();
    let mut blocks = WorkBlocks::new();
    blocks.register_static(&*VanillaBlocks).unwrap();
    println!("Vanilla blocks registered in {}us", start.elapsed().as_micros());
    let blocks_count = blocks.blocks_count();
    let states_count = blocks.states_count();
    println!("Vanilla blocks count: {}", blocks_count);
    println!("Vanilla states count: {}", states_count);
    println!("========================");
    println!();

    println!("====== EXTENSIONS ======");
    let start = Instant::now();
    VanillaBlocks.DIRT.add_ext(TestBlockExt {
        dummy_property: 42
    });
    println!("Dirt ext added (in {}ns)", start.elapsed().as_nanos());
    let start = Instant::now();
    let test = &*VanillaBlocks.DIRT.get_ext::<TestBlockExt>().unwrap();
    println!("Dirt ext value: {} (in {}ns)", test.dummy_property, start.elapsed().as_nanos());
    println!("========================");
    println!();

    println!("====== TEST BLOCK ======");
    let block = &VanillaBlocks.BREWING_STAND;
    println!("Testing block {}...", block.get_name());
    let state = block.get_default_state();

    println!("Duration 'with' (same): {}ns", time_average_with(&*state, &PROP_HAS_BOTTLE_0, false));
    println!("Duration 'with' (diff): {}ns", time_average_with(&*state, &PROP_HAS_BOTTLE_0, true));

    println!("State: {:?}", state);
    println!("State with: {:?}", state.with(&PROP_HAS_BOTTLE_0, true));
    let state_sid = blocks.get_sid_from(&*state).unwrap();
    println!("State uid in reg: {:?}", state_sid);
    println!("State from its sid: {:?}", blocks.get_state_from(state_sid));
    println!("========================");
    println!();

    println!("===== OTHER BLOCKS =====");
    println!("State with uid 1 in reg: {:?}", blocks.get_state_from(1));
    println!("State with uid 2 in reg: {:?}", blocks.get_state_from(2));
    println!("State with uid 54 in reg: {:?}", blocks.get_state_from(54));
    println!("========================");
    println!();

    println!("===== MEMORY USAGE =====");
    let block_sizeof = size_of::<Block>();
    let state_sizeof = size_of::<BlockState>();
    println!("Block sizeof: {} (total: {})", block_sizeof, block_sizeof * blocks_count);
    println!("State sizeof: {} (total: {})", state_sizeof, state_sizeof * states_count);
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


struct TestBlockExt {
    dummy_property: u32
}
