use std::time::Instant;

use mc_runtime::world::{World, WorldSystemExecutor};
use mc_runtime::util::{tick_loop};


fn main() {

    let mut world = World::new();
    world.with_executor(register_systems);
    world.simple_run();

}

fn register_systems(world: &mut World, executor: &mut WorldSystemExecutor) {

    world.insert_component(TpsTrackerComponent::new());
    executor.add_system(debug_tps);

    println!("Systems");
    for system_name in executor.iter_system_names() {
        println!("- {}", system_name);
    }

}


fn debug_tps(world: &mut World) {
    let mut tps_tracker = world.get_component_mut::<TpsTrackerComponent>().unwrap();
    println!("TPS: {}", tps_tracker.tick());
}


pub struct TpsTrackerComponent {
    last_time: Instant,
    tps_sum: f32,
    tps_count: usize,
    tps: f32
}

impl TpsTrackerComponent {

    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            tps_sum: 0.0,
            tps_count: 0,
            tps: 0.0
        }
    }

    pub fn tick(&mut self) -> f32 {

        let now = Instant::now();
        let duration = now - self.last_time;
        let tps = 1.0 / duration.as_secs_f32();

        self.last_time = now;

        self.tps_sum += tps;
        self.tps_count += 1;

        if self.tps_count == 20 {
            self.tps = self.tps_sum / 20.0;
            self.tps_sum = 0.0;
            self.tps_count = 0;
        }

        self.tps

    }

}
