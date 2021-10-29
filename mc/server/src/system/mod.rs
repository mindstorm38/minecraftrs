use mc_runtime::world::{World, WorldSystemExecutor};

pub mod protocol;
pub mod player;


/// Register all systems required for the server to run.
pub fn register_systems(world: &mut World, executor: &mut WorldSystemExecutor) {

    protocol::register_systems(world, executor);

    executor.add_system(player::system_player_view);
    executor.add_system(mc_runtime::system::system_load_chunks);

}
