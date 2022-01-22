use std::io::Read;
use std::sync::Arc;

use nbt::decode::{read_compound_tag, TagDecodeError};
use nbt::{CompoundTag, Tag, CompoundTagError};
use hecs::EntityBuilder;
use thiserror::Error;

use crate::world::level::{LevelEnv, BaseEntity};
use crate::world::chunk::{ChunkStatus, Light};
use crate::world::source::ProtoChunk;
use crate::entity::GlobalEntities;
use crate::block::BlockState;
use crate::biome::Biome;
use crate::util::{NbtExt, PackedIterator};


/// The only supported data version for decoding. Current is `1.18.1`.
pub const DATA_VERSION: i32 = 2865;


#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Data version {0} is not supported.")]
    UnsupportedDataVersion(i32),
    #[error("Unknown block state '{0}' in the palette for the chunk environments.")]
    UnknownBlockState(String),
    #[error("Unknown property value: {0}")]
    UnknownBlockProperty(String),
    #[error("Unknown biome ID: {0}")]
    UnknownBiome(String),
    #[error("Unknown entity type: {0}")]
    UnknownEntityType(String),
    #[error("Malformed entity: {0}")]
    MalformedEntity(String),
    #[error("The NBT raw data cannot be decoded: {0}")]
    Nbt(#[from] TagDecodeError),
    #[error("The chunk's NBT structure is malformed and some fields are missing or are of the wrong type: {0}")]
    Malformed(String)
}

impl<'a> From<CompoundTagError<'a>> for DecodeError {
    fn from(err: CompoundTagError<'a>) -> Self {
        Self::Malformed(format!("{}", err))
    }
}


/// Decode the NBT data from a reader and delegate chunk decoding to `decode_chunk`.
pub fn decode_chunk_from_reader(reader: &mut impl Read, chunk: &mut ProtoChunk) -> Result<(), DecodeError> {
    decode_chunk(&read_compound_tag(reader)?, chunk)
}

