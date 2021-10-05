use std::io::{Write, Read, Result as IoResult};
use super::{ReadablePacket, ClientState};
use crate::protocol::RawReader;
use crate::packet::serial::*;


#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: u16,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: ClientState
}

impl ReadablePacket for HandshakePacket {
    fn read_packet(src: &mut RawReader) -> IoResult<Self> {
        Ok(Self {
            protocol_version: src.read_var_int()? as u16,
            server_addr: src.read_string()?,
            server_port: src.read_u16()?,
            next_state: ClientState::from_id(src.read_var_int()? as u8)
                .unwrap_or(ClientState::Status)
        })
    }
}
