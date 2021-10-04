use mc_runtime::world::WorldContext;
use mc_server::packet::PacketServer;

fn main() {

    let server = PacketServer::bind("0.0.0.0", 25565).unwrap();

    let mut ctx = WorldContext::new();
    ctx.world.insert_component(server);
    ctx.register(mc_server::system::register_systems);
    ctx.run_simple();

}
