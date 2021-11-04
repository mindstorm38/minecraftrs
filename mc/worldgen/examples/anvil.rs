use std::time::Duration;
use std::sync::Arc;

use mc_core::world::source::LoadOrGenLevelSource;
use mc_core::world::anvil::source::AnvilLevelSource;
use mc_core::world::level::{Level, LevelEnv};
use mc_core::world::chunk::ChunkHeight;

use mc_worldgen::gen::legacy::LegacyGenLevelSource;
use mc_worldgen::gen::r102::R102Provider;

use mc_vanilla::ext::WithVanilla;

fn main() {

    unsafe {
        mc_core::perf::enable();
    }

    let region_dir = std::env::var("REGION_DIR").expect("Missing region dir");
    let anvil_source = AnvilLevelSource::new(region_dir);

    let gen_provider = R102Provider::new(3048926232851431861);
    let gen_source = LegacyGenLevelSource::new(gen_provider, 4);

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
    let range = 8;

    for cx in (center_x - range)..(center_x + range) {
        for cz in (center_z - range)..(center_z + range) {
            level.request_chunk_load(cx, cz);
        }
    }

    loop {
        level.load_chunks();
        std::thread::sleep(Duration::from_millis(1));
    }

}
