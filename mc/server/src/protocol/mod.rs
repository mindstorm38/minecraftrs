use std::net::SocketAddr;
use std::io::Cursor;

use crate::packet::RawPacket;
use thiserror::Error;

pub mod handshake;
pub mod status;


#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
    #[error("{0}")]
    IoError(#[from] std::io::Error)
}

pub type PacketResult<T> = Result<T, PacketError>;


/// To implement for packets that could be written.
pub trait WritablePacket {
    fn write_packet(&mut self, dst: Cursor<&mut Vec<u8>>) -> PacketResult<()>;
}

/// To implement for packets that could be read.
pub trait ReadablePacket: Sized {
    fn read_packet(src: Cursor<&Vec<u8>>) -> PacketResult<Self>;
}

// Implementations for empty packets //

impl WritablePacket for () {
    fn write_packet(&mut self, _dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {
        Ok(())
    }
}

impl ReadablePacket for () {
    fn read_packet(_src: Cursor<&Vec<u8>>) -> PacketResult<Self> {
        Ok(())
    }
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
