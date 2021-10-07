use super::{ReadablePacket, WritablePacket, PacketResult};
use std::io::Cursor;


#[derive(Debug)]
pub struct RequestStatusPacket;

impl ReadablePacket for RequestStatusPacket {
    fn read_packet(_src: Cursor<&Vec<u8>>) -> PacketResult<Self> {
        Ok(Self)
    }
}


#[derive(Debug)]
pub struct StatusPacket {
    game_version: &'static str,
    protocol_version: u16,
    max_players: u32,
    online_players: u32,
}

impl WritablePacket for StatusPacket {
    fn write_packet(&mut self, dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {
        todo!()
    }
}
