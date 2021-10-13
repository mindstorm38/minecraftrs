use crate::world::chunk::Chunk;
use crate::block::BlockState;
use nbt::CompoundTag;


pub fn encode_chunk(tag_root: &mut CompoundTag, chunk: &mut Chunk) {

    let (cx, cz) = chunk.get_position();

    tag_root.insert_i32("DataVersion", 2586);

    tag_root.insert_compound_tag("Level", {

        let mut tag_level = CompoundTag::new();

        tag_level.insert_i32("xPos", cx);
        tag_level.insert_i32("zPos", cz);
        // Core crate don't support status, support for status is not planned in near future
        // because chunks should be generated independently, this is opposed to Notchian gen.
        tag_level.insert_str("Status", "full");

        tag_level.insert_i32_vec("Biomes", {
            chunk.iter_biomes().map(|biome| biome.get_id()).collect()
        });

        tag_level.insert_compound_tag_vec("Sections", {
            chunk.iter_loaded_sub_chunks()
                .map(|(cy, sc)| {
                    let mut tag_section = CompoundTag::new();
                    tag_section.insert_i8("Y", cy);
                    tag_section
                })
        });

        tag_level

    });

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
