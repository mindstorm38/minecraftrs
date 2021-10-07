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
    state: ClientState,
    /// Pointing to `ProtocolServer.packets`, this is used mostly when state is changed in order
    /// to relocate packets to the right vector.
    packets: Vec<(u16, usize)>
}

/// The main server component that is registered in the `World` using `register_systems`.
/// Use this structure to register handy listeners that will decode packets before call.
pub struct ProtocolServer {
    /// Internal TCP packet server, raw packets are fetched from it.
    server: PacketServer,
    /// Mapping all client's addresses to a structure storing their state.
    clients: HashMap<SocketAddr, ProtocolClient>,
    packets: HashMap<(ClientState, u16), Vec<Option<RawPacket>>>,
    // packets_readers: HashMap<(ClientState, u16), Box<dyn ReadableToEventPacket>>,
}

impl ProtocolServer {

    /*pub fn register_packet_reader<P>(&mut self, state: ClientState, id: u16)
    where
        P: ReadablePacket + 'static
    {
        self.packets_readers.insert((state, id), Box::new(ReadableToEventPacketWrapper::<P> {
            phantom: PhantomData
        }));
    }*/

    pub fn send_packet<P>(&self, addr: SocketAddr, id: u16, packet: &mut P)
    where
        P: WritablePacket
    {
        let mut raw_packet = RawPacket::blank(addr, id);
        packet.write_packet(raw_packet.get_cursor_mut());
        self.server.send(raw_packet);
    }

    pub fn pop_packet<P>(&mut self, state: ClientState, id: u16) -> Option<(SocketAddr, P)>
    where
        P: ReadablePacket
    {
        match self.packets.get_mut(&(state, id)) {
            None => None,
            Some(packets) => {
                loop {
                    match packets.pop() {
                        Some(Some(packet)) => {
                            // TODO: Should not unwrap in the future.
                            break Some((
                                packet.addr,
                                P::read_packet(packet.get_cursor()).unwrap()
                            ))
                        }
                        Some(None) => (), // Just loop
                        None => break None
                    }
                }
            }
        }
    }

    pub fn iter_packets<P>(&self, state: ClientState, id: u16) -> PacketIterator<P>
    where
        P: ReadablePacket
    {
        PacketIterator {
            packets_it: self.packets.get(&(state, id)).map(|v| v.iter()),
            phantom: PhantomData
        }
    }

    pub fn get_client(&self, addr: SocketAddr) -> Option<&ProtocolClient> {
        self.clients.get(&addr)
    }

    pub fn get_client_mut(&mut self, addr: SocketAddr) -> Option<&mut ProtocolClient> {
        self.clients.get_mut(&addr)
    }

    // Internal //

    fn insert_raw_packet_internal(
        state: ClientState,
        packets: &mut HashMap<(ClientState, u16), Vec<Option<RawPacket>>>,
        packet: RawPacket
    ) -> usize {
        let packets = match packets.entry((state, packet.id)) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Vec::new())
        };
        let idx = packets.len();
        packets.push(Some(packet));
        idx
    }

    fn insert_raw_packet(&mut self, packet: RawPacket) {
        // SAFETY: We unwrap because the caller must ensure that the client is connected.
        // If it's not the case we want to panic.
        let client = self.clients.get_mut(&packet.addr).unwrap();
        let packet_id = packet.id;
        let idx = Self::insert_raw_packet_internal(client.state, &mut self.packets, packet);
        client.packets.push((packet_id, idx));
    }

    fn change_client_state(&mut self, addr: SocketAddr, state: ClientState) {
        // SAFETY: Check insert_raw_packet
        let client = self.clients.get_mut(&addr).unwrap();
        if client.state != state {
            let old_state = client.state;
            client.state = state;
            for (id, idx) in &mut client.packets {
                // SAFETY: We unwrap because we want to panic if wrong indices are being stored.
                // Note that the .take() should also return `Some`, we check it in debug.
                let raw_packet = self.packets.get_mut(&(old_state, *id))
                    .unwrap()[*idx].take().unwrap();
                *idx = Self::insert_raw_packet_internal(state, &mut self.packets, raw_packet);
            }
        }
    }

}


/// A packet iterator returned from `ProtocolServer::poll_packets`.
pub struct PacketIterator<'a, P> {
    packets_it: Option<Iter<'a, Option<RawPacket>>>,
    phantom: PhantomData<*const P>
}