/// Decode a chunk from its NBT data.
pub fn decode_chunk(tag_root: &CompoundTag, chunk: &mut ProtoChunk) -> Result<(), DecodeError> {

    let data_version = tag_root.get_i32("DataVersion")?;
    if data_version != DATA_VERSION {
        return Err(DecodeError::UnsupportedDataVersion(data_version));
    }

    // Removed in 1.18 data version
    // let tag_level = tag_root.get_compound_tag("Level")?;

    let cx = tag_root.get_i32("xPos")?;
    let _lower_cy = tag_root.get_i32("yPos")?;
    let cz = tag_root.get_i32("zPos")?;
    check_position(chunk, cx, cz)?;

    chunk.set_status(match tag_root.get_str("Status")? {
        "empty" => ChunkStatus::Empty,
        "structure_starts" => ChunkStatus::StructureStarts,
        "structure_references" => ChunkStatus::StructureReferences,
        "biomes" => ChunkStatus::Biomes,
        "noise" => ChunkStatus::Noise,
        "surface" => ChunkStatus::Surface,
        "carvers" => ChunkStatus::Carvers,
        "liquid_carvers" => ChunkStatus::LiquidCarvers,
        "features" => ChunkStatus::Features,
        "light" => ChunkStatus::Light,
        "spawn" => ChunkStatus::Spawn,
        "heightmaps " => ChunkStatus::Heightmaps,
        "full" => ChunkStatus::Full,
        unknown_status => {
            return Err(DecodeError::Malformed(format!("Unknown status: {}.", unknown_status)));
        }
    });

    // Common environment
    let env = Arc::clone(chunk.get_env());
    let height = chunk.get_height();

    /*if height.min != lower_cy {
        return Err(DecodeError::Malformed(
            format!("The environment's height minimum Y ({}) is not valid for decoding chunk with minimum Y if {}.", height.min, lower_cy)
        ));
    }*/

    // Sections
    let tag_sections = tag_root.get_compound_tag_vec("sections")?;
    for tag_section in tag_sections {

        // Sub chunk height
        let cy = tag_section.get_i8("Y")?;

        if cy < height.min || cy > height.max {
            return Err(DecodeError::Malformed(format!("Invalid section at Y {}, supported height is {:?}", cy, height)));
        }

        if let Ok(tag_biomes) = tag_section.get_compound_tag("biomes") {

            let tag_palette = tag_biomes.get_str_vec("palette")?;

            let mut biomes_palette = Vec::new();
            for tag_biome in tag_palette {
                biomes_palette.push(decode_biome(tag_biome, &env)?);
            }

            let biomes_offset = (cy - height.min) as usize * 64;

            if let Ok(tag_data) = tag_biomes.get_i64_vec("data") {

                let bits = tag_data.len() as u8; // Simplified the (len * 64 / 64)
                let unpacked_biomes = tag_data.iter()
                    .map(|&v| v as u64)
                    .unpack_aligned(bits)
                    .take(64)
                    .map(|v| v as usize);

                unsafe {
                    chunk.set_biomes_raw(biomes_offset, biomes_palette, unpacked_biomes);
                }

            } else {

                // If only one biome is present, and no data, we must apply this biome to the whole
                // sub chunk.
                if biomes_palette.len() == 1 {
                    unsafe {
                        chunk.set_biomes_raw(biomes_offset, biomes_palette, std::iter::repeat(0).take(64))
                    }
                }

            }

        }

        if let Ok(tag_block_states) = tag_section.get_compound_tag("block_states") {

            let tag_palette = tag_block_states.get_compound_tag_vec("palette")?;

            let mut blocks_palette = Vec::new();
            for tag_block in tag_palette {
                blocks_palette.push(decode_block_state(tag_block, &env)?);
            }

            if let Ok(tag_data) = tag_block_states.get_i64_vec("data") {

                let bits = (tag_data.len() * 64 / 4096) as u8;
                let unpacked_blocks = tag_data.iter()
                    .map(|&v| v as u64)
                    .unpack_aligned(bits)
                    .take(4096)
                    .map(|v| v as usize);

                if let Ok(sub_chunk) = chunk.ensure_sub_chunk(cy) {
                    unsafe {
                        sub_chunk.set_blocks_raw(blocks_palette, unpacked_blocks);
                    }
                }

            } else {

                // Because it is unclear what is expected when data tag is absent, I suppose that we
                // should create and fill the sub chunk if the palette contains only one block state.
                // We only fill the sub chunk if the only block is not the null block.
                if blocks_palette.len() == 1 && env.blocks.get_sid_from(&blocks_palette[0]).unwrap() != 0 {
                    if let Ok(sub_chunk) = chunk.ensure_sub_chunk(cy) {
                        sub_chunk.fill_block(blocks_palette[0]).unwrap();
                    }
                }

            }

        }

        #[inline]
        fn iter_light_slice(slice: &[i8]) -> impl Iterator<Item = u8> + '_ {
            slice.iter().flat_map(|&v| [(v as u8) & 0xF, ((v as u8) & 0xF0) >> 4])
        }

        if let Ok(tag_block_light) = tag_section.get_i8_vec("BlockLight") {
            if let Ok(sub_chunk) = chunk.ensure_sub_chunk(cy) {
                unsafe {
                    sub_chunk.set_lights_raw(Light::Block, iter_light_slice(&tag_block_light[..]));
                }
            }
        }

        if let Ok(tag_sky_light) = tag_section.get_i8_vec("SkyLight") {
            if let Ok(sub_chunk) = chunk.ensure_sub_chunk(cy) {
                unsafe {
                    sub_chunk.set_lights_raw(Light::Block, iter_light_slice(&tag_sky_light[..]));
                }
            }
        }

    }

    // TODO: Heightmaps
    // TODO: Block entities

    Ok(())

}

