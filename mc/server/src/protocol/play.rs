use super::{ReadablePacket, WritablePacket, PacketResult, PacketError};
use crate::packet::serial::*;

use mc_vanilla::biome::VANILLA_BIOMES;
use mc_vanilla::util::GameMode;
use mc_runtime::world::World;
use mc_core::pos::BlockPos;

use nbt::CompoundTag;

use std::io::{Cursor, Write, Read};


/// Client bound
pub struct JoinGamePacket<'a> {
    pub eid: i32,
    pub hardcore: bool,
    pub game_mode: GameMode,
    pub last_game_mode: Option<GameMode>,
    pub world: &'a World,
    pub level_index: usize,
    pub hashed_seed: u64,
    pub view_distance: u8,
}

impl<'a> WritablePacket for JoinGamePacket<'a> {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {

        if self.level_index >= self.world.levels.len() {
            return Err(PacketError::InvalidField("given level index is out of bounds"));
        }

        dst.write_i32(self.eid).unwrap();
        dst.write_bool(self.hardcore).unwrap();
        dst.write_u8(self.game_mode.get_id()).unwrap();
        dst.write_i8(self.last_game_mode.map(|gm| gm.get_id() as i8).unwrap_or(-1)).unwrap();
        dst.write_var_int(self.world.levels.len() as i32).unwrap();

        for level in &self.world.levels {
            dst.write_string((**level).borrow().get_id().as_str()).unwrap();
        }

        let mut dimension_codec = CompoundTag::new();
        dimension_codec.insert_compound_tag("minecraft:dimension_type", {
            let mut dimension_type = CompoundTag::new();
            dimension_type.insert_str("type", "minecraft:dimension_type");
            dimension_type.insert_compound_tag_vec("value", self.world.levels.iter()
                .enumerate()
                .map(|(i, level)| {
                    let mut dimension_value = CompoundTag::new();
                    dimension_value.insert_str("name", (**level).borrow().get_id());
                    dimension_value.insert_i32("id", i as i32);
                    dimension_value.insert_compound_tag("element", {
                        let mut dimension_element = CompoundTag::new();
                        dimension_element.insert_bool("piglin_safe", false);
                        dimension_element.insert_bool("natural", true);
                        dimension_element.insert_f32("ambient_light", 0.0);
                        dimension_element.insert_str("infiniburn", "minecraft:infiniburn_overworld");
                        dimension_element.insert_bool("respawn_anchor_works", false);
                        dimension_element.insert_bool("has_skylight", true);
                        dimension_element.insert_bool("bed_works", true);
                        dimension_element.insert_str("effects", "minecraft:overworld");
                        dimension_element.insert_bool("has_raids", true);
                        dimension_element.insert_i32("logical_height", 256);
                        dimension_element.insert_f32("coordinate_scale", 1.0);
                        dimension_element.insert_bool("ultrawarm", false);
                        dimension_element.insert_bool("has_ceiling", false);
                        dimension_element
                    });
                    dimension_value
                }));
            dimension_type
        });
        dimension_codec.insert_compound_tag("minecraft:worldgen/biome", {

            let mut biome_reg = CompoundTag::new();
            biome_reg.insert_str("type", "minecraft:worldgen/biome");
            biome_reg.insert_compound_tag_vec("value", VANILLA_BIOMES.iter()
                .map(|&biome| {

                    let mut biome_value = CompoundTag::new();
                    biome_value.insert_str("name", biome.get_name());
                    biome_value.insert_i32("id", biome.get_id());
                    biome_value.insert_compound_tag("element", {
                        let mut biome_element = CompoundTag::new();
                        biome_element.insert_str("precipitation", "none");
                        biome_element.insert_compound_tag("effects", {
                            let mut biome_effects = CompoundTag::new();
                            biome_effects.insert_i32("sky_color", 8103167);
                            biome_effects.insert_i32("water_fog_color", 329011);
                            biome_effects.insert_i32("fog_color", 12638463);
                            biome_effects.insert_i32("water_color", 4159204);
                            biome_effects.insert_compound_tag("mood_sound", {
                                let mut biome_mood_sound = CompoundTag::new();
                                biome_mood_sound.insert_i32("tick_delay", 6000);
                                biome_mood_sound.insert_f64("offset", 2.0);
                                biome_mood_sound.insert_str("sound", "minecraft:ambient.cave");
                                biome_mood_sound.insert_i32("block_search_extent", 8);
                                biome_mood_sound
                            });
                            biome_effects
                        });
                        biome_element.insert_f32("depth", 0.1);
                        biome_element.insert_f32("temperature", 0.5);
                        biome_element.insert_f32("scale", 0.2);
                        biome_element.insert_f32("downfall", 0.5);
                        biome_element.insert_str("category", "none");
                        biome_element
                    });

                    biome_value

                }));

            biome_reg

        });

