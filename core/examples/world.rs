use mc_core::world::level::{LevelEnv, Level};
use mc_core::world::chunk::{Chunk, SubChunk, ChunkHeight};

use mc_core::world::anvil::source::{AnvilLevelSource, AnvilLevelSourceBuilder};

use std::mem::size_of;
use std::sync::Arc;
use std::time::Duration;


fn main() {

    println!("===== MEMORY USAGE =====");
    let chunk_sizeof = size_of::<Chunk>();
    let sub_chunk_sizeof = size_of::<SubChunk>();
    println!("Chunk height sizeof: {}", size_of::<ChunkHeight>());
    println!("Level sizeof: {}", size_of::<Level<AnvilLevelSource>>());
    println!("Chunk sizeof: {}", chunk_sizeof);
    println!("SubChunk sizeof: {}", sub_chunk_sizeof);
    println!("For a whole loaded region: {}", 32 * 32 * (chunk_sizeof + 16 * sub_chunk_sizeof));
    println!("========================");
    println!();

    println!("====== ANVIL TEST ======");

    let env = Arc::new(LevelEnv::new_vanilla().unwrap());
    let source = AnvilLevelSourceBuilder::new(r"C:\Users\Theo\AppData\Roaming\.minecraft\saves\Amplified Test").unwrap();
    let mut level = Level::new("overworld".to_string(), env, source);

    println!("Level height: {:?}", level.get_height());

    level.request_chunk(0, 0);

    loop {

        println!("Loading...");
        level.load_chunks();

        if let Some(chunk) = level.get_storage().get_chunk(0, 0) {
            println!("Chunk loaded.");
            break
        }

        println!("Chunk not loaded, waiting 1 second.");
        std::thread::sleep(Duration::from_secs(1));

    }




    /*let level_dat_file = File::open(r"C:\Users\Theo\AppData\Roaming\.minecraft\saves\Amplified Test\level.dat").unwrap();
    let mut level_dat_file = GzDecoder::new(level_dat_file);
    let level_tag = read_compound_tag(&mut level_dat_file);

    println!("Level tag: {:?}", level_tag);

    let mut region_file = RegionFile::new(
        PathBuf::from(r"C:\Users\Theo\AppData\Roaming\.minecraft\saves\Test JDataPack\DIM1\region_copy"),
        0,
        0
    ).unwrap();

    let mut reader = region_file.get_chunk_reader(0, 0).unwrap();
    let tag = read_compound_tag(&mut reader).unwrap();

    println!("Chunk tag: {:?}", tag);*/

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
