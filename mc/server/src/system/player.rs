use mc_runtime::world::World;

use super::protocol::{ProtocolServer, ProtocolPlayerEntity};
use crate::protocol::play::{ChunkDataPacket, UpdateViewPositionPacket};

const LOAD_DISTANCE: i32 = 8;


pub fn system_player_view(world: &mut World) {

    let proto_server = world.components.get::<ProtocolServer>().unwrap();

    for level in &world.levels {
        let mut level = &mut *level.borrow_mut();
        for (_, comp) in level.entities.ecs.query_mut::<(&mut ProtocolPlayerEntity)>() {

            let (cx, cz): (i32, i32) = comp.chunk_pos;

            if comp.last_chunk_pos.is_none()  {

                proto_server.send_packet(comp.addr, 0x40, &mut UpdateViewPositionPacket {
                    cx,
                    cz
                });

                for rcx in (cx - LOAD_DISTANCE)..(cx + LOAD_DISTANCE) {
                    for rcz in (cz - LOAD_DISTANCE)..(cz + LOAD_DISTANCE) {
                        if let Some(chunk) = level.chunks.get_chunk(rcx, rcz) {
                            proto_server.send_packet(comp.addr, 0x20, &mut ChunkDataPacket::new(&*chunk));
                            println!("Sending chunk {}/{} to {}.", rcx, rcz, comp.addr);
                        }
                    }
                }

            }

            comp.last_chunk_pos = Some(comp.chunk_pos);

        }
    }

}
