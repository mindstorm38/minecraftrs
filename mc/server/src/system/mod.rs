use std::collections::HashMap;
use std::net::SocketAddr;

use mc_runtime::world::{World, WorldSystemExecutor};

use crate::packet::{PacketServer, Event};
use crate::protocol::ClientState;



struct ProtocolServer {
    clients: HashMap<SocketAddr, ProtocolClient>
}

struct ProtocolClient {
    state: ClientState
}


/// Main system of the packet server. It receive and dispatch events to the world.
fn system_packet_server(world: &mut World) {

    let packet_server = world.get_component_mut::<PacketServer>().unwrap();
    let mut proto_server = world.get_component_mut::<ProtocolServer>().unwrap();

    while let Some(event) = packet_server.try_recv_event() {
        match event {
            Event::Connected(addr) => {
                println!("[{}] Connected.", addr);
                proto_server.clients.insert(addr, ProtocolClient {
                    state: ClientState::Handshake
                });
            }
            Event::Packet(packet) => {
                if let Some(client) = proto_server.clients.get(&packet.addr) {

                }
            }
            Event::Disconnected(addr) => {
                println!("[{}] Disconnected.", addr);
                proto_server.clients.remove(&addr);
            }
        }
    }

}

/// A function that you can use to register the system that runs packet server. You
/// must insert a `PacketServer` component to the World before calling this function.
///
/// # Safety
/// This function will panic if component `PacketServer` is missing from the `World`.
pub fn register_systems(world: &mut World, executor: &mut WorldSystemExecutor) {

    assert!(world.get_component::<PacketServer>().is_ok(),
            "Before register packet server system, you must add it as a component to the World.");

    world.insert_component(ProtocolServer {
        clients: HashMap::new()
    });

    executor.add_system(system_packet_server);

}
