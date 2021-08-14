use std::io::Read;

use nbt::decode::{read_compound_tag, TagDecodeError};
use nbt::{CompoundTag, Tag, CompoundTagError};
use thiserror::Error;

use crate::world::chunk::{Chunk, ChunkStatus};


#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unknown block state '{0}' in the palette for the chunk environments.")]
    UnknownBlockState(String),
    #[error("Unknown property value: {0}")]
    UnknownBlockProperty(String),
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
pub fn decode_chunk_from_reader(reader: &mut impl Read, chunk: &mut Chunk) -> Result<(), DecodeError> {
    decode_chunk(read_compound_tag(reader)?, chunk)
}

/// Decode a chunk from its NBT data.
pub fn decode_chunk(root: CompoundTag, chunk: &mut Chunk) -> Result<(), DecodeError> {

    let _data_version = root.get_i32("DataVersion")?;
    let level = root.get_compound_tag("Level")?;

    let actual_cx = level.get_i32("xPos")?;
    let actual_cz = level.get_i32("zPos")?;
    let (cx, cz) = chunk.get_position();

    if actual_cx != cx || actual_cz != cz {
        return Err(DecodeError::Malformed(
            format!("Incoherent position given by the chunk NBT, expected {}/{}, got {}/{}.",
                    cx, cz, actual_cx, actual_cz)));
    }

    chunk.set_status(match level.get_str("Status")? {
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

    let sections = level.get_compound_tag_vec("Sections")?;
    for section in sections {
        let cy = section.get_i8("Y")?;
        if chunk.get_height().contains(cy) {

            println!("==== SECTION {} ====", cy);

            if let Ok(palette) = section.get_compound_tag_vec("Palette") {

                let mut palette_states = Vec::new();

                for block in palette {

                    let block_name = block.get_str("Name")?;

                    let mut block_state = chunk.get_env().blocks
                        .get_state_from_name(block_name)
                        .ok_or_else(|| DecodeError::UnknownBlockState(block_name.to_string()))?;

                    if let Ok(block_properties) = block.get_compound_tag("Properties") {
                        for (prop_name, prop_value_tag) in block_properties.iter() {
                            if let Tag::String(prop_value) = prop_value_tag {
                                block_state = block_state
                                    .with_raw(prop_name, &prop_value)
                                    .ok_or_else(|| DecodeError::UnknownBlockProperty(format!("{}/{}={}", block_name, prop_name, prop_value)))?;
                            }
                        }
                    }

                    palette_states.push(block_state);

                }

                println!("palette_states: {}", palette_states.len().next_power_of_two());

            }

        }
    }

    Ok(())

}