impl<'a, P: ReadablePacket> Iterator for PacketIterator<'a, P> {

    type Item = (SocketAddr, P);

    fn next(&mut self) -> Option<Self::Item> {
        match self.packets_it {
            None => None,
            Some(ref mut it) => {
                loop {
                    match it.next() {
                        Some(Some(packet)) => {
                            // TODO: For now I unwrap, but we should in the
                            //       future return the result.
                            break Some((
                                packet.addr,
                                P::read_packet(packet.get_cursor()).unwrap()
                            ))
                        }
                        Some(None) => (), // Go to the next
                        None => break None
                    }
                }
            }
        }
    }

}


/*/// A wrapper for a packet that also contains the sender's socket address. All packets are wrapped
/// into this into the event tracker. You can use the `EventTrackerPacketExt` extension trait for
/// `EventTracker` to avoid writing `PacketEvent<YourPacket>` every time.
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

/// An extension trait that is only implemented to `EventTracker` and is a shortcut
/// for calling `poll_events` with the given generic parameter `P` wrapped into
/// `PacketEvent<P>`. Read `PacketEvent`'s doc for more information.
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
}*/


/// Main system of the packet server. It receive and dispatch events to the world.
fn system_packet_server(world: &mut World) {

    // Here we cannot use directly get_component_mut from World, because this would prevent the
    // borrow checker from allowing us to mutably reference event tracker at the same time.
    let mut proto_server = world.components.get_mut::<ProtocolServer>().unwrap();

    for client in proto_server.clients.values_mut() {
        client.packets.clear();
    }

    for raw_packets in proto_server.packets.values_mut() {
        raw_packets.clear();
    }

    while let Some(event) = proto_server.server.try_recv_event() {
        match event {
            Event::Connected(addr) => {
                println!("[{}] Connected.", addr);
                proto_server.clients.insert(addr, ProtocolClient {
                    state: ClientState::Handshake,
                    packets: Vec::new()
                });
            }
            Event::Packet(packet) => {
                proto_server.insert_raw_packet(packet);
                /*// We must temporarily get a real mutable reference because RefMut seems to
                // prevent multiple mutable reference to multiple structure's fields.
                let proto_server = &mut *proto_server;
                if let Some(client) = proto_server.clients.get_mut(&packet.addr) {
                    let packets = match proto_server.packets.entry((client.state, packet.id)) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(Vec::new())
                    };
                    client.packets.push((packet.id, packets.len()));
                    packets.push(Some(packet));
                }*/
                /*
                if let Some(client) = proto_server.clients.get(&packet.addr) {
                    println!("[{}] [{:?}] packet id: {}", packet.addr, client.state, packet.id);
                    let packet_id = (client.state, packet.id);
                    if let Some(reader) = proto_server.packets_readers.get_mut(&packet_id) {
                        reader.read_packet_to_event(&mut packet, event_tracker);
                    }
                }*/
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

    while let Some((
        addr,
        packet
    )) = proto_server.pop_packet::<HandshakePacket>(ClientState::Handshake, 0x00) {
        println!("[{}] New state: {:?}", addr, packet.next_state);
        proto_server.change_client_state(addr, packet.next_state);
    }

    /*let mut states_changes = Vec::new();
    for (addr, packet) in proto_server.iter_packets::<HandshakePacket>(ClientState::Handshake, 0x00) {
        println!("[{}] New state: {:?}", addr, packet.next_state);
        states_changes.push((addr, packet.next_state));
    }
    for (addr, state) in states_changes {
        proto_server.change_client_state(addr, state);
    }*/

    /*for event in world.event_tracker.poll_packets::<HandshakePacket>() {
        if let Some(client) = proto_server.get_client_mut(event.addr) {
            println!("[{}] New state: {:?}", event.addr, event.packet.next_state);
            // TODO: Check restrictions, no downgrade, no 'play' status.
            client.state = event.packet.next_state;
        }
    }

    for event in world.event_tracker.poll_packets::<RequestStatusPacket>() {
        println!("status requested from {}", event.addr);
    }*/

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
        packets: HashMap::new(),
        // packets_readers: HashMap::new(),
    };

    // server.register_packet_reader::<HandshakePacket>(ClientState::Handshake, 0x00);
    // server.register_packet_reader::<RequestStatusPacket>(ClientState::Status, 0x00);

    world.insert_component(server);

    executor.add_system(system_packet_server);
    executor.add_system(system_packet_core);

}
