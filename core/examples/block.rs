use mc_core::block::{Block, Blocks, StaticBlocks, Property, PropertySerializable, BlockState};
use mc_core::block::vanilla::*;
use std::mem::size_of;
use std::time::Instant;


fn main() {

    let start = Instant::now();
    let mut blocks = Blocks::new();
    blocks.register(&*VanillaBlocks);
    println!("States computed in {}s", start.elapsed().as_secs_f32());

    let start = Instant::now();
    VanillaBlocks.DIRT.add_ext(TestBlockExt {
        dummy_property: 42
    });
    println!("Dirt ext added (in {}ns)", start.elapsed().as_nanos());

    let block = &VanillaBlocks.BREWING_STAND;
    let state = block.get_default_state();

    println!("Duration 'with' (same): {}ns", time_average_with(&*state, &PROP_HAS_BOTTLE_0, false));
    println!("Duration 'with' (diff): {}ns", time_average_with(&*state, &PROP_HAS_BOTTLE_0, true));

    println!("State: {:?}", state);
    println!("State with: {:?}", state.with(&PROP_HAS_BOTTLE_0, true));
    println!("State uid in reg: {}", blocks.get_state_uid(&*state));
    println!("State with uid 1 in reg: {:?}", blocks.get_state(1));
    println!("State with uid 2 in reg: {:?}", blocks.get_state(2));
    println!("State with uid 54 in reg: {:?}", blocks.get_state(54));
    println!("State sizeof: {}", size_of::<BlockState>());
    println!("Block sizeof: {}", size_of::<Block>());
    println!("States count: {}", VanillaBlocks.get_last_uid());
    println!("Blocks count: {}", VanillaBlocks.get_block_count());

    let start = Instant::now();
    let test = &*VanillaBlocks.DIRT.get_ext::<TestBlockExt>().unwrap();
    println!("Dirt ext value: {} (in {}ns)", test.dummy_property, start.elapsed().as_nanos());

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
