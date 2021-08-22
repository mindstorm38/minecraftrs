use mc_core::world::level::{LevelEnv, Level};
use mc_core::world::chunk::{Chunk, SubChunk, ChunkHeight};
use mc_core::world::anvil::source::{AnvilLevelSource, AnvilLevelSourceBuilder};
use mc_vanilla::ext::WithVanilla;

use std::mem::size_of;
use std::sync::Arc;
use std::time::Duration;


fn main() {

    println!("===== MEMORY USAGE =====");
    let chunk_sizeof = size_of::<Chunk>();
    let sub_chunk_sizeof = size_of::<SubChunk>();
    println!("Chunk height sizeof: {}", size_of::<ChunkHeight>());
    println!("Level sizeof: {}", size_of::<Level>());
    println!("Chunk sizeof: {}", chunk_sizeof);
    println!("SubChunk sizeof: {}", sub_chunk_sizeof);
    println!("For a whole loaded region: {}", 32 * 32 * (chunk_sizeof + 16 * sub_chunk_sizeof));
    println!("========================");
    println!();

    println!("====== ANVIL TEST ======");

    let env = Arc::new(LevelEnv::with_vanilla());
    let source = AnvilLevelSourceBuilder::new(r"C:\Users\Theo\AppData\Roaming\.minecraft\saves\Amplified Test").unwrap();
    let mut level = Level::new("overworld".to_string(), env, source);

    println!("Level height: {:?}", level.get_height());

    for cx in 0..2 {
        for cz in 0..2 {
            level.request_chunk(cx, cz);
        }
    }

    loop {
        level.load_chunks();
        std::thread::sleep(Duration::from_secs(1));
    }

}
