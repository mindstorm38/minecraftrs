//! Protocol implementation in Rust for Minecraft 1.16.5
//!
//! Source: https://wiki.vg/index.php?title=Protocol&oldid=16681

use std::io::Cursor;

use thiserror::Error;

pub mod handshake;
pub mod status;
pub mod login;
pub mod play;


#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
    #[error("Invalid field: {0}")]
    InvalidField(&'static str),
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


/// Client state is a state that is kept cached for each client connection and is
/// used to change the meaning of received packet IDs.
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
        self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        if id < 4 {
            Some(unsafe { std::mem::transmute(id) })
        } else {
            None
        }
    }

}