        dst.write_nbt(&dimension_codec).unwrap();
        dst.write_nbt({
            dimension_codec.get_compound_tag("minecraft:dimension_type").unwrap()
                .get_compound_tag_vec("value").unwrap()[self.level_index]
                .get_compound_tag("element").unwrap()
        }).unwrap();

        dst.write_string((*self.world.levels[self.level_index]).borrow().get_id().as_str()).unwrap();

        dst.write_i64(self.hashed_seed as i64).unwrap();
        dst.write_var_int(0).unwrap();
        dst.write_var_int(self.view_distance as i32).unwrap();
        dst.write_bool(false).unwrap();
        dst.write_bool(true).unwrap();
        dst.write_bool(false).unwrap();
        dst.write_bool(false).unwrap();

        Ok(())

    }
}


/// Client & server bound
pub enum PluginMessage {
    Brand(String),
    Custom {
        id: String,
        data: Vec<u8>
    }
}

impl WritablePacket for PluginMessage {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {

        dst.write_string(match self {
            Self::Brand(_) => "minecraft:brand",
            Self::Custom { id, .. } => id.as_str()
        }).unwrap();

        match self {
            Self::Brand(brand) => dst.write_string(brand.as_str()).unwrap(),
            Self::Custom { data, .. } => dst.write_all(&data[..]).unwrap()
        }

        Ok(())

    }
}

impl ReadablePacket for PluginMessage {
    fn read_packet(mut src: Cursor<&Vec<u8>>) -> PacketResult<Self> {
        let channel = src.read_string()?;
        Ok(match channel.as_str() {
            "minecraft:brand" => Self::Brand(src.read_string()?),
            channel => Self::Custom {
                id: channel.to_string(),
                data: {
                    let mut data = Vec::new();
                    src.read_to_end(&mut data)?;
                    data
                }
            }
        })
    }
}


/// Client bound
pub struct SpawnPositionPacket {
    pub pos: BlockPos
}

impl WritablePacket for SpawnPositionPacket {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {
        dst.write_block_pos(&self.pos).unwrap();
        Ok(())
    }
}


/// Client bound
pub struct PlayerAbilitiesPacket {
    pub invulnerable: bool,
    pub flying: bool,
    pub allow_flying: bool,
    pub instant_break: bool,
    pub flying_speed: f32,
    pub fov_modifier: f32
}

impl WritablePacket for PlayerAbilitiesPacket {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {
        dst.write_u8(crate::build_flags!(
            self.invulnerable,
            self.flying,
            self.allow_flying,
            self.instant_break
        )).unwrap();
        dst.write_f32(self.flying_speed).unwrap();
        dst.write_f32(self.fov_modifier).unwrap();
        Ok(())
    }
}


/// Client bound
pub struct PlayerPosAndLook {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub rel_x: bool,
    pub rel_y: bool,
    pub rel_z: bool,
    pub rel_yaw: bool,
    pub rel_pitch: bool,
    pub tp_id: u32
}

impl WritablePacket for PlayerPosAndLook {
    fn write_packet(&mut self, mut dst: Cursor<&mut Vec<u8>>) -> PacketResult<()> {
        dst.write_f64(self.x).unwrap();
        dst.write_f64(self.y).unwrap();
        dst.write_f64(self.z).unwrap();
        dst.write_f32(self.yaw).unwrap();
        dst.write_f32(self.pitch).unwrap();
        dst.write_u8(crate::build_flags!(
            self.rel_x,
            self.rel_y,
            self.rel_z,
            self.rel_yaw,
            self.rel_pitch
        )).unwrap();
        dst.write_var_int(self.tp_id as i32).unwrap();
        Ok(())
    }
}


pub struct ChunkDataPacket {
    cx: i32,
    cz: i32,
}