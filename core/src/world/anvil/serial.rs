use std::io::Read;

use nbt::decode::{read_compound_tag, TagDecodeError};
use nbt::{CompoundTag, CompoundTagError};
use thiserror::Error;

use crate::world::chunk::{Chunk, ChunkStatus};


#[derive(Error, Debug)]
pub enum DecodeError {
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

    let data_version = root.get_i32("DataVersion")?;
    let level = root.get_compound_tag("Level")?;

    let actual_cx = level.get_i32("xPos")?;
    let actual_cz = level.get_i32("zPos")?;
    let (cx, cz) = chunk.get_position();

    if actual_cx != cx || actual_cz != cz {
        return Err(DecodeError::Malformed(
            format!("Incoherent position given by the chunk NBT, expected {}/{}, got {}/{}.",
                    cx, cz, actual_cx, actual_cz)));
    }

    let status = match level.get_str("Status")? {
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
    };

    let sections = level.get_compound_tag_vec("Sections")?;
    for section in sections {



    }

    Ok(())

}
