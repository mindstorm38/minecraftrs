use std::io::{Write, Read, Result as IoResult};
use super::{Packet, ClientState};
use crate::packet::serial::*;


pub struct RequestStatusPacket;

impl Packet for RequestStatusPacket {

    const ID: usize = 0;

    fn encode<W: Write>(&mut self, _dst: &mut W) -> IoResult<()> {
        Ok(())
    }

    fn decode<R: Read>(_src: &mut R) -> IoResult<Self> {
        Ok(Self)
    }

}


pub struct StatusPacket {
    game_version: &'static str,
    protocol_version: u16,
    max_players: u32,
    online_players: u32,
}
