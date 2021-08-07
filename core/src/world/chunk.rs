use std::sync::{Arc, Weak, RwLock};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::util::packed::{PackedArray, Palette};
use crate::block::BlockState;
use crate::biome::Biome;

use super::level::{LevelEnv, Level, LevelHeight};


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const BLOCKS_DATA_SIZE: usize = SIZE * SIZE * SIZE;
/// The total count of biomes samples for 3 dimensional biomes.
pub const BIOMES_2D_DATA_SIZE: usize = SIZE * SIZE;
/// The total count of biomes samples for 3 dimensional biomes.
pub const BIOMES_3D_DATA_SIZE: usize = 4 * 4 * 4;


/// Used to calculate the index in a data array of `BLOCKS_DATA_SIZE`.
#[inline]
fn calc_block_index(x: u8, y: u8, z: u8) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16, "x: {}, y: {}, z: {}", x, y, z);
    x as usize | ((z as usize) << 4) | ((y as usize) << 8)
}


/*#[inline]
fn calc_biome_2d_index(x: u8, z: u8) -> usize {
    debug_assert!(x < 16 && z < 16, "x: {}, z: {}", x, z);
    x as usize | ((z as usize) << 4)
}*/


#[inline]
fn calc_biome_3d_index(x: u8, y: u8, z: u8) -> usize {
    debug_assert!(x < 4 && y < 4 && z < 4, "x: {}, y: {}, z: {}", x, y, z);
    x as usize | ((z as usize) << 2) | ((y as usize) << 4)
}


#[derive(Debug)]
pub enum ChunkError {
    IllegalVerticalPos,
    SubChunkUnloaded,
    IllegalBlock,
    IllegalBiome
}


pub type ChunkResult<T> = Result<T, ChunkError>;


/// A vertical chunk, 16x16 blocks.
pub struct Chunk<'env> {
    level: Weak<RwLock<Level<'env>>>,
    env: &'env LevelEnv,
    height: LevelHeight,
    cx: i32,
    cz: i32,
    populated: bool,
    sub_chunks: HashMap<i8, SubChunk<'env>>
}

impl<'env> Chunk<'env> {

    pub(super) fn new(level: Weak<RwLock<Level<'env>>>, cx: i32, cz: i32) -> Self {

        let level_arc = level.upgrade().unwrap();
        let level_guard = level_arc.read().unwrap();

        Chunk {
            level,
            env: level_guard.get_env(),
            height: level_guard.get_height(),
            cx,
            cz,
            populated: false,
            sub_chunks: HashMap::new()
        }

    }

