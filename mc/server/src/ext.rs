use std::io::{Result as IoResult, Error as IoError, Read, ErrorKind};
use std::string::FromUtf8Error;

use mc_core::pos::BlockPos;

use byteorder::{BE, ReadBytesExt};
use thiserror::Error;
use std::fmt::{Display, Formatter};


#[derive(Error, Debug)]
pub struct IllegalVarNum;

impl Display for IllegalVarNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("An illegal variable-length number was encountered.")
    }
}



pub trait PacketReadExt {

    fn read_i8(&mut self) -> IoResult<i8>;
    fn read_i16(&mut self) -> IoResult<i16>;
    fn read_i32(&mut self) -> IoResult<i32>;
    fn read_i64(&mut self) -> IoResult<i64>;

    fn read_f32(&mut self) -> IoResult<f32>;
    fn read_f64(&mut self) -> IoResult<f64>;

    fn read_var_int(&mut self) -> IoResult<i32>;
    fn read_var_long(&mut self) -> IoResult<i64>;

    fn read_string(&mut self) -> IoResult<String>;
    fn read_block_pos(&mut self) -> IoResult<BlockPos>;

}

impl<R> PacketReadExt for R
where
    R: Read
{

    fn read_i8(&mut self) -> IoResult<i8> {
        ReadBytesExt::read_i8(self)
    }

    fn read_i16(&mut self) -> IoResult<i16> {
        ReadBytesExt::read_i16::<BE>(self)
    }

    fn read_i32(&mut self) -> IoResult<i32> {
        ReadBytesExt::read_i32::<BE>(self)
    }

    fn read_i64(&mut self) -> IoResult<i64> {
        ReadBytesExt::read_i64::<BE>(self)
    }

    fn read_f32(&mut self) -> IoResult<f32> {
        ReadBytesExt::read_f32::<BE>(self)
    }

    fn read_f64(&mut self) -> IoResult<f64> {
        ReadBytesExt::read_f64::<BE>(self)
    }

    fn read_var_int(&mut self) -> IoResult<i32> {
        let mut value = 0;
        let mut offset = 0;
        loop {
            if offset == 35 {
                return Err(IoError::new(ErrorKind::InvalidData, IllegalVarNum));
            }
            let byte = ReadBytesExt::read_u8(self)?;
            value |= ((byte & 0b01111111) as i32) << offset;
            if byte & 0b10000000 == 0 {
                return Ok(value);
            }
            offset += 7;
        }
    }

    fn read_var_long(&mut self) -> IoResult<i64> {
        let mut value = 0;
        let mut offset = 0;
        loop {
            if offset == 70 {
                return Err(IoError::new(ErrorKind::InvalidData, IllegalVarNum));
            }
            let byte = ReadBytesExt::read_u8(self)?;
            value |= ((byte & 0b01111111) as i64) << offset;
            if byte & 0b10000000 == 0 {
                return Ok(value);
            }
            offset += 7;
        }
    }

    fn read_string(&mut self) -> IoResult<String> {
        let len = self.read_var_int()? as usize;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf[..])?;
        Ok(String::from_utf8(buf).map_err(|err| {
            IoError::new(ErrorKind::InvalidData, err)
        })?)
    }

    fn read_block_pos(&mut self) -> IoResult<BlockPos> {
        let val = PacketReadExt::read_i64(self)?;
        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = ((val << 26) >> 38) as i32;  // Double shift to keep sign.
        Ok(BlockPos::new(x, y, z))
    }

}
