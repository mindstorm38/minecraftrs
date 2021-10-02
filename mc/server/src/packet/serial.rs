use std::io::{Read, Write, ErrorKind, Result as IoResult, Error as IoError};
use std::fmt::{Display, Formatter};
use std::error::Error;

use mc_core::pos::BlockPos;

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use nbt::{CompoundTag, decode::TagDecodeError};
use uuid::Uuid;


#[derive(Debug)]
pub struct VarNumError;

impl Error for VarNumError {}

impl Display for VarNumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("An illegal variable-length number was encountered.")
    }
}



pub trait PacketReadExt {

    fn read_bool(&mut self) -> IoResult<bool>;
    fn read_u8(&mut self) -> IoResult<u8>;
    fn read_i8(&mut self) -> IoResult<i8>;
    fn read_u16(&mut self) -> IoResult<u16>;
    fn read_i16(&mut self) -> IoResult<i16>;
    fn read_i32(&mut self) -> IoResult<i32>;
    fn read_i64(&mut self) -> IoResult<i64>;

    fn read_f32(&mut self) -> IoResult<f32>;
    fn read_f64(&mut self) -> IoResult<f64>;

    fn read_var_int(&mut self) -> IoResult<i32>;
    fn read_var_long(&mut self) -> IoResult<i64>;

    fn read_uuid(&mut self) -> IoResult<Uuid>;
    fn read_string(&mut self) -> IoResult<String>;
    fn read_block_pos(&mut self) -> IoResult<BlockPos>;
    fn read_nbt(&mut self) -> IoResult<CompoundTag>;

}

impl<R> PacketReadExt for R
where
    R: Read
{
    fn read_bool(&mut self) -> IoResult<bool> {
        ReadBytesExt::read_u8(self).map(|b| b != 0)
    }

    fn read_u8(&mut self) -> IoResult<u8> {
        ReadBytesExt::read_u8(self)
    }

    fn read_i8(&mut self) -> IoResult<i8> {
        ReadBytesExt::read_i8(self)
    }

    fn read_u16(&mut self) -> IoResult<u16> {
        ReadBytesExt::read_u16::<BE>(self)
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
                return Err(IoError::new(ErrorKind::InvalidData, VarNumError));
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
                return Err(IoError::new(ErrorKind::InvalidData, VarNumError));
            }
            let byte = ReadBytesExt::read_u8(self)?;
            value |= ((byte & 0b01111111) as i64) << offset;
            if byte & 0b10000000 == 0 {
                return Ok(value);
            }
            offset += 7;
        }
    }

    fn read_uuid(&mut self) -> IoResult<Uuid> {
        let mut bytes = [0; 16];
        self.read_exact(&mut bytes)?;
        Ok(Uuid::from_bytes(bytes))
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

    fn read_nbt(&mut self) -> IoResult<CompoundTag> {
        nbt::decode::read_compound_tag(self).map_err(|err| {
            match err {
                TagDecodeError::IOError { io_error } => io_error,
                err => IoError::new(ErrorKind::InvalidData, err)
            }
        })
    }

}


pub trait PacketWriteExt {

    fn write_bool(&mut self, b: bool) -> IoResult<()>;
    fn write_u8(&mut self, val: u8) -> IoResult<()>;
    fn write_i8(&mut self, val: i8) -> IoResult<()>;
    fn write_u16(&mut self, val: u16) -> IoResult<()>;
    fn write_i16(&mut self, val: i16) -> IoResult<()>;
    fn write_i32(&mut self, val: i32) -> IoResult<()>;
    fn write_i64(&mut self, val: i64) -> IoResult<()>;

    fn write_f32(&mut self, val: f32) -> IoResult<()>;
    fn write_f64(&mut self, val: f64) -> IoResult<()>;

    fn write_var_int(&mut self, val: i32) -> IoResult<()>;
    fn write_var_long(&mut self, val: i64) -> IoResult<()>;

    fn write_uuid(&mut self, uuid: &Uuid) -> IoResult<()>;
    fn write_string(&mut self, s: &str) -> IoResult<()>;
    fn write_block_pos(&mut self, pos: &BlockPos) -> IoResult<()>;
    fn write_nbt(&mut self, nbt: &CompoundTag) -> IoResult<()>;

}

impl<W> PacketWriteExt for W
where
    W: Write
{

    fn write_bool(&mut self, b: bool) -> IoResult<()> {
        WriteBytesExt::write_u8(self, if b { 1 } else { 0 })
    }

    fn write_u8(&mut self, val: u8) -> IoResult<()> {
        WriteBytesExt::write_u8(self, val)
    }

    fn write_i8(&mut self, val: i8) -> IoResult<()> {
        WriteBytesExt::write_i8(self, val)
    }

    fn write_u16(&mut self, val: u16) -> IoResult<()> {
        WriteBytesExt::write_u16::<BE>(self, val)
    }

    fn write_i16(&mut self, val: i16) -> IoResult<()> {
        WriteBytesExt::write_i16::<BE>(self, val)
    }

    fn write_i32(&mut self, val: i32) -> IoResult<()> {
        WriteBytesExt::write_i32::<BE>(self, val)
    }

    fn write_i64(&mut self, val: i64) -> IoResult<()> {
        WriteBytesExt::write_i64::<BE>(self, val)
    }

    fn write_f32(&mut self, val: f32) -> IoResult<()> {
        WriteBytesExt::write_f32::<BE>(self, val)
    }

    fn write_f64(&mut self, val: f64) -> IoResult<()> {
        WriteBytesExt::write_f64::<BE>(self, val)
    }

    fn write_var_int(&mut self, val: i32) -> IoResult<()> {
        let mut val = val as u32;
        loop {
            if (val & 0xFFFFFF80) == 0 {
                WriteBytesExt::write_u8(self, val as u8)?;
                return Ok(());
            }
            WriteBytesExt::write_u8(self, (val & 0x7F) as u8 | 0x80)?;
            val >>= 7;
        }
    }

    fn write_var_long(&mut self, val: i64) -> IoResult<()> {
        let mut val = val as u64;
        loop {
            if (val & 0xFFFFFFFFFFFFFF80) == 0 {
                WriteBytesExt::write_u8(self, val as u8)?;
                return Ok(());
            }
            WriteBytesExt::write_u8(self, (val & 0x7F) as u8 | 0x80)?;
            val >>= 7;
        }
    }

    fn write_uuid(&mut self, uuid: &Uuid) -> IoResult<()> {
        self.write_all(uuid.as_bytes())
    }

    fn write_string(&mut self, s: &str) -> IoResult<()> {
        assert!(s.len() <= i32::MAX as usize);
        self.write_var_int(s.len() as i32)?;
        self.write_all(s.as_bytes())
    }

    fn write_block_pos(&mut self, pos: &BlockPos) -> IoResult<()> {
        let mut val = 0u64;
        val |= ((pos.x & 0x3FFFFFF) as u64) << 38;
        val |= ((pos.z & 0x3FFFFFF) as u64) << 12;
        val |= (pos.y & 0xFFF) as u64;
        WriteBytesExt::write_u64::<BE>(self, val)
    }

    fn write_nbt(&mut self, nbt: &CompoundTag) -> IoResult<()> {
        nbt::encode::write_compound_tag(self, nbt)
    }

}
