use mc_core::world::level::{LevelEnv, LevelHeight, Level};
use mc_core::world::chunk::{Chunk, SubChunk};
use mc_core::world::loader::NoChunkLoader;

use mc_core::world::anvil::region::RegionFile;

use std::mem::size_of;
use std::path::PathBuf;

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
    println!();

    println!("====== ANVIL TEST ======");

    let mut region_file = RegionFile::new(
        PathBuf::from(r"C:\Users\Theo\AppData\Roaming\.minecraft\saves\Test JDataPack\DIM1\region_copy"),
        0,
        0
    ).unwrap();

    let mut reader = region_file.get_chunk_reader(0, 0).unwrap();
    let mut data = Vec::new();
    reader.read_to_end(&mut data);

    println!("Chunk data length: {}", data.len());
    println!("Chunk data: {:?}", data);
    println!("Chunk data (string): {}", data.iter().map(|&data| char::from(data)).collect::<String>());

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
