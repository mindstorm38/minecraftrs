use std::collections::HashMap;
use std::net::SocketAddr;

use mc_server::packet::{PacketServer, Event, RawPacket};
use mc_server::packet::serial::*;

use mc_server::protocol::{ClientState, Packet};
use mc_server::protocol::handshake::*;

use mc_core::pos::BlockPos;

use serde_json::json;
use uuid::Uuid;

use nbt::CompoundTag;


#[derive(Debug)]
struct Client {
    addr: SocketAddr,
    state: ClientState,
    profile: Option<ClientProfile>
}

#[derive(Debug)]
struct ClientProfile {
    username: String,
    uuid: Uuid,
    eid: i32
}


fn main() {

    let mut server = PacketServer::bind("0.0.0.0", 25565).unwrap();
    let mut clients = HashMap::new();
    let mut entity_id = 0;

    loop {

        use ClientState::*;

        match server.recv_event() {
            Event::Connected(addr) => {
                println!("[{}] Connected.", addr);
                clients.insert(addr, Client {
                    addr,
                    state: Handshake,
                    profile: None
                });
            }
            Event::Packet(mut packet) => {

                if let Some(client) = clients.get_mut(&packet.addr) {

                    println!("[{}] [{:?}] Packet#{} {:02X?}", packet.addr, client.state, packet.id, &packet.data.get_ref()[..]);

                    match (client.state, packet.id) {
                        (Handshake, 0x00) => {

                            let handshake = HandshakePacket::decode_raw(&mut packet).unwrap();

                            println!(" => protocol version: {}, addr: '{}', port: {}, next state: {:?}",
                                     handshake.protocol_version,
                                     handshake.server_addr,
                                     handshake.server_port,
                                     handshake.next_state);

                            if let Status | Login = handshake.next_state {
                                client.state = handshake.next_state;
                            } else {
                                print!(" => invalid next state");
                            }

                        }
                        (Status, 0x00) => {

                            let mut res_packet = packet.response(0x00);
                            let json_payload = json!({
                                        "version": {
                                            "name": "1.16.5",
                                            "protocol": 754
                                        },
                                        "players": {
                                            "max": 10,
                                            "online": 0,
                                            "sample": []
                                        },
                                        "description": {
                                            "text": "Test MinecraftRS server"
                                        }
                                    });

                            let json_payload_encoded = serde_json::to_string(&json_payload).unwrap();
                            res_packet.data.write_string(json_payload_encoded.as_str());
                            server.send_raw(res_packet);

                        }
                        (Status, 0x01) => {
                            let token = packet.data.read_i64().unwrap();
                            let mut res_packet = packet.response(0x01);
                            res_packet.data.write_i64(token).unwrap();
                            server.send_raw(res_packet);
                        }
                        (Login, 0x00) => {

                            let username = packet.data.read_string().unwrap();
                            if username.chars().count() > 16 {
                                println!(" => invalid username length");
                            } else {

                                println!(" => set username: {}", username);

                                let uuid = Uuid::new_v4();
                                let eid = entity_id;
                                entity_id += 1;

                                let mut login_success_packet = packet.response(0x02);
                                login_success_packet.data.write_uuid(&uuid).unwrap();
                                login_success_packet.data.write_string(username.as_str()).unwrap();
                                client.profile = Some(ClientProfile {
                                    username,
                                    uuid,
                                    eid
                                });
                                client.state = Play;
                                server.send_raw(login_success_packet);

                                let mut dimension_codec = CompoundTag::new();
                                dimension_codec.insert_compound_tag("minecraft:dimension_type", {
                                    let mut dimension_type = CompoundTag::new();
                                    dimension_type.insert_str("type", "minecraft:dimension_type");
                                    dimension_type.insert_compound_tag_vec("value", {
                                        let mut dimension_value = CompoundTag::new();
                                        dimension_value.insert_str("name", "minecraft:overworld");
                                        dimension_value.insert_i32("id", 0);
                                        dimension_value.insert_compound_tag("element", {
                                            let mut dimension_element = CompoundTag::new();
                                            dimension_element.insert_bool("piglin_safe", false);
                                            dimension_element.insert_bool("natural", true);
                                            dimension_element.insert_f32("ambient_light", 0.0);
                                            dimension_element.insert_str("infiniburn", "minecraft:infiniburn_overworld");
                                            dimension_element.insert_bool("respawn_anchor_works", false);
                                            dimension_element.insert_bool("has_skylight", true);
                                            dimension_element.insert_bool("bed_works", true);
                                            dimension_element.insert_str("effects", "minecraft:overworld");
                                            dimension_element.insert_bool("has_raids", true);
                                            dimension_element.insert_i32("logical_height", 256);
                                            dimension_element.insert_f32("coordinate_scale", 1.0);
                                            dimension_element.insert_bool("ultrawarm", false);
                                            dimension_element.insert_bool("has_ceiling", false);
                                            dimension_element

                                        });
                                        [dimension_value]
                                    });
                                    dimension_type
                                });
                                dimension_codec.insert_compound_tag("minecraft:worldgen/biome", {
                                    let mut biome_reg = CompoundTag::new();
                                    biome_reg.insert_str("type", "minecraft:worldgen/biome");
                                    biome_reg.insert_compound_tag_vec("value", {
                                        let mut biome_value = CompoundTag::new();
                                        biome_value.insert_str("name", "minecraft:the_void");
                                        biome_value.insert_i32("id", 127);
                                        biome_value.insert_compound_tag("element", {
                                            let mut biome_element = CompoundTag::new();
                                            biome_element.insert_str("precipitation", "none");
                                            biome_element.insert_compound_tag("effects", {
                                                let mut biome_effects = CompoundTag::new();
                                                biome_effects.insert_i32("sky_color", 8103167);
                                                biome_effects.insert_i32("water_fog_color", 329011);
                                                biome_effects.insert_i32("fog_color", 12638463);
                                                biome_effects.insert_i32("water_color", 4159204);
                                                biome_effects.insert_compound_tag("mood_sound", {
                                                    let mut biome_mood_sound = CompoundTag::new();
                                                    biome_mood_sound.insert_i32("tick_delay", 6000);
                                                    biome_mood_sound.insert_f64("offset", 2.0);
                                                    biome_mood_sound.insert_str("sound", "minecraft:ambient.cave");
                                                    biome_mood_sound.insert_i32("block_search_extent", 8);
                                                    biome_mood_sound
                                                });
                                                biome_effects
                                            });
                                            biome_element.insert_f32("depth", 0.1);
                                            biome_element.insert_f32("temperature", 0.5);
                                            biome_element.insert_f32("scale", 0.2);
                                            biome_element.insert_f32("downfall", 0.5);
                                            biome_element.insert_str("category", "none");
                                            biome_element
                                        });
                                        [biome_value]
                                    });
                                    biome_reg
                                });

                                let mut join_packet = packet.response(0x24);
                                join_packet.data.write_i32(eid);
                                join_packet.data.write_bool(false);
                                join_packet.data.write_i8(1);
                                join_packet.data.write_i8(-1);
                                join_packet.data.write_var_int(1);
                                join_packet.data.write_string("minecraft:overworld");
                                join_packet.data.write_nbt(&dimension_codec);
                                join_packet.data.write_nbt({
                                    dimension_codec.get_compound_tag("minecraft:dimension_type").unwrap()
                                        .get_compound_tag_vec("value").unwrap()[0]
                                        .get_compound_tag("element").unwrap()
                                });
                                join_packet.data.write_string("minecraft:overworld");
                                join_packet.data.write_i64(0);
                                join_packet.data.write_var_int(0);
                                join_packet.data.write_var_int(8);
                                join_packet.data.write_bool(false);
                                join_packet.data.write_bool(true);
                                join_packet.data.write_bool(false);
                                join_packet.data.write_bool(false);
                                server.send_raw(join_packet);

                                let mut spawn_packet = packet.response(0x42);
                                spawn_packet.data.write_block_pos(&BlockPos::new(0, 0, 0));
                                server.send_raw(spawn_packet);

                                let mut abilities_packet = packet.response(0x30);
                                abilities_packet.data.write_u8(1 | 2 | 4 | 8);
                                abilities_packet.data.write_f32(0.05);
                                abilities_packet.data.write_f32(0.1);
                                server.send_raw(abilities_packet);

                                let mut pos_packet = packet.response(0x34);
                                pos_packet.data.write_f64(0.0);
                                pos_packet.data.write_f64(0.0);
                                pos_packet.data.write_f64(0.0);
                                pos_packet.data.write_f32(0.0);
                                pos_packet.data.write_f32(0.0);
                                pos_packet.data.write_u8(0);
                                pos_packet.data.write_var_int(1234);
                                server.send_raw(pos_packet);

                            }

                        }
                        _ => {
                            println!(" => invalid packet 0x{:02X} ({:?})", packet.id, client.state);
                        }
                    }

                }

            }
            Event::Disconnected(addr) => {
                println!("[{}] Disconnected.", addr);
                clients.remove(&addr);
            }
        }

    }

}
