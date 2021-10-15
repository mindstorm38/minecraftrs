use mc_runtime::world::WorldContext;
use mc_server::packet::PacketServer;

use mc_core::world::source::{LevelGeneratorSource, SuperFlatGenerator};
use mc_core::world::level::{Level, LevelEnv};
use mc_core::world::chunk::ChunkHeight;

use mc_vanilla::ext::WithVanilla;
use mc_vanilla::block::*;
use std::sync::Arc;

fn main() {

    let server = PacketServer::bind("0.0.0.0", 25565).unwrap();

    let mut super_flat = SuperFlatGenerator::new();
    super_flat.add_layer(BEDROCK.get_default_state(), 0, 1);
    super_flat.add_layer(DIRT.get_default_state(), 1, 3);
    super_flat.add_layer(GRASS.get_default_state(), 4, 1);

    let super_flat_source = LevelGeneratorSource::new(super_flat, 1);

    let env = Arc::new(LevelEnv::with_vanilla());
    let mut level = Level::new("minecraft:overworld".to_string(), env, ChunkHeight::new(0, 15), super_flat_source);

    for cx in -2..2 {
        for cz in -2..2 {
            level.request_chunk_load(cx, cz);
        }
    }

    let mut ctx = WorldContext::new();
    ctx.world.add_level(level);
    ctx.world.insert_component(server);
    ctx.register(mc_server::system::register_systems);
    ctx.run_simple();

}
