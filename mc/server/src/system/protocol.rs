use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::SocketAddr;

use mc_runtime::world::{World, WorldSystemExecutor};
use mc_core::world::level::BaseEntity;
use mc_vanilla::util::GameMode;
use mc_vanilla::entity::PLAYER;

use crate::packet::{PacketServer, Event, RawPacket};
use crate::protocol::{ClientState, ReadablePacket, WritablePacket, PacketResult};

use crate::protocol::handshake::HandshakePacket;
use crate::protocol::status::{RequestStatusPacket, StatusPacket, PingPacket};
use crate::protocol::login::{LoginStartPacket, LoginSuccessPacket};
use crate::protocol::play::{JoinGamePacket, SpawnPositionPacket, PlayerAbilitiesPacket, PlayerPosAndLook, PluginMessage};

use hecs::Entity;
use uuid::Uuid;


/// This structure keeps track of a specific client.
pub struct ProtocolClient {
    /// The socket address of the client.
    addr: SocketAddr,
    /// The client protocol state, this is an really important information with the packet ID,
    /// but the state is not sent with it, so we must track it.
    state: ClientState,
    /// Optional profile when logged-in.
    profile: Option<PlayProfile>
}

/// The profile of the client once in play state.
pub struct PlayProfile {
    level_idx: usize,
    entity: Entity,
    username: String,
    uuid: Uuid
}

/// An entity component for player entities. It contains additional information
/// about player's connection and chunk positions used for updating player's view.
pub struct ProtocolPlayerEntity {
    pub addr: SocketAddr,
    pub chunk_pos: (i32, i32),
    pub last_chunk_pos: Option<(i32, i32)>
}

/// The main server component that is registered in the `World` using `register_systems`.
/// Use this structure to register handy listeners that will decode packets before call.
pub struct ProtocolServer {
    /// Internal TCP packet server, raw packets are fetched from it.
    server: PacketServer,
    /// Mapping all client's addresses to a structure storing their state.
    clients: HashMap<SocketAddr, ProtocolClient>,
    /// Packet listeners.
    packet_listeners: HashMap<(ClientState, u16), Vec<Box<dyn PacketListener>>>
}

impl ProtocolServer {

    #[inline]
    pub fn send_packet<P>(&self, addr: SocketAddr, id: u16, packet: &mut P)
    where
        P: WritablePacket
    {
        self.server.send(write_packet(addr, id, packet).unwrap());
    }

    pub fn get_client(&self, addr: SocketAddr) -> Option<&ProtocolClient> {
        self.clients.get(&addr)
    }

    pub fn get_client_mut(&mut self, addr: SocketAddr) -> Option<&mut ProtocolClient> {
        self.clients.get_mut(&addr)
    }

    pub fn add_listener<F, P>(&mut self, state: ClientState, id: u16, func: F)
    where
        F: FnMut(PacketEvent<P>) + 'static,
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


/// The wrapper used when calling back a packet listener, this wrapper allows you to
/// access the decoded packet, read or mutate the client and to send response packet
/// to anyone (there is a shortcut method to send a response to the sender).
pub struct PacketEvent<'a, 'b, P> {
    pub world: &'a World,
    pub client: &'b mut ProtocolClient,
    server: &'b PacketServer,
    pub packet: P,
}

impl<'a, 'b, P> PacketEvent<'a, 'b, P> {

    #[inline]
    pub fn send_packet<R>(&self, addr: SocketAddr, id: u16, packet: &mut R)
    where
        R: WritablePacket
    {
        self.server.send(write_packet(addr, id, packet).unwrap());
    }

