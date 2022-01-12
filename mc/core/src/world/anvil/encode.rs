use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::Write;

use crate::util::{PackedArray, PackedIterator};
use crate::world::chunk::{Chunk, ChunkStatus};
use crate::heightmap::HeightmapType;
use crate::block::BlockState;

use nbt::encode::write_compound_tag;
use nbt::CompoundTag;


/// The only supported data version for encoding. Current is `1.18.1`.
pub const DATA_VERSION: i32 = 2865;


/// Decode the NBT data from a reader and delegate chunk decoding to `decode_chunk`.
pub fn encode_chunk_to_writer(writer: &mut impl Write, chunk: &Chunk) {
    let mut root = CompoundTag::new();
    encode_chunk(&mut root, chunk);
    write_compound_tag(writer, &root).unwrap();
}

pub fn encode_chunk(tag_root: &mut CompoundTag, chunk: &Chunk) {

    let (cx, cz) = chunk.get_position();
    let height = chunk.get_height();

    tag_root.insert_i32("DataVersion", DATA_VERSION);
    tag_root.insert_i32("xPos", cx);
    tag_root.insert_i32("yPos", height.min as i32);
    tag_root.insert_i32("zPos", cz);

    tag_root.insert_str("Status", match chunk.get_status() {
        ChunkStatus::Empty => "empty",
        ChunkStatus::StructureStarts => "structure_starts",
        ChunkStatus::StructureReferences => "structure_references",
        ChunkStatus::Biomes => "biomes",
        ChunkStatus::Noise => "noise",
        ChunkStatus::Surface => "surface",
        ChunkStatus::Carvers => "carvers",
        ChunkStatus::LiquidCarvers => "liquid_carvers",
        ChunkStatus::Features => "features",
        ChunkStatus::Light => "light",
        ChunkStatus::Spawn => "spawn",
        ChunkStatus::Heightmaps => "heightmaps",
        ChunkStatus::Full => "full"
    });

    tag_root.insert_compound_tag_vec("sections", {

        let mut biomes_it = chunk.iter_biomes();

        let mut biomes_tmp = Vec::with_capacity(64);
        let mut biomes_indices = HashMap::new();

        let mut block_states_tmp = Vec::with_capacity(4096);
        let mut block_states_indices = HashMap::new();

        height.iter()
            .map(|cy| {

                let mut tag_section = CompoundTag::new();
                tag_section.insert_i8("Y", cy);

                tag_section.insert_compound_tag("biomes", {

                    let mut tag_biomes = CompoundTag::new();

                    let mut biomes_palette = Vec::new();
                    biomes_indices.clear();
                    biomes_tmp.clear();

                    for _ in 0..64 {
                        let biome = biomes_it.next().unwrap();
                        let sid = match biomes_indices.entry(biome.get_key()) {
                            Entry::Occupied(o) => *o.get(),
                            Entry::Vacant(v) => {
                                biomes_palette.push(biome.get_name());
                                *v.insert(biomes_palette.len() - 1)
                            }
                        };
                        biomes_tmp.push(sid);
                    }

                    if biomes_palette.len() > 1 {

                        let bits = PackedArray::calc_min_byte_size((biomes_palette.len() - 1) as u64);
                        tag_biomes.insert_i64_vec("data", biomes_tmp.iter()
                            .map(|&v| v as u64)
                            .pack_aligned(bits)
                            .map(|v| v as i64)
                            .collect());

                    }

                    tag_biomes.insert_str_vec("palette", biomes_palette);

                    tag_biomes

                });

                tag_section.insert_compound_tag("block_states", {

                    let mut tag_block_states = CompoundTag::new();

                    let mut block_states_palette = Vec::new();

                    if let Some(sub_chunk) = chunk.get_sub_chunk(cy) {

                        block_states_indices.clear();
                        block_states_tmp.clear();

                        for block_state in sub_chunk.iter_blocks() {
                            let sid = match block_states_indices.entry(block_state.get_key()) {
                                Entry::Occupied(o) => *o.get(),
                                Entry::Vacant(v) => {
                                    block_states_palette.push(encode_block_state(block_state));
                                    *v.insert(block_states_palette.len() - 1)
                                }
                            };
                            block_states_tmp.push(sid);
                        }

                        if block_states_palette.len() > 1 {

                            let bits = PackedArray::calc_min_byte_size((block_states_tmp.len() - 1) as u64)
                                .max(4); // Minimum byte size of 4

                            tag_block_states.insert_i64_vec("data", block_states_tmp.iter()
                                .map(|&v| v as u64)
                                .pack_aligned(bits)
                                .map(|v| v as i64)
                                .collect());

                        }

                    } else {
                        // Add the null block to the palette, this seems to be a convention.
                        block_states_palette.push(encode_block_state(chunk.get_env().blocks.get_state_from(0).unwrap()));
                    }

                    tag_biomes.insert_str_vec("palette", block_states_palette);

                    tag_block_states

                });

                tag_section

            });

    });

    let mut tag_heightmaps = CompoundTag::new();

    for heightmap_type in chunk.get_env().heightmaps.iter_heightmap_types() {
        if let Some(arr) = encode_heightmap(chunk, heightmap_type) {
            tag_heightmaps.insert_i64_vec(heightmap_type.get_name(), arr);
        }
    }

}

pub fn encode_block_state(state: &'static BlockState) -> CompoundTag {

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

pub fn encode_heightmap(chunk: &Chunk, heightmap_type: &'static HeightmapType) -> Option<Vec<i64>> {
    let (byte_size, it) = chunk.iter_heightmap_raw_columns(heightmap_type)?;
    Some(it.pack_aligned(byte_size).map(|v| v as i64).collect())
}
