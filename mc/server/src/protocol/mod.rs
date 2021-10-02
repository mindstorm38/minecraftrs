use std::io::{Write, Read, Result as IoResult, Cursor};
use std::net::SocketAddr;

use crate::packet::RawPacket;

pub mod handshake;
pub mod status;


/// A trait to implement to custom packet structures.
pub trait Packet: Sized {

    const ID: usize;

    fn encode<W: Write>(&mut self, dst: &mut W) -> IoResult<()>;

    fn decode<R: Read>(src: &mut R) -> IoResult<Self>;

    fn encode_raw(&mut self, addr: SocketAddr) -> IoResult<RawPacket> {
        let mut raw = RawPacket {
            addr,
            id: Self::ID,
            data: Cursor::new(Vec::new())
        };
        self.encode(&mut raw.data);
        Ok(raw)
    }

    fn decode_raw(raw: &mut RawPacket) -> IoResult<Self> {
        Self::decode(&mut raw.data)
    }

}


#[repr(u8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ClientState {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

impl ClientState {

    pub fn get_id(self) -> u8 {
        unsafe { std::mem::transmute(self) }
    }

    pub fn from_id(id: u8) -> Option<Self> {
        if id >= 0 && id < 4 {
            Some(unsafe { std::mem::transmute(id) })
        } else {
            None
        }
    }

}