/// Decode block state from a state compound tag.
pub fn decode_block_state(tag_block: &CompoundTag, env: &LevelEnv) -> Result<&'static BlockState, DecodeError> {

    let block_name = tag_block.get_str("Name")?;

    let mut block_state = env.blocks
        .get_block_from_name(block_name)
        .map(|block| block.get_default_state())
        .ok_or_else(|| DecodeError::UnknownBlockState(block_name.to_string()))?;

    if let Ok(block_properties) = tag_block.get_compound_tag("Properties") {
        for (prop_name, prop_value_tag) in block_properties.iter() {
            if let Tag::String(prop_value) = prop_value_tag {
                block_state = block_state
                    .with_raw(prop_name, &prop_value)
                    .ok_or_else(|| DecodeError::UnknownBlockProperty(format!("{}/{}={}", block_name, prop_name, prop_value)))?;
            }
        }
    }

    Ok(block_state)

}

/// Decode biome from it's name and the environment.
pub fn decode_biome(name: &str, env: &LevelEnv) -> Result<&'static Biome, DecodeError> {
    env.biomes.get_biome_from_name(name).ok_or_else(|| DecodeError::UnknownBiome(name.to_string()))
}

/// Decode chunk entities stored in there own files.
pub fn decode_entities(tag_root: &CompoundTag, chunk: &mut ProtoChunk) -> Result<(), DecodeError> {

    // TODO: Use data version to apply data fixers
    let _data_version = tag_root.get_i32("DataVersion")?;
    let tag_position = tag_root.get_i32_vec("Position")?;

    if tag_position.len() != 2 {
        return Err(DecodeError::Malformed("Invalid 'Position', expected exactly two integers.".to_string()))
    }

    let cx = tag_position[0];
    let cz = tag_position[1];
    check_position(chunk, cx, cz)?;

    let tag_entities = tag_root.get_compound_tag_vec("Entities")?;

    // Common environment
    let env = Arc::clone(chunk.get_env());

    // Decode entities
    for tag_entity in tag_entities {
        // TODO: Don't return err here, maybe make a softer abort?
        decode_entity(tag_entity, &env.entities, chunk)?;
    }

    Ok(())

}

/// Internal function to decode an entity, optionally recursively if it have passengers.
/// The function returns the index of the entity builder in the proto chunk, this is used
/// to set entity passengers indices.
pub fn decode_entity(tag_entity: &CompoundTag, entities: &GlobalEntities, chunk: &mut ProtoChunk) -> Result<usize, DecodeError> {

    let entity_id = tag_entity.get_str("id")?;

    let (entity_type, entity_codecs) = entities
        .get_entity_type_and_codecs(entity_id)
        .ok_or_else(|| {
            DecodeError::UnknownEntityType(entity_id.to_string())
        })?;

    let uuid = tag_entity.get_uuid("UUID")?;
    let pos = tag_entity.get_entity_pos("Pos")?;

    let mut entity_builder = EntityBuilder::new();

    for &entity_codec in entity_codecs {
        // TODO: Don't return err here, maybe make a softer abort?
        entity_codec.decode(tag_entity, &mut entity_builder).map_err(|msg| {
            DecodeError::MalformedEntity(msg)
        })?;
    }

    entity_builder.add(BaseEntity::new(entity_type, uuid, pos));

    let proto_index = chunk.add_proto_entity(entity_builder);

    if let Ok(tag_passengers) = tag_entity.get_compound_tag_vec("Passengers") {
        for tag_passenger in tag_passengers {
            let passenger_proto_index = decode_entity(tag_passenger, entities, chunk)?;
            chunk.add_proto_entity_passengers(proto_index, passenger_proto_index);
        }
    }

    Ok(proto_index)

}

fn check_position(chunk: &ProtoChunk, cx: i32, cz: i32) -> Result<(), DecodeError> {
    let (expected_cx, expected_cz) = chunk.get_position();
    if expected_cx != cx || expected_cz != cz {
        Err(DecodeError::Malformed(
            format!("Incoherent position given by the chunk NBT, expected {}/{}, got {}/{}.",
                    expected_cx, expected_cz, cx, cz)
        ))
    } else {
        Ok(())
    }
}
