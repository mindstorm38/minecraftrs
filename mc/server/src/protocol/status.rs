use std::io::{Write, Read, Result as IoResult};
use crate::packet::serial::*;


pub struct RequestStatusPacket;


pub struct StatusPacket {
    game_version: &'static str,
    protocol_version: u16,
    max_players: u32,
    online_players: u32,
}