    /// Return a strong counted reference to the `Level` owning this chunk.
    ///
    /// # Panics
    /// This method panic if this chunk is no longer owned (should not happen).
    pub fn get_level(&self) -> Arc<RwLock<Level<'env>>> {
        self.level.upgrade().expect("This chunk is no longer owned by its level.")
    }

    /// Get the chunk position `(x, z)`.
    pub fn get_position(&self) -> (i32, i32) {
        (self.cx, self.cz)
    }

    pub fn is_populated(&self) -> bool {
        self.populated
    }

    pub fn set_populated(&mut self, populated: bool) {
        self.populated = populated;
    }

    /// Ensure that a sub chunk is existing at a specific chunk-Y coordinate, if this coordinate
    /// is out of the height of the level, `Err(ChunkError::IllegalVerticalPos)` is returned.
    /// You can pass `Some(&SubChunkOptions)` in order to change the default block and biome used
    /// to fill the sub chunk if it need to be created.
    pub fn ensure_sub_chunk<'a, 'b>(
        &'a mut self,
        cy: i8,
        options: Option<&'b SubChunkOptions>
    ) -> ChunkResult<&'a mut SubChunk<'env>> {

        if self.height.includes(cy) {
            match self.sub_chunks.entry(cy) {
                Entry::Occupied(o) => Ok(o.into_mut()),
                Entry::Vacant(v) => {
                    Ok(v.insert(SubChunk::new(self.env, options)?))
                }
            }
        } else {
            Err(ChunkError::IllegalVerticalPos)
        }

    }

    /// Get a sub chunk reference at a specified index.
    pub fn get_sub_chunk(&self, cy: i8) -> Option<&SubChunk<'env>> {
        self.sub_chunks.get(&cy)
    }

    /// Get a sub chunk mutable reference at a specified index.
    pub fn mut_sub_chunk(&mut self, cy: i8) -> Option<&mut SubChunk<'env>> {
        self.sub_chunks.get_mut(&cy)
    }

    /// Return the number of sub chunks in the height of this chunk.
    pub fn get_sub_chunks_count(&self) -> usize {
        self.sub_chunks.len()
    }

    /// Return the configured height for the level owning this chunk.
    pub fn get_height(&self) -> LevelHeight {
        return self.height
    }

    // BLOCKS //

    /// Get the actual block at a specific position.
    ///
    /// Returns `Ok(&BlockState)` if the block is found and valid,
    /// `Err(ChunkError::SubChunkUnloaded)` if no sub chunk is loaded at the given coordinates.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn get_block(&self, x: u8, y: i32, z: u8) -> ChunkResult<&'static BlockState> {
        match self.get_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::SubChunkUnloaded),
            Some(sub_chunk) => Ok(sub_chunk.get_block(x, (y & 15) as u8, z))
        }
    }

    /// Set the block at a specific position relative to this chunk.
    ///
    /// Return `Ok(())` if the biome was successfully set, `Err(ChunkError::IllegalVerticalPos)` if
    /// the given Y coordinate is invalid for the level or `Err(ChunkError::IllegalBlock)` if the
    /// given block state is not registered in the current world..
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn set_block(&mut self, x: u8, y: i32, z: u8, state: &'static BlockState) -> ChunkResult<()> {
        self.ensure_sub_chunk((y >> 4) as i8, None)?
            .set_block(x, (y & 15) as u8, z, state)
    }

    // BIOMES //

    pub fn get_biome(&self, x: u8, y: i32, z: u8) -> ChunkResult<&'static Biome> {
        match self.get_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::SubChunkUnloaded),
            Some(sub_chunk) => Ok(sub_chunk.get_biome(x, (y & 15) as u8, z))
        }
    }

    pub fn set_biome(&mut self, x: u8, y: i32, z: u8, biome: &'static Biome) -> ChunkResult<()> {
        self.ensure_sub_chunk((y >> 4) as i8, None)?
            .set_biome(x, (y & 15) as u8, z, biome)
    }

}


/// Options structure used when constructing a new `SubChunk`.
pub struct SubChunkOptions {
    /// The default block state used to fill
    pub default_block: Option<&'static BlockState>,
    pub default_biome: Option<&'static Biome>
}

impl Default for SubChunkOptions {
    fn default() -> Self {
        Self {
            default_block: None,
            default_biome: None
        }
    }
}


/// A sub chunk, 16x16x16 blocks.
pub struct SubChunk<'env> {
    env: &'env LevelEnv,
    /// Blocks palette. It is limited to 128 blocks in order to support most blocks in a natural
    /// generation, which is the case of most of the sub chunks that are untouched by players.
    /// In case of "artificial chunks" made by players, the block palette is likely to overflow
    /// the 128 block states limit, in this case it switch to the global palette (`GlobalBlocks`
    /// in the level environment).
    blocks_palette: Option<Palette<*const BlockState>>,
    /// Cube blocks array.
    blocks: PackedArray,
    /// Modern cube biomes array, this array does not use any palette since the global palette
    /// should be small enough to only take 7 to 8 bits per point. Since there 64 biomes in
    /// a sub chunk, this make only 64 octets to store.
    biomes: PackedArray,
}

impl<'env> SubChunk<'env> {

    fn new(env: &'env LevelEnv, options: Option<&SubChunkOptions>) -> ChunkResult<Self> {

        let default_state = match options {
            Some(SubChunkOptions { default_block: Some(default_block), .. }) => {
                env.blocks().check_state(*default_block, || ChunkError::IllegalBlock)?
            },
            _ => {
                // SAFETY: Here we can unwrap because the level has already check that the
                //         global palettes contains at least one state.
                env.blocks().get_state_from(0).unwrap()
            }
        };

        let default_biome = match options {
            Some(SubChunkOptions { default_biome: Some(default_biome), .. }) => {
                env.biomes().get_sid_from(*default_biome).ok_or_else(|| ChunkError::IllegalBiome)?
            },
            _ => 0
        };

        // This is 7 bits for current vanilla biomes, this only take 64 octets of storage.
        let biomes_byte_size = PackedArray::calc_min_byte_size(env.biomes().biomes_count() as u64);

        // The palettes are initialized with an initial state and biome (usually air and void, if
        // vanilla environment), this is required because the packed array has default values of 0,
        // and they must have a corresponding valid value in palettes, at least at the beginning.
        Ok(SubChunk {
            env,
            blocks_palette: Some(Palette::new(Some(default_state), 128)),
            blocks: PackedArray::new(BLOCKS_DATA_SIZE, 4, None),
            biomes: PackedArray::new(BIOMES_3D_DATA_SIZE, biomes_byte_size, Some(default_biome as u64))
        })

    }

