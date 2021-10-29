use super::{ReadablePacket, WritablePacket, PacketResult};
use crate::packet::serial::*;

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
    pub game_version: &'static str,
    pub protocol_version: u16,
    pub max_players: u32,
    pub online_players: u32,
    pub description: String
}

impl WritablePacket for StatusPacket {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {

        let payload = serde_json::json!({
            "version": {
                "name": self.game_version,
                "protocol": self.protocol_version
            },
            "players": {
                "max": self.max_players,
                "online": self.online_players,
                "sample": []
            },
            "description": {
                "text": self.description
            }
        });

        dst.write_string(serde_json::to_string(&payload).unwrap().as_str())?;
        Ok(())

    }
}


// Ping / Pong //

#[derive(Debug)]
pub struct PingPacket {
    pub token: i64
}

impl PingPacket {
    pub fn get_pong(&self) -> PongPacket {
        PongPacket {
            token: self.token
        }
    }
}

impl ReadablePacket for PingPacket {
    fn read_packet(mut src: Cursor<&Vec<u8>>) -> PacketResult<Self> {
        Ok(Self {
            token: src.read_i64()?
        })
    }
}


pub struct PongPacket {
    pub token: i64
}

impl WritablePacket for PongPacket {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {
        dst.write_i64(self.token)?;
        Ok(())
    }
}
