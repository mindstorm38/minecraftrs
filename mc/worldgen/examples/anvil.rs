use std::time::Duration;
use std::sync::Arc;

use mc_core::world::source::{LoadOrGenLevelSource, LevelGeneratorSource, SuperFlatGenerator};
use mc_core::world::anvil::source::AnvilLevelSource;
use mc_core::world::level::{Level, LevelEnv};
use mc_core::world::chunk::ChunkHeight;

use mc_vanilla::ext::WithVanilla;
use mc_vanilla::block::*;

fn main() {

    let region_dir = std::env::var("REGION_DIR").expect("Missing region dir");
    let anvil_source = AnvilLevelSource::new(region_dir);

    let mut gen = SuperFlatGenerator::new();
    gen.add_layer(BEDROCK.get_default_state(), 0, 1);
    gen.add_layer(DIRT.get_default_state(), 1, 3);
    gen.add_layer(GRASS_BLOCK.get_default_state(), 3, 1);

    let gen_source = LevelGeneratorSource::new(gen, 1);

    let load_or_gen_source = LoadOrGenLevelSource::new(
        anvil_source,
        gen_source
    );

    let mut level = Level::new(
        "overworld".to_string(),
        Arc::new(LevelEnv::with_vanilla()),
        ChunkHeight::new(0, 15),
        load_or_gen_source
    );

    for cx in 0..16 {
        for cz in 0..16 {
            level.request_chunk_load(cx, cz);
        }
    }

    loop {

        level.load_chunks_with_callback(|cx, cz, chunk| {

        });

        std::thread::sleep(Duration::from_millis(1));

    }

}