    // BLOCKS //

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> &'static BlockState {

        // SAFETY: The unwrap should be safe because the block index is expected to be right,
        //         moreover it is checked in debug mode.
        let sid = self.blocks.get(calc_block_index(x, y, z)).unwrap() as u32;

        // SAFETY: The unwraps should be safe in this case because the `set_block` ensure
        //         that the palette is updated according to the blocks contents. If the user
        //         directly manipule the contents of palette or blocks array it MUST ensure
        //         that is correct, if not this will panic here.
        match self.blocks_palette {
            Some(ref palette) => unsafe {
                // Here we transmute a `*const BlockState` to `&'static BlockState`, this is safe.
                std::mem::transmute(palette.get_item(sid as usize).unwrap())
            },
            None => self.env.blocks().get_state_from(sid).unwrap()
        }

    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, state: &'static BlockState) -> ChunkResult<()> {

        let idx = calc_block_index(x, y, z);
        match self.ensure_block_sid(state) {
            Some(sid) => {
                self.blocks.set(idx, sid as u64);
                Ok(())
            },
            None => Err(ChunkError::IllegalBlock)
        }

    }

    fn ensure_block_sid(&mut self, state: &'static BlockState) -> Option<u32> {

        let global_palette = self.env.blocks();

        if let Some(ref mut palette) = self.blocks_palette {

            let state_ptr = state as *const BlockState;

            // Here we intentionally dont use palette.ensure_index, because we want to check if
            // the given state is supported by the underlying global palette before actually
            // inserting it to the local palette.
            match palette.search_index(state_ptr) {
                Some(sid) => return Some(sid as u32),
                None => {
                    if global_palette.has_state(state) {
                        match palette.insert_index(state_ptr) {
                            Some(sid) => {
                                if sid as u64 > self.blocks.max_value() {
                                    self.blocks.resize_byte(self.blocks.byte_size() + 1);
                                }
                                return Some(sid as u32);
                            },
                            None => {
                                // In this case, the local palette is full, we have to switch to
                                // the global one. So we don't return anything to skip the match.
                            }
                        }
                    } else {
                        return None;
                    }
                }
            }

            self.use_global_blocks();

        }

        global_palette.get_sid_from(state)

    }

    fn use_global_blocks(&mut self) {
        if let Some(ref local_palette) = self.blocks_palette {
            let global_palette = self.env.blocks();
            let new_byte_size = PackedArray::calc_min_byte_size(global_palette.states_count() as u64);
            self.blocks.resize_byte_and_replace(new_byte_size, move |sid| unsafe {
                global_palette.get_sid_from(std::mem::transmute(
                    local_palette.get_item(sid as usize).unwrap()
                )).unwrap() as u64
            });
            self.blocks_palette = None;
        }
    }

    // BIOMES //

    pub fn get_biome(&self, x: u8, y: u8, z: u8) -> &'static Biome {
        let sid = self.biomes.get(calc_biome_3d_index(x >> 2, y >> 2, z >> 2)).unwrap() as u16;
        self.env.biomes().get_biome_from(sid).unwrap()
    }

    pub fn set_biome(&mut self, x: u8, y: u8, z: u8, biome: &'static Biome) -> ChunkResult<()> {
        let idx = calc_biome_3d_index(x >> 2, y >> 2, z >> 2);
        match self.env.biomes().get_sid_from(biome) {
            Some(sid) => {
                self.biomes.set(idx, sid as u64);
                Ok(())
            },
            None => Err(ChunkError::IllegalBiome)
        }
    }

}
