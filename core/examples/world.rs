use mc_core::world::level::{LevelEnv, LevelHeight, Level};
use mc_core::world::chunk::{Chunk, SubChunk};

use std::mem::size_of;
use mc_core::world::loader::NoChunkLoader;

fn main() {

    let env = LevelEnv::new_vanilla()
        .unwrap()
        .with_height(0, 15);

    let _lvl = Level::new("overworld".to_string(), &env, NoChunkLoader);

    println!("===== MEMORY USAGE =====");
    let chunk_sizeof = size_of::<Chunk>();
    let sub_chunk_sizeof = size_of::<SubChunk>();
    println!("Height sizeof: {}", size_of::<LevelHeight>());
    println!("Level sizeof: {}", size_of::<Level>());
    println!("Chunk sizeof: {}", chunk_sizeof);
    println!("SubChunk sizeof: {}", sub_chunk_sizeof);
    println!("For a whole loaded region: {}", 32 * 32 * (chunk_sizeof + 16 * sub_chunk_sizeof));
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
