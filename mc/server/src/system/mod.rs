use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::slice::Iter;
use std::fmt::Debug;

use mc_runtime::world::{World, WorldSystemExecutor};

use crate::packet::{PacketServer, Event, RawPacket};
use crate::protocol::{ClientState, ReadablePacket, WritablePacket};
use crate::protocol::handshake::HandshakePacket;
use crate::protocol::status::RequestStatusPacket;


/// This structure keeps track of a specific client.
pub struct ProtocolClient {
    /// The client protocol state, this is an really important information with the packet ID,
    /// but the state is not sent with it, so we must track it.
    state: ClientState
}

/// The main server component that is registered in the `World` using `register_systems`.
/// Use this structure to register handy listeners that will decode packets before call.
pub struct ProtocolServer {
    /// Internal TCP packet server, raw packets are fetched from it.
    server: PacketServer,
    /// Mapping all client's addresses to a structure storing their state.
    clients: HashMap<SocketAddr, ProtocolClient>,
    packet_listeners: HashMap<(ClientState, u16), Vec<Box<dyn PacketListener>>>
}

impl ProtocolServer {

    pub fn get_client(&self, addr: SocketAddr) -> Option<&ProtocolClient> {
        self.clients.get(&addr)
    }

    pub fn get_client_mut(&mut self, addr: SocketAddr) -> Option<&mut ProtocolClient> {
        self.clients.get_mut(&addr)
    }

    pub fn add_listener<F, P>(&mut self, state: ClientState, id: u16, func: F)
    where
        F: FnMut(&World, &mut ProtocolClient, P) + 'static,
        P: ReadablePacket + 'static
    {
        match self.packet_listeners.entry((state, id)) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Vec::new())
        }.push(Box::new(PacketListenerWrapper {
            func,
            phantom: PhantomData
        }));
    }

}


struct PacketListenerWrapper<F, P> {
    func: F,
    phantom: PhantomData<*const P>
}

trait PacketListener {
    fn accept_packet(&mut self, world: &World, client: &mut ProtocolClient, raw_packet: &RawPacket);
}

impl<F, P> PacketListener for PacketListenerWrapper<F, P>
where
    F: FnMut(&World, &mut ProtocolClient, P),
    P: ReadablePacket
{
    fn accept_packet(&mut self, world: &World, client: &mut ProtocolClient, raw_packet: &RawPacket) {
        // TODO: We should not unwrap un the future.
        let packet = P::read_packet(raw_packet.get_cursor()).unwrap();
        (self.func)(world, client, packet);
    }
}


/// Main system of the packet server. It receive and dispatch events to the world.
fn system_packet_server(world: &mut World) {

    // Here we cannot use directly get_component_mut from World, because this would prevent the
    // borrow checker from allowing us to mutably reference event tracker at the same time.
    let mut proto_server = world.components.get_mut::<ProtocolServer>().unwrap();

    while let Some(event) = proto_server.server.try_recv_event() {
        match event {
            Event::Connected(addr) => {
                println!("[{}] Connected.", addr);
                proto_server.clients.insert(addr, ProtocolClient {
                    state: ClientState::Handshake
                });
            }
            Event::Packet(packet) => {
                // We must temporarily get a real mutable reference because RefMut seems to
                // prevent multiple mutable reference to multiple structure's fields.
                let proto_server = &mut *proto_server;
                let client = proto_server.clients.get_mut(&packet.addr).unwrap();
                if let Some(listeners) = proto_server.packet_listeners.get_mut(&(client.state, packet.id)) {
                    for listener in listeners {
                        listener.accept_packet(world, client, &packet);
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
/// must insert a `PacketServer` component to the World before calling this function,
/// this function will then remove `PacketServer` from components and move it into a
/// new component `ProtocolServer`. This component is then the main entry point for
/// protocol communications.
///
/// # Safety
/// This function will panic if component `PacketServer` is missing from the `World`.
pub fn register_systems(world: &mut World, executor: &mut WorldSystemExecutor) {

    let server = world.remove_component::<PacketServer>()
        .expect("Before register packet server system, you must add it as a component to the World.");

    let mut server = ProtocolServer {
        server,
        clients: HashMap::new(),
        packet_listeners: HashMap::new()
    };

    server.add_listener(ClientState::Handshake, 0x00, |_, client, packet: HandshakePacket| {
        println!("Client new state: {:?}", packet.next_state);
        client.state = packet.next_state;
    });

    world.insert_component(server);

    executor.add_system(system_packet_server);

}
