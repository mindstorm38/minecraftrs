use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::SocketAddr;

use mc_runtime::world::{World, WorldSystemExecutor};

use crate::packet::{PacketServer, Event, RawPacket};
use crate::protocol::{ClientState, ReadablePacket, RawReader};
use crate::protocol::handshake::HandshakePacket;


struct ProtocolClient {
    state: ClientState
}

pub struct ProtocolServer {
    clients: HashMap<SocketAddr, ProtocolClient>,
    listeners: HashMap<(ClientState, u16), Box<dyn UntypedPacketListener>>
}

impl ProtocolServer {

    pub fn add_listener<R, F>(&mut self, state: ClientState, id: u16, func: F)
    where
        R: ReadablePacket + 'static,
        F: FnMut(SocketAddr, R) + 'static
    {
        self.listeners.insert((state, id), Box::new(PacketListener {
            func,
            phantom: PhantomData
        }));
    }

}



struct PacketListener<R, F> {
    func: F,
    phantom: PhantomData<*const R>
}

trait UntypedPacketListener {
    fn accept_packet(&mut self, addr: SocketAddr, src: &mut RawReader);
}

impl<R, F> UntypedPacketListener for PacketListener<R, F>
where
    R: ReadablePacket,
    F: FnMut(SocketAddr, R)
{
    fn accept_packet(&mut self, addr: SocketAddr, src: &mut RawReader) {
        match R::read_packet(src) {
            Ok(packet) => (self.func)(addr, packet),
            Err(_) => {}
        }
    }
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
            Event::Packet(mut packet) => {
                if let Some(client) = proto_server.clients.get(&packet.addr) {
                    let state = client.state;
                    if let Some(listener) = proto_server.listeners.get_mut(&(state, packet.id)) {
                        listener.accept_packet(packet.addr, &mut packet.data);
                    }
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

    let mut server = ProtocolServer {
        clients: HashMap::new(),
        listeners: HashMap::new()
    };

    server.add_listener(ClientState::Handshake, 0x00, |addr, event: HandshakePacket| {
        println!("handshake received: {:?}", event);
    });

    world.insert_component(server);

    executor.add_system(system_packet_server);

}
