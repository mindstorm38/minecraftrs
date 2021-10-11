use std::sync::Arc;
use std::io::Read;

use nbt::decode::{read_compound_tag, TagDecodeError};
use nbt::{CompoundTag, Tag, CompoundTagError};
use hecs::EntityBuilder;
use thiserror::Error;

use crate::world::chunk::{ChunkStatus, SubChunk};
use crate::world::level::{LevelEnv, BaseEntity};
use crate::world::source::ProtoChunk;
use crate::entity::GlobalEntities;
use crate::block::BlockState;
use crate::util::{Rect, NbtExt, PackedIterator};


#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unknown block state '{0}' in the palette for the chunk environments.")]
    UnknownBlockState(String),
    #[error("Unknown property value: {0}")]
    UnknownBlockProperty(String),
    #[error("Unknown biome ID: {0}")]
    UnknownBiome(i32),
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

    // TODO: Use data version to apply data fixers
    let _data_version = tag_root.get_i32("DataVersion")?;
    let tag_level = tag_root.get_compound_tag("Level")?;

    let cx = tag_level.get_i32("xPos")?;
    let cz = tag_level.get_i32("zPos")?;
    check_position(chunk, cx, cz)?;

    chunk.set_status(match tag_level.get_str("Status")? {
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
    let env = chunk.clone_env();
    let height = chunk.get_height();

    if let Ok(raw_biomes) = tag_level.get_i32_vec("Biomes") {

        if raw_biomes.len() == 256 {

            let mut biomes = Vec::with_capacity(256);

            for raw_biome_id in raw_biomes {
                match env.biomes.get_biome_from_id(*raw_biome_id) {
                    Some(biome) => biomes.push(biome),
                    None => return Err(DecodeError::UnknownBiome(*raw_biome_id))
                }
            }

            chunk.set_biomes_2d(&Rect::from_raw(biomes, 16, 16)).unwrap();

        } else if raw_biomes.len() == height.len() * 64 {

            let mut vec = Vec::with_capacity(raw_biomes.len());

            for id in raw_biomes {
                match env.biomes.get_biome_from_id(*id) {
                    Some(biome) => vec.push(biome),
                    None => return Err(DecodeError::UnknownBiome(*id))
                }
            }

            chunk.set_biomes_3d(&vec[..]).unwrap();

        } else {
            return Err(DecodeError::Malformed(format!("Malformed biomes array of length {}.", raw_biomes.len())));
        }

    }

    // Sections
    for tag_section in tag_level.get_compound_tag_vec("Sections")? {

        let cy = tag_section.get_i8("Y")?;

        if let Some(_chunk_offset) = chunk.get_sub_chunk_offset(cy) {

            let mut sub_chunk = SubChunk::new_default(Arc::clone(&env));

            if let Ok(tag_packed_blocks) = tag_section.get_i64_vec("BlockStates") {
                if let Ok(tag_blocks_palette) = tag_section.get_compound_tag_vec("Palette") {

                    let mut blocks_palette = Vec::new();
                    for tag_block in tag_blocks_palette {
                        blocks_palette.push(decode_block_state(tag_block, &env)?);
                    }

                    let bits = (tag_packed_blocks.len() * 64 / 4096).max(4) as u8;

                    let unpacked_blocks = tag_packed_blocks.iter()
                        .map(|&v| v as u64)
                        .unpack_aligned(bits)
                        .take(4096);

                    unsafe {
                        sub_chunk.set_blocks_raw(blocks_palette, unpacked_blocks);
                    }

                }
            }

            // SAFETY: We can unwrap because we have already checked the validity of 'cy'.
            chunk.replace_sub_chunk(cy, sub_chunk).unwrap();

        }

    }

    Ok(())

}

/// Decode block state from a state compound tag.
fn decode_block_state(tag_block: &CompoundTag, env: &LevelEnv) -> Result<&'static BlockState, DecodeError> {

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

/// Decode chunk entities stored in there own files.
fn decode_entities(tag_root: &CompoundTag, chunk: &mut ProtoChunk) -> Result<(), DecodeError> {

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
    let env = chunk.clone_env();

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
fn decode_entity(tag_entity: &CompoundTag, entities: &GlobalEntities, chunk: &mut ProtoChunk) -> Result<usize, DecodeError> {

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
