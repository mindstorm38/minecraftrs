use std::time::Instant;

use mc_runtime::world::{WorldContext, World, WorldSystemExecutor};
use mc_runtime::util::{tick_loop};

use mc_runtime::system::system_load_chunks;


fn main() {

    let mut ctx = WorldContext::new();
    ctx.register(register_systems);
    ctx.run_simple();

}

fn register_systems(world: &mut World, executor: &mut WorldSystemExecutor) {

    executor.add_system(system_load_chunks);

    println!("Systems");
    for system_name in executor.iter_system_names() {
        println!("- {}", system_name);
    }

}