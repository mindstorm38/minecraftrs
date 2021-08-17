use std::sync::Arc;
use std::io::Read;

use nbt::decode::{read_compound_tag, TagDecodeError};
use nbt::{CompoundTag, Tag, CompoundTagError};
use thiserror::Error;

use crate::world::chunk::{Chunk, ChunkStatus, SubChunk};
use crate::world::level::LevelEnv;
use crate::util::PackedIterator;
use crate::block::BlockState;
use crate::biome::Biome;
use std::time::Instant;


#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unknown block state '{0}' in the palette for the chunk environments.")]
    UnknownBlockState(String),
    #[error("Unknown property value: {0}")]
    UnknownBlockProperty(String),
    #[error("Unknown biome ID: {0}")]
    UnknownBiome(u8),
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
pub fn decode_chunk(tag_root: CompoundTag, chunk: &mut Chunk) -> Result<(), DecodeError> {

    let global_start = Instant::now();

    let data_version = tag_root.get_i32("DataVersion")?;
    let tag_level = tag_root.get_compound_tag("Level")?;

    let actual_cx = tag_level.get_i32("xPos")?;
    let actual_cz = tag_level.get_i32("zPos")?;
    let (cx, cz) = chunk.get_position();

    if actual_cx != cx || actual_cz != cz {
        return Err(DecodeError::Malformed(
            format!("Incoherent position given by the chunk NBT, expected {}/{}, got {}/{}.",
                    cx, cz, actual_cx, actual_cz)));
    }

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

    // Biomes are computed before section decoding, biome references are stored in a
    // vec of 64 or `height.len() * 64` biomes, this depends on the used format.
    let biomes: Option<Vec<&'static Biome>> = match tag_level.get_i32_vec("Biomes").ok() {
        Some(raw_biomes) => {

            if raw_biomes.len() == 256 {

                static FROM_2D_INDICES: [usize; 16] = [
                    0, 4, 8, 12,
                    64, 68, 72, 76,
                    128, 132, 136, 140,
                    192, 196, 200, 204
                ];

                let mut vec = Vec::with_capacity(64);

                for _ in 0..4 {
                    for idx in FROM_2D_INDICES {
                        let id = raw_biomes[idx] as u8;
                        vec.push(crate::biome::legacy::from_id(id)
                            .ok_or_else(|| DecodeError::UnknownBiome(id))?);
                    }
                }

                Some(vec)

            } else if raw_biomes.len() == height.len() * 64 {

                let mut vec = Vec::with_capacity(raw_biomes.len());

                for id in raw_biomes {
                    vec.push(crate::biome::legacy::from_id(*id as u8)
                        .ok_or_else(|| DecodeError::UnknownBiome(*id as u8))?);
                }

                Some(vec)

            } else {
                return Err(DecodeError::Malformed(format!("Malformed biomes array of length {}.", raw_biomes.len())));
            }

        },
        None => None
    };

    // Sections
    let start = Instant::now();
    for tag_section in tag_level.get_compound_tag_vec("Sections")? {

        let cy = tag_section.get_i8("Y")?;

        if let Some(chunk_offset) = chunk.get_chunk_offset(cy) {

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

            if let Some(biomes) = &biomes {
                if biomes.len() == 64 {
                    sub_chunk.set_biomes(biomes.iter().copied());
                } else {
                    let sub_chunk_biomes = &biomes[(chunk_offset * 64)..((chunk_offset + 1) * 64)];
                    sub_chunk.set_biomes(sub_chunk_biomes.into_iter().copied());
                }
            }

            chunk.replace_sub_chunk(cy, sub_chunk);

        }

    }

    println!("time to process sections: {}ms", start.elapsed().as_secs_f32() * 1000.0);
    println!("time to process chunk: {}ms", global_start.elapsed().as_secs_f32() * 1000.0);

    Ok(())

}


fn decode_block_state(tag_block: &CompoundTag, env: &LevelEnv) -> Result<&'static BlockState, DecodeError> {

    let block_name = tag_block.get_str("Name")?;

    let mut block_state = env.blocks
        .get_state_from_name(block_name)
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


fn encode_block_state(state: &'static BlockState) -> CompoundTag {

    let block = state.get_block();
    let mut tag_block = CompoundTag::new();
    tag_block.insert_str("Name", block.get_name());

    if let Some(it) = state.iter_raw_states() {
        let mut tag_props = CompoundTag::new();
        for (name, value) in it {
            tag_props.insert_str(name, value);
        }
        tag_block.insert_compound_tag("Properties", tag_props);
    }

    tag_block

}
