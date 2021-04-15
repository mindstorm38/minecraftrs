use mc_core::world::World;
use mc_core::block::vanilla::*;

fn main() {

    let world = World::new_vanilla();
    let mut world_guard = world.write().unwrap();

    let overworld = world_guard.add_level("overworld").unwrap();
    let mut overworld_guard = overworld.write().unwrap();

    let chunk = overworld_guard.get_storage_mut().get_chunk_mut(0, 0).unwrap();
    let mut chunk_guard = chunk.write();
    chunk_guard.set_block(0, 0, 0, VanillaBlocks.BEACON.get_default_state());

    println!("level id: {}", overworld_guard.get_id());

}