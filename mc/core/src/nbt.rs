//! This module defines some NBT utilities which extends the real NBT crate,
//! named 'named-binary-tag'.

use nbt::{CompoundTag, CompoundTagError, Tag};
use uuid::Uuid;

use crate::pos::EntityPos;


pub trait NbtExt {

    fn insert_uuid(&mut self, name: impl ToString, value: Uuid);
    fn get_uuid<'a>(&'a self, name: &'a str) -> Result<Uuid, CompoundTagError<'a>>;

    fn insert_entity_pos(&mut self, name: impl ToString, value: EntityPos);
    fn get_entity_pos<'a>(&'a self, name: &'a str) -> Result<EntityPos, CompoundTagError<'a>>;

}

impl NbtExt for CompoundTag {

    fn insert_uuid(&mut self, name: impl ToString, value: Uuid) {
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
            uuid_raw |= (vec[3] as u32 as u128);
            Ok(Uuid::from_u128(uuid_raw))
        } else {
            Err(CompoundTagError::TagWrongType {
                name,
                actual_tag: self.get(name).unwrap()
            })
        }
    }

    fn insert_entity_pos(&mut self, name: impl ToString, value: EntityPos) {
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

}
