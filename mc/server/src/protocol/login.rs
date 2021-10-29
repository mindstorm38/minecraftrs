use super::{ReadablePacket, WritablePacket, PacketResult, PacketError};
use crate::packet::serial::*;

use std::io::Cursor;
use uuid::Uuid;


#[derive(Debug)]
pub struct LoginStartPacket {
    pub username: String
}

impl ReadablePacket for LoginStartPacket {
    fn read_packet(mut src: Cursor<&Vec<u8>>) -> PacketResult<Self> {
        Ok(Self {
            username: {
                let s = src.read_string()?;
                if s.len() > 16 {
                    return Err(PacketError::InvalidField("username field should not exceed 16 characters"))
                } else {
                    s
                }
            }
        })
    }
}


#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub username: String,
    pub uuid: Uuid
}

impl WritablePacket for LoginSuccessPacket {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {
        dst.write_uuid(&self.uuid)?;
        dst.write_string(self.username.as_str())?;
        Ok(())
    }
}
