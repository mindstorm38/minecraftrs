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
    level.request_chunk(1, 0);
    level.request_chunk(2, 0);
    level.request_chunk(3, 0);
    level.request_chunk(31, 0);

    loop {

        level.load_chunks();

        if let Some(_) = level.get_storage().get_chunk(0, 0) {
            break
        }

        std::thread::sleep(Duration::from_secs(1));

    }

    println!("========================");

}