    #[inline]
    pub fn answer_packet<R>(&self, id: u16, packet: &mut R)
    where
        R: WritablePacket
    {
        self.send_packet(self.client.addr, id, packet);
    }

}


/// Internal function to write a packet to a raw packet.
fn write_packet<P>(addr: SocketAddr, id: u16, packet: &mut P) -> PacketResult<RawPacket>
where
    P: WritablePacket
{
    let mut raw_packet = RawPacket::blank(addr, id);
    packet.write_packet(raw_packet.get_cursor_mut())?;
    Ok(raw_packet)
}


/// Internal generic wrapper structure for packet listener.
struct PacketListenerWrapper<F, P> {
    func: F,
    phantom: PhantomData<*const P>
}

/// Internal trait used for dynamic dispatching to generic `PacketListenerWrapper`.
trait PacketListener {
    fn accept_packet<'a, 'b>(&mut self, world: &'a World, server: &'b PacketServer, client: &'b mut ProtocolClient, raw_packet: &RawPacket);
}

impl<F, P> PacketListener for PacketListenerWrapper<F, P>
where
    F: FnMut(PacketEvent<P>),
    P: ReadablePacket
{
    fn accept_packet<'a, 'b>(&mut self, world: &'a World, server: &'b PacketServer, client: &'b mut ProtocolClient, raw_packet: &RawPacket) {
        // TODO: We should not unwrap un the future.
        let packet = P::read_packet(raw_packet.get_cursor()).unwrap();
        (self.func)(PacketEvent {
            world,
            client,
            server,
            packet
        });
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
                    addr,
                    state: ClientState::Handshake,
                    profile: None
                });
            }
            Event::Packet(packet) => {
                // We must temporarily get a real mutable reference because RefMut seems to
                // prevent multiple mutable reference to multiple structure's fields.
                let proto_server = &mut *proto_server;
                let client = proto_server.clients.get_mut(&packet.addr).unwrap();
                if let Some(listeners) = proto_server.packet_listeners.get_mut(&(client.state, packet.id)) {
                    for listener in listeners {
                        listener.accept_packet(world, &proto_server.server, client, &packet);
                    }
                }
            }
            Event::Disconnected(addr) => {
                println!("[{}] Disconnected.", addr);
                let client = proto_server.clients.remove(&addr).unwrap();
                if let Some(ref play_profile) = client.profile {
                    world.levels[play_profile.level_idx].borrow_mut().entities.remove_entity(play_profile.entity);
                }
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

    server.add_listener::<_, HandshakePacket>(ClientState::Handshake, 0x00, |e| {
        if let ClientState::Status | ClientState::Login = e.packet.next_state {
            e.client.state = e.packet.next_state;
        }
    });

    server.add_listener::<_, RequestStatusPacket>(ClientState::Status, 0x00, |e| {
        e.answer_packet(0x00, &mut StatusPacket {
            game_version: "1.16.5",
            protocol_version: 754,
            max_players: 10,
            online_players: 0,
            description: "Minecraft Rust server".to_string()
        });
    });

    server.add_listener::<_, PingPacket>(ClientState::Status, 0x01, |e| {
        e.answer_packet(0x01, &mut e.packet.get_pong());
    });

    server.add_listener::<_, LoginStartPacket>(ClientState::Login, 0x00, |e| {

        println!("[{}] Login: {}", e.client.addr, e.packet.username);

        let profile = {

            let mut level = e.world.levels[0].borrow_mut();
            let entity = level.spawn_entity(&PLAYER, Default::default()).unwrap();

            level.entities.ecs.insert_one(entity, ProtocolPlayerEntity {
                addr: e.client.addr,
                chunk_pos: (0, 0),
                last_chunk_pos: None
            });

            let base_entity = level.entities.ecs.get::<BaseEntity>(entity).unwrap();

            PlayProfile {
                level_idx: 0,
                entity,
                username: e.packet.username.clone(),
                uuid: base_entity.uuid
            }

        };

        e.answer_packet(0x02, &mut LoginSuccessPacket {
            username: profile.username.clone(),
            uuid: profile.uuid
        });

        e.client.state = ClientState::Play;
        e.client.profile = Some(profile);

        e.answer_packet(0x24, &mut JoinGamePacket {
            eid: 1234,
            hardcore: false,
            game_mode: GameMode::Survival,
            last_game_mode: None,
            world: e.world,
            level_index: 0,
            hashed_seed: 0,
            view_distance: 8
        });

        e.answer_packet(0x17, &mut PluginMessage::Brand("MinecraftRS".to_string()));

        e.answer_packet(0x42, &mut SpawnPositionPacket {
            pos: Default::default()
        });

        e.answer_packet(0x30, &mut PlayerAbilitiesPacket {
            invulnerable: true,
            flying: true,
            allow_flying: true,
            instant_break: true,
            flying_speed: 0.05,
            fov_modifier: 0.1
        });

        e.answer_packet(0x34, &mut PlayerPosAndLook {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            rel_x: false,
            rel_y: false,
            rel_z: false,
            rel_yaw: false,
            rel_pitch: false,
            tp_id: 0
        });

    });

    world.insert_component(server);

    executor.add_system(system_packet_server);

}
