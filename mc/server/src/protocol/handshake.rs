use std::io::{Write, Read, Result as IoResult};
use super::{Packet, ClientState};
use crate::packet::serial::*;


pub struct HandshakePacket {
    pub protocol_version: u16,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: ClientState
}

impl Packet for HandshakePacket {

    const ID: usize = 0x00;

    fn encode<W: Write>(&mut self, dst: &mut W) -> IoResult<()> {
        dst.write_var_int(self.protocol_version as i32)?;
        dst.write_string(&self.server_addr)?;
        dst.write_u16(self.server_port);
        dst.write_var_int(self.next_state.get_id() as i32);
        Ok(())
    }

    fn decode<R: Read>(src: &mut R) -> IoResult<Self> {
        Ok(Self {
            protocol_version: src.read_var_int()? as u16,
            server_addr: src.read_string()?,
            server_port: src.read_u16()?,
            next_state: ClientState::from_id(src.read_var_int()? as u8).unwrap_or(ClientState::Status)
        })
    }

}
