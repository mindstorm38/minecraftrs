use mc_core::world::level::{LevelEnv, Level};
use mc_core::world::chunk::{Chunk, SubChunk, ChunkHeight};
use mc_core::world::anvil::source::{AnvilLevelSource};
use mc_vanilla::ext::WithVanilla;

use std::mem::size_of;
use std::sync::Arc;
use std::time::Duration;


/// This example can be run to test which block states or properties are not properly decoded
/// from a debug world. This example currently supports 1.18.1.
///
/// Put the path to the directory of the world into the 'MCRS_LEVEL_DIR' environment variable.
fn main() {

    println!("===== MEMORY USAGE (THEORY) =====");
    let chunk_sizeof = size_of::<Chunk>();
    let sub_chunk_sizeof = size_of::<SubChunk>();
    println!("Chunk height sizeof: {}", size_of::<ChunkHeight>());
    println!("Level sizeof: {}", size_of::<Level>());
    println!("Chunk sizeof: {}", chunk_sizeof);
    println!("SubChunk sizeof: {}", sub_chunk_sizeof);
    println!("For a whole loaded region: {}", 32 * 32 * (chunk_sizeof + 16 * sub_chunk_sizeof));
    println!("=================================");
    println!();

    // Sleep to have the time to analyse memory consumption.
    std::thread::sleep(Duration::from_secs(5));

    println!("====== ANVIL TEST ======");

    let level_dir = std::env::var("MCRS_LEVEL_DIR").unwrap();
    let env = Arc::new(LevelEnv::with_vanilla());
    let source = AnvilLevelSource::new(level_dir);
    let height = ChunkHeight {
        min: -4,
        max: 19
    };

    let mut level = Level::new("overworld".to_string(), env, height, source);

    for cx in 0..=31 {
        for cz in 0..=31 {
            level.request_chunk_load(cx, cz);
        }
    }

    level.load_chunks_blocking();

    println!("========================");
    println!();

    // Sleep to have the time to analyse memory consumption.
    std::thread::sleep(Duration::from_secs(5));

    // For 1.18.1 debug world, the program takes 12Mo of RAM to load the world.

    let chunks_count = level.chunks.get_chunks_count();
    let sub_chunks_count: usize = level.chunks.iter_chunks().map(|c| {
        c.read().unwrap().iter_loaded_sub_chunks().count()
    }).sum();

    println!("===== MEMORY USAGE =====");
    println!("Level height: {:?}", level.get_height());
    println!("Chunks count: {}", chunks_count);
    println!("Sub chunks count: {}", sub_chunks_count);
    println!("========================");

}
