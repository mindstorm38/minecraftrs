use std::io::{Write, Read, Result as IoResult, Cursor};
use std::net::SocketAddr;

use crate::packet::RawPacket;

pub mod handshake;
pub mod status;


pub type RawWriter = Cursor<Vec<u8>>;
pub type RawReader = Cursor<Vec<u8>>;


pub trait WritablePacket {
    fn write_packet(&mut self, dst: &mut RawWriter) -> IoResult<()>;
}

pub trait ReadablePacket: Sized {
    fn read_packet(src: &mut RawReader) -> IoResult<Self>;
}


#[repr(u8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
        if id < 4 {
            Some(unsafe { std::mem::transmute(id) })
        } else {
            None
        }
    }

}
