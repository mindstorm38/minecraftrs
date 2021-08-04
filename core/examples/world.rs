use mc_core::world::level::{LevelEnv, LevelBuilder, LevelHeight, Level};
use mc_core::world::chunk::{Chunk, SubChunk};

use std::mem::size_of;

fn main() {

    let env = LevelEnv::new_vanilla().unwrap();

    let lvl = LevelBuilder::new("overworld")
        .with_height(0, 15)
        .build(&env);

    println!("===== MEMORY USAGE =====");
    println!("Height sizeof: {}", size_of::<LevelHeight>());
    println!("Level sizeof: {}", size_of::<Level>());
    println!("Chunk sizeof: {}", size_of::<Chunk>());
    println!("SubChunk sizeof: {}", size_of::<SubChunk>());
    println!("========================");

    /*let world = World::new_vanilla();
    let mut world_guard = world.write().unwrap();

    let overworld = world_guard.add_level("overworld").unwrap();
    let mut overworld_guard = overworld.write().unwrap();

    let chunk = overworld_guard.get_storage_mut().get_chunk_mut(0, 0).unwrap();
    let mut chunk_guard = chunk.write();
    chunk_guard.set_block(0, 0, 0, VanillaBlocks.BEACON.get_default_state());

    println!("level id: {}", overworld_guard.get_id());*/

}