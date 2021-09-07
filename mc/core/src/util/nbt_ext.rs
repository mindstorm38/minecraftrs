//! This module defines some NBT utilities which extends the real NBT crate,
//! named 'named-binary-tag'.

use nbt::{CompoundTag, CompoundTagError};
use uuid::Uuid;

use crate::pos::{EntityPos, BlockPos};


pub trait NbtExt {

    fn get_i8_or<'a>(&'a self, name: &'a str, default: i8) -> i8;
    fn get_i16_or<'a>(&'a self, name: &'a str, default: i16) -> i16;
    fn get_i32_or<'a>(&'a self, name: &'a str, default: i32) -> i32;
    fn get_i64_or<'a>(&'a self, name: &'a str, default: i64) -> i64;
    fn get_bool_or<'a>(&'a self, name: &'a str, default: bool) -> bool;

    fn insert_uuid(&mut self, name: impl ToString, value: &Uuid);
    fn get_uuid<'a>(&'a self, name: &'a str) -> Result<Uuid, CompoundTagError<'a>>;

    fn insert_entity_pos(&mut self, name: impl ToString, value: &EntityPos);
    fn get_entity_pos<'a>(&'a self, name: &'a str) -> Result<EntityPos, CompoundTagError<'a>>;

    fn insert_block_pos(&mut self, name: impl ToString, value: &BlockPos);
    fn get_block_pos<'a>(&'a self, name: &'a str) -> Result<BlockPos, CompoundTagError<'a>>;

    fn insert_split_block_pos(&mut self, x_name: impl ToString, y_name: impl ToString, z_name: impl ToString, value: &BlockPos);
    fn get_split_block_pos<'a>(&'a self, x_name: &'a str, y_name: &'a str, z_name: &'a str) -> Result<BlockPos, CompoundTagError<'a>>;

    fn get_string_vec<'a>(&'a self, name: &'a str) -> Result<Vec<String>, CompoundTagError<'a>>;

}

impl NbtExt for CompoundTag {

    #[inline]
    fn get_i8_or<'a>(&'a self, name: &'a str, default: i8) -> i8 {
        self.get_i8(name).unwrap_or(default)
    }

    #[inline]
    fn get_i16_or<'a>(&'a self, name: &'a str, default: i16) -> i16 {
        self.get_i16(name).unwrap_or(default)
    }

    #[inline]
    fn get_i32_or<'a>(&'a self, name: &'a str, default: i32) -> i32 {
        self.get_i32(name).unwrap_or(default)
    }

    #[inline]
    fn get_i64_or<'a>(&'a self, name: &'a str, default: i64) -> i64 {
        self.get_i64(name).unwrap_or(default)
    }

    #[inline]
    fn get_bool_or<'a>(&'a self, name: &'a str, default: bool) -> bool {
        self.get_bool(name).unwrap_or(default)
    }

    fn insert_uuid(&mut self, name: impl ToString, value: &Uuid) {
        let mut uuid_values = Vec::with_capacity(4);
        let uuid_raw = value.as_u128();
        uuid_values[0] = ((uuid_raw >> 96) & 0xFFFFFFFF) as i32;
        uuid_values[1] = ((uuid_raw >> 64) & 0xFFFFFFFF) as i32;
        uuid_values[2] = ((uuid_raw >> 32) & 0xFFFFFFFF) as i32;
        uuid_values[3] = (uuid_raw & 0xFFFFFFFF) as i32;
        self.insert_i32_vec(name, uuid_values);
    }

    fn get_uuid<'a>(&'a self, name: &'a str) -> Result<Uuid, CompoundTagError<'a>> {
        let vec = self.get_i32_vec(name)?;
        if vec.len() == 4 {
            let mut uuid_raw = 0;
            uuid_raw |= (vec[0] as u32 as u128) << 96;
            uuid_raw |= (vec[1] as u32 as u128) << 64;
            uuid_raw |= (vec[2] as u32 as u128) << 32;
            uuid_raw |= vec[3] as u32 as u128;
            Ok(Uuid::from_u128(uuid_raw))
        } else {
            Err(CompoundTagError::TagWrongType {
                name,
                actual_tag: self.get(name).unwrap()
            })
        }
    }

    fn insert_entity_pos(&mut self, name: impl ToString, value: &EntityPos) {
        self.insert_f64_vec(name, value.into_array());
    }

    fn get_entity_pos<'a>(&'a self, name: &'a str) -> Result<EntityPos, CompoundTagError<'a>> {
        let raw_pos = self.get_f64_vec(name)?;
        if raw_pos.len() == 3 {
            Ok(EntityPos::new(raw_pos[0], raw_pos[1], raw_pos[2]))
        } else {
            Err(CompoundTagError::TagWrongType {
                name,
                actual_tag: self.get(name).unwrap()
            })
        }
    }

    fn insert_block_pos(&mut self, name: impl ToString, value: &BlockPos) {
        let mut comp = CompoundTag::new();
        comp.insert_split_block_pos("X", "Y", "Z", value);
        self.insert_compound_tag(name, comp);
    }

    fn get_block_pos<'a>(&'a self, name: &'a str) -> Result<BlockPos, CompoundTagError<'a>> {
        let comp = self.get_compound_tag(name)?;
        comp.get_split_block_pos("X", "Y", "Z")
    }

    fn insert_split_block_pos(&mut self, x_name: impl ToString, y_name: impl ToString, z_name: impl ToString, value: &BlockPos) {
        self.insert_i32(x_name, value.x);
        self.insert_i32(y_name, value.y);
        self.insert_i32(z_name, value.z);
    }

    fn get_split_block_pos<'a>(&'a self, x_name: &'a str, y_name: &'a str, z_name: &'a str) -> Result<BlockPos, CompoundTagError<'a>> {
        let x = self.get_i32(x_name)?;
        let y = self.get_i32(y_name)?;
        let z = self.get_i32(z_name)?;
        Ok(BlockPos::new(x, y, z))
    }

    fn get_string_vec<'a>(&'a self, name: &'a str) -> Result<Vec<String>, CompoundTagError<'a>> {
        self.get_str_vec(name).map(|raw| {
            raw.into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
    }

}
