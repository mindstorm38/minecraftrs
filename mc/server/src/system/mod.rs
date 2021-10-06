use std::collections::hash_map::Entry;
use std::fmt::{Debug, Formatter};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::SocketAddr;

use mc_runtime::world::{World, WorldSystemExecutor};
use mc_runtime::util::{EventTracker, EventIterator};

use crate::packet::{PacketServer, Event, RawPacket};
use crate::protocol::{ClientState, ReadablePacket, RawReader, WritablePacket};
use crate::protocol::handshake::HandshakePacket;
use crate::protocol::status::RequestStatusPacket;


/// This structure keeps track of a specific client.
pub struct ProtocolClient {
    state: ClientState
}

/// The main server component that is registered in the `World` using `register_systems`.
/// Use this structure to register handy listeners that will decode packets before call.
pub struct ProtocolServer {
    server: PacketServer,
    clients: HashMap<SocketAddr, ProtocolClient>,
    packets_readers: HashMap<(ClientState, u16), Box<dyn ReadableToEventPacket>>,
}

impl ProtocolServer {

    pub fn register_packet_reader<P>(&mut self, state: ClientState, id: u16)
    where
        P: ReadablePacket + 'static
    {
        self.packets_readers.insert((state, id), Box::new(ReadableToEventPacketWrapper::<P> {
            phantom: PhantomData
        }));
    }

    pub fn send_packet<P>(&self, addr: SocketAddr, id: u16, packet: &mut P)
    where
        P: WritablePacket
    {
        let mut raw_packet = RawPacket::blank(addr, id);
        packet.write_packet(&mut raw_packet.data);
        self.server.send(raw_packet);
    }

    pub fn get_client(&self, addr: SocketAddr) -> Option<&ProtocolClient> {
        self.clients.get(&addr)
    }

    pub fn get_client_mut(&mut self, addr: SocketAddr) -> Option<&mut ProtocolClient> {
        self.clients.get_mut(&addr)
    }

}


/// A wrapper for a packet that also contains the sender's socket address. All packets
/// are wrapped into this into the event tracker.
pub struct PacketEvent<P> {
    pub packet: P,
    pub addr: SocketAddr
}

impl<P: Debug> Debug for PacketEvent<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketEvent")
            .field("addr", &self.addr)
            .field("packet", &self.packet)
            .finish()
    }
}

// Extension to EventTracker for polling packets directly.

pub trait EventTrackerPacketExt {
    fn poll_packets<P: 'static>(&self) -> EventIterator<PacketEvent<P>>;
}

impl EventTrackerPacketExt for EventTracker {
    fn poll_packets<P: 'static>(&self) -> EventIterator<PacketEvent<P>> {
        self.poll_events::<PacketEvent<P>>()
    }
}

// Internal dynamic dispatch wrapper //

/// Internal trait to allow dynamic dispatch over a generic reader.
trait ReadableToEventPacket {
    fn read_packet_to_event(&mut self, raw_packet: &mut RawPacket, event_tracker: &mut EventTracker);
}

/// Internal structure that is the only implementor of `ReadableToEventPacket`.
struct ReadableToEventPacketWrapper<P> {
    phantom: PhantomData<*const P>
}

impl<P: ReadablePacket + 'static> ReadableToEventPacket for ReadableToEventPacketWrapper<P> {
    fn read_packet_to_event(&mut self, raw_packet: &mut RawPacket, event_tracker: &mut EventTracker) {
        if let Ok(packet) = P::read_packet(&mut raw_packet.data) {
            event_tracker.push_event(PacketEvent {
                packet,
                addr: raw_packet.addr
            });
        }
    }
}


/// Main system of the packet server. It receive and dispatch events to the world.
fn system_packet_server(world: &mut World) {

    // Here we cannot use directly get_component_mut from World, because this would prevent the
    // borrow checker from allowing us to mutably reference event tracker at the same time.
    let mut proto_server = world.components.get_mut::<ProtocolServer>().unwrap();
    let event_tracker = &mut world.event_tracker;

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
                    let packet_id = (client.state, packet.id);
                    if let Some(reader) = proto_server.packets_readers.get_mut(&packet_id) {
                        reader.read_packet_to_event(&mut packet, event_tracker);
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


fn system_packet_core(world: &mut World) {

    let mut proto_server = world.components.get_mut::<ProtocolServer>().unwrap();

    for event in world.event_tracker.poll_packets::<HandshakePacket>() {
        if let Some(client) = proto_server.get_client_mut(event.addr) {
            // TODO: Check restrictions, no downgrade, no 'play' status.
            client.state = event.packet.next_state;
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
        packets_readers: HashMap::new(),
    };

    server.register_packet_reader::<HandshakePacket>(ClientState::Handshake, 0x00);
    server.register_packet_reader::<RequestStatusPacket>(ClientState::Status, 0x00);

    world.insert_component(server);

    executor.add_system(system_packet_server);
    executor.add_system(system_packet_core);

}
