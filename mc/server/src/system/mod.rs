use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::vec::Drain;

use mc_runtime::world::{World, WorldSystemExecutor};

use crate::packet::{PacketServer, Event, RawPacket};
use crate::protocol::{ClientState, ReadablePacket, RawReader, WritablePacket};
use crate::protocol::handshake::HandshakePacket;


/// This structure keeps track of a specific client.
struct ProtocolClient {
    state: ClientState
}

/// The main server component that is registered in the `World` using `register_systems`.
/// Use this structure to register handy listeners that will decode packets before call.
pub struct ProtocolServer {
    server: PacketServer,
    clients: HashMap<SocketAddr, ProtocolClient>,
    packets: HashMap<(ClientState, u16), Vec<RawPacket>>,
}

impl ProtocolServer {

    pub fn send_packet<P>(&self, addr: SocketAddr, id: u16, packet: &mut P)
    where
        P: WritablePacket
    {
        let mut raw_packet = RawPacket::blank(addr, id);
        packet.write_packet(&mut raw_packet.data);
        self.server.send(raw_packet);
    }

    pub fn poll_packet<P>(&mut self, state: ClientState, id: u16) -> Option<PacketHandle<P>>
    where
        P: ReadablePacket
    {
        match self.packets.get_mut(&(state, id)) {
            None => None,
            Some(packets) => {
                match packets.pop() {
                    None => None,
                    Some(mut raw_packet) => {
                        match P::read_packet(&mut raw_packet.data) {
                            Ok(packet) => Some(PacketHandle {
                                server: self,
                                packet,
                                addr: raw_packet.addr
                            }),
                            Err(_) => None
                        }
                    }
                }
            }
        }
    }

    fn get_client_mut(&mut self, addr: SocketAddr) -> Option<&mut ProtocolClient> {
        self.clients.get_mut(&addr)
    }

}


/// A handle to a decoded packet, it can be useful to know sender's address of the packet
/// and/or to answer to it directly with a writable packet.
pub struct PacketHandle<'a, P> {
    pub server: &'a mut ProtocolServer,
    pub packet: P,
    pub addr: SocketAddr
}

impl<'a, P> PacketHandle<'a, P> {
    pub fn answer_packet<R>(&self, id: u16, packet: &mut R)
    where
        R: WritablePacket
    {
        self.server.send_packet(self.addr, id, packet);
    }
}


/// Main system of the packet server. It receive and dispatch events to the world.
fn system_packet_server(world: &mut World) {

    let mut proto_server = world.get_component_mut::<ProtocolServer>().unwrap();

    while let Some(event) = proto_server.server.try_recv_event() {
        match event {
            Event::Connected(addr) => {
                println!("[{}] Connected.", addr);
                proto_server.clients.insert(addr, ProtocolClient {
                    state: ClientState::Handshake
                });
            }
            Event::Packet(mut packet) => {
                // We must temporarily get a real mutable reference because RefMut seems to
                // prevent multiple mutable reference to multiple structure's fields.
                let proto_server = &mut *proto_server;
                if let Some(client) = proto_server.clients.get(&packet.addr) {
                    match proto_server.packets.entry((client.state, packet.id)) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => {
                            v.insert(Vec::new())
                        }
                    }.push(packet);
                }
            }
            Event::Disconnected(addr) => {
                println!("[{}] Disconnected.", addr);
                proto_server.clients.remove(&addr);
            }
        }
    }

}


fn system_packet_core(world: &mut World) {

    let mut proto_server = world.get_component_mut::<ProtocolServer>().unwrap();

    while let Some(packet) = proto_server.poll_packet::<HandshakePacket>(ClientState::Handshake, 0x00) {
        println!("{:?}", packet.packet);
        packet.server.get_client_mut(packet.addr).unwrap().state = packet.packet.next_state;
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
        packets: HashMap::new()
    };

    /*server.add_listener(ClientState::Handshake, 0x00, |client, event: HandshakePacket| {
        println!("handshake received: {:?}", event);
    });*/

    world.insert_component(server);

    executor.add_system(system_packet_server);
    executor.add_system(system_packet_core);

}
