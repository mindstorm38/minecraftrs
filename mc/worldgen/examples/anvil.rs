use std::time::Duration;
use std::sync::Arc;

use mc_core::world::source::{LoadOrGenLevelSource, LevelGeneratorSource};
use mc_core::world::anvil::source::AnvilLevelSource;
use mc_core::world::level::{Level, LevelEnv};
use mc_core::world::chunk::ChunkHeight;

use mc_worldgen::gen::release102::LevelGenRelease102;

use mc_vanilla::ext::WithVanilla;

fn main() {

    let region_dir = std::env::var("REGION_DIR").expect("Missing region dir");
    let anvil_source = AnvilLevelSource::new(region_dir);

    let gen_builder = LevelGenRelease102::builder(3048926232851431861);
    let gen_source = LevelGeneratorSource::new(gen_builder, 6);

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

    let center_x = -24;
    let center_z = 37;

    for cx in (center_x - 16)..(center_x + 16) {
        for cz in (center_z - 16)..(center_z + 16) {
            level.request_chunk_load(cx, cz);
        }
    }

    loop {

        level.load_chunks_with_callback(|cx, cz, chunk| {

        });

        std::thread::sleep(Duration::from_millis(1));

    }

}
