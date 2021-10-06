use std::io::{Result as IoResult};

use crate::protocol::{ReadablePacket, RawReader, WritablePacket, RawWriter};
use crate::packet::serial::*;


#[derive(Debug)]
pub struct RequestStatusPacket;

impl ReadablePacket for RequestStatusPacket {
    fn read_packet(_src: &mut RawReader) -> IoResult<Self> {
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
    fn write_packet(&mut self, _dst: &mut RawWriter) -> IoResult<()> {
        todo!()
    }
}
