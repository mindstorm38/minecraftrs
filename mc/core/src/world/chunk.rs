use std::sync::Arc;

use thiserror::Error;

use crate::util::{PackedArray, Palette, cast_vec_ref_to_ptr};
use crate::block::{BlockState};
use crate::biome::Biome;
use super::level::LevelEnv;


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const BLOCKS_DATA_SIZE: usize = SIZE * SIZE * SIZE;
/// The total count of biomes samples for 3 dimensional biomes.
pub const BIOMES_DATA_SIZE: usize = 4 * 4 * 4;


/// An error enumeration for all operations related to blocks and biomes on
/// sub chunks and chunks. Check each variation for specific documentation.
#[derive(Error, Debug)]
pub enum ChunkError {
    #[error("You are trying to alter a sub chunk that is out of the height range.")]
    SubChunkOutOfRange,
    #[error("You are trying get information from an unloaded sub chunk.")]
    SubChunkUnloaded,
    #[error("You gave a block reference that is not supported by this chunk's level's env.")]
    IllegalBlock,
    #[error("You gave a biome reference that is not supported by this chunk's level's env.")]
    IllegalBiome,
    #[error("You are trying to access an unloaded chunk.")]
    ChunkUnloaded  // Made for LevelStorage
}

/// A type alias with an error type of `ChunkError`.
pub type ChunkResult<T> = Result<T, ChunkError>;


/// An enumeration for the different generation status used by the game. Depending on the
/// generation algorithm some status might not be used.
#[derive(Debug, Copy, Clone)]
pub enum ChunkStatus {
    Empty,
    StructureStarts,
    StructureReferences,
    Biomes,
    Noise,
    Surface,
    Carvers,
    LiquidCarvers,
    Features,
    Light,
    Spawn,
    Heightmaps,
    Full
}


/// A vertical chunk, 16x16 blocks. This vertical chunk is composed of multiple `SubChunk`s,
/// each sub chunk has stores blocks, biomes, lights. This structure however stores metadata
/// about the chunk like inhabited time and generation status. All entities and block entities
/// are also stored in the chunk.
pub struct Chunk {
    /// Chunk X position.
    cx: i32,
    /// Chunk Z position.
    cz: i32,
    /// The level environment with block and biomes global palettes.
    env: Arc<LevelEnv>,
    /// The array of sub chunks, defined once with a specific size depending on the height.
    sub_chunks: Vec<Option<SubChunk>>,
    /// The offset of the
    sub_chunks_offset: i8,
    /// The current generation status of this chunk.
    status: ChunkStatus,
    /// Total number of ticks players has been in this chunk, this increase faster when
    /// more players are in the chunk.
    inhabited_time: u64
}

impl Chunk {

    pub(super) fn new(env: Arc<LevelEnv>, height: ChunkHeight, cx: i32, cz: i32) -> Self {

        Chunk {
            cx,
            cz,
            env,
            status: ChunkStatus::Empty,
            sub_chunks: (0..height.len()).map(|_| None).collect(),
            sub_chunks_offset: height.min,
            inhabited_time: 0
        }

    }

    /// Get the chunk position `(x, z)`.
    #[inline]
    pub fn get_position(&self) -> (i32, i32) {
        (self.cx, self.cz)
    }

    #[inline]
    pub fn get_env(&self) -> &LevelEnv {
        &self.env
    }

    #[inline]
    pub fn clone_env(&self) -> Arc<LevelEnv> {
        Arc::clone(&self.env)
    }

    #[inline]
    pub fn get_status(&self) -> ChunkStatus {
        self.status
    }

    #[inline]
    pub fn set_status(&mut self, status: ChunkStatus) {
        self.status = status;
    }

    #[inline]
    pub fn get_inhabited_time(&self) -> u64 {
        self.inhabited_time
    }

    #[inline]
    pub fn set_inhabited_time(&mut self, time: u64) {
        self.inhabited_time = time;
    }

    /// Ensure that a sub chunk is existing at a specific chunk-Y coordinate, if this coordinate
    /// is out of the height of the level, `Err(ChunkError::IllegalVerticalPos)` is returned.
    /// You can pass `Some(&SubChunkOptions)` in order to change the default block and biome used
    /// to fill the sub chunk if it need to be created.
    pub fn ensure_sub_chunk(&mut self, cy: i8, options: Option<&SubChunkOptions>) -> ChunkResult<&mut SubChunk> {

        let offset = self.calc_chunk_offset(cy).ok_or(ChunkError::SubChunkOutOfRange)?;

        match self.sub_chunks.get_mut(offset) {
            Some(Some(sub_chunk)) => Ok(sub_chunk),
            Some(sub_chunk ) => {
                Ok(sub_chunk.insert(SubChunk::new(Arc::clone(&self.env), options)?))
            },
            None => Err(ChunkError::SubChunkOutOfRange)
        }

    }

    /// Replace the sub chunk at the given Y chunk coordinate by the given one.
    ///
    /// # Panics (debug only):
    /// The method panics if the given sub chunk has not the same sub chunk environment as self.
    pub fn replace_sub_chunk(&mut self, cy: i8, sub_chunk: SubChunk) -> ChunkResult<&mut SubChunk> {
        debug_assert!(Arc::ptr_eq(&self.env, &sub_chunk.env));
        let offset = self.calc_chunk_offset(cy).ok_or(ChunkError::SubChunkOutOfRange)?;
        let container = self.sub_chunks.get_mut(offset).ok_or(ChunkError::SubChunkOutOfRange)?;
        Ok(container.insert(sub_chunk))
    }

    /// Get a sub chunk reference at a specified index.
    pub fn get_sub_chunk(&self, cy: i8) -> Option<&SubChunk> {
        let offset = self.calc_chunk_offset(cy)?;
        match self.sub_chunks.get(offset) {
            Some(Some(chunk)) => Some(chunk),
            _ => None
        }
    }

    /// Get a sub chunk mutable reference at a specified index.
    pub fn mut_sub_chunk(&mut self, cy: i8) -> Option<&mut SubChunk> {
        let offset = self.calc_chunk_offset(cy)?;
        match self.sub_chunks.get_mut(offset) {
            Some(Some(chunk)) => Some(chunk),
            _ => None
        }
    }

    /// Return the number of sub chunks in the height of this chunk.
    pub fn get_sub_chunks_count(&self) -> usize {
        self.sub_chunks.len()
    }

    /// Return the configured height for the level owning this chunk.
    pub fn get_height(&self) -> ChunkHeight {
        ChunkHeight {
            min: self.sub_chunks_offset,
            max: self.sub_chunks_offset + self.sub_chunks.len() as i8 - 1
        }
    }

    /// Return the linear non-negative offset of the chunk at the given position,
    /// the position can be negative. None is returned if overflow occurred.
    ///
    /// As its name implies, this method only calculate the offset, the returned
    /// offset might be out of the chunk's height.
    pub fn calc_chunk_offset(&self, cy: i8) -> Option<usize> {
        cy.checked_sub(self.sub_chunks_offset).map(|v| v as usize)
    }

    /// Return the linear non-negative offset of the chunk at the given position,
    /// the position can be negative. None is returned if the chunk position is
    /// not within the chunk's height.
    pub fn get_chunk_offset(&self, cy: i8) -> Option<usize> {
        match self.calc_chunk_offset(cy) {
            Some(off) if off < self.sub_chunks.len() => Some(off),
            _ => None
        }
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

    #[inline]
    pub fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState> {
        self.get_block((x & 15) as u8, y, (z & 15) as u8)
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

    #[inline]
    pub fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()> {
        self.set_block((x & 15) as u8, y, (z & 15) as u8, state)
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


/// Options used when constructing a new `SubChunk`.
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
pub struct SubChunk {
    env: Arc<LevelEnv>,
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

// We must unsafely implement Send + Sync because of the `Palette<*const BlockState>`, this field
// is safe because it references a 'static reference to a lazy loaded `BlockState`.
unsafe impl Send for SubChunk {}
unsafe impl Sync for SubChunk {}

// Palette capacity must ensure that most of the chunks will be supported AND that the palette
// lookup is fast, but because the lookup is O(N) we have to find a balanced capacity.
const BLOCKS_PALETTE_CAPACITY: usize = 128;


impl SubChunk {

    /// Build a new sub chunk, you can pass Some SubChunkOptions if you want to change the
    /// default block state or biome.
    ///
    /// If no options is given or both `SubChunkOptions::default_block` and
    /// `SubChunkOptions::default_biome` are None, then the method will never return an
    /// `ChunkError`.
    pub fn new(env: Arc<LevelEnv>, options: Option<&SubChunkOptions>) -> ChunkResult<Self> {

        let default_state = match options {
            Some(SubChunkOptions { default_block: Some(default_block), .. }) => {
                env.blocks.check_state(*default_block, || ChunkError::IllegalBlock)?
            },
            _ => {
                // SAFETY: Here we can unwrap because the level has already checked that the
                //         global palettes contains at least one state.
                env.blocks.get_state_from(0).unwrap()
            }
        };

        let default_biome = match options {
            Some(SubChunkOptions { default_biome: Some(default_biome), .. }) => {
                env.biomes.get_sid_from(*default_biome).ok_or_else(|| ChunkError::IllegalBiome)?
            },
            _ => 0
        };

        let biomes_byte_size = Self::get_biomes_byte_size(&env);

        // The palettes are initialized with an initial state and biome (usually air and void, if
        // vanilla environment), this is required because the packed array has default values of 0,
        // and they must have a corresponding valid value in palettes, at least at the beginning.
        Ok(SubChunk {
            env,
            blocks_palette: Some(Palette::new(Some(default_state), BLOCKS_PALETTE_CAPACITY)),
            blocks: PackedArray::new(BLOCKS_DATA_SIZE, 4, None),
            biomes: PackedArray::new(BIOMES_DATA_SIZE, biomes_byte_size, Some(default_biome as u64))
        })

    }

    /// Build a new sub chunk using the default block and default biome to fill it.
    pub fn new_default(env: Arc<LevelEnv>) -> Self {
        Self::new(env, None).unwrap()
    }

    #[inline]
    fn get_blocks_byte_size(env: &LevelEnv) -> u8 {
        PackedArray::calc_min_byte_size(env.blocks.states_count() as u64 - 1)
    }

    #[inline]
    fn get_biomes_byte_size(env: &LevelEnv) -> u8 {
        // This is 7 bits for current vanilla biomes, this only take 64 octets of storage.
        // Subtracting 1 because the maximum biome save ID is the maximum value.
        PackedArray::calc_min_byte_size(env.biomes.biomes_count() as u64 - 1)
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
            None => self.env.blocks.get_state_from(sid).unwrap()
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

        if let Some(ref mut palette) = self.blocks_palette {

            let state_ptr = state as *const BlockState;

            // Here we intentionally dont use palette.ensure_index, because we want to check if
            // the given state is supported by the underlying global palette before actually
            // inserting it to the local palette.
            match palette.search_index(state_ptr) {
                Some(sid) => return Some(sid as u32),
                None => {
                    if self.env.blocks.has_state(state) {
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

        self.env.blocks.get_sid_from(state)

    }

    fn use_global_blocks(&mut self) {
        if let Some(ref local_palette) = self.blocks_palette {
            let new_byte_size = Self::get_blocks_byte_size(&self.env);
            let global_palette = &self.env.blocks;
            self.blocks.resize_byte_and_replace(new_byte_size, move |sid| {
                let state = local_palette.get_item(sid as usize).unwrap();
                global_palette.get_sid_from(unsafe { std::mem::transmute(state) }).unwrap() as u64
            });
            self.blocks_palette = None;
        }
    }

    /// # Safety:
    /// You must ensure that the given palette contains only valid states for this
    /// chunk's level's environment.
    ///
    /// The blocks iterator must return only valid indices for the given palette.
    pub unsafe fn set_blocks_raw<I>(&mut self, palette: Vec<&'static BlockState>, mut blocks: I)
    where
        I: Iterator<Item = u64>
    {

        assert_ne!(palette.len(), 0, "Palette length is zero.");

        if palette.len() <= BLOCKS_PALETTE_CAPACITY {

            let palette = cast_vec_ref_to_ptr(palette);
            let byte_size = PackedArray::calc_min_byte_size(palette.len() as u64 - 1).max(4);
            self.blocks_palette.insert(Palette::from_raw(palette, BLOCKS_PALETTE_CAPACITY));

            // We resize raw because we don't care of the old content, and the new byte size might
            // be smaller than the old one.
            self.blocks.resize_raw(byte_size);
            self.blocks.replace(|_| {
                blocks.next().unwrap_or(0)
            });

        } else {

            self.blocks_palette = None;

            let byte_size = Self::get_blocks_byte_size(&self.env);
            let global_blocks = &self.env.blocks;

            self.blocks.resize_raw(byte_size);
            self.blocks.replace(|_| {
                let palette_idx = blocks.next().unwrap_or(0);
                let state = palette[palette_idx as usize];
                // SAFETY: We can unwrap because this is an safety condition of the method.
                global_blocks.get_sid_from(state).unwrap() as u64
            });

        }

    }

    // BIOMES //

    pub fn get_biome(&self, x: u8, y: u8, z: u8) -> &'static Biome {
        let sid = self.biomes.get(calc_biome_index(x >> 2, y >> 2, z >> 2)).unwrap() as u16;
        self.env.biomes.get_biome_from(sid).unwrap()
    }

    pub fn set_biome(&mut self, x: u8, y: u8, z: u8, biome: &'static Biome) -> ChunkResult<()> {
        let idx = calc_biome_index(x >> 2, y >> 2, z >> 2);
        match self.env.biomes.get_sid_from(biome) {
            Some(sid) => {
                self.biomes.set(idx, sid as u64);
                Ok(())
            },
            None => Err(ChunkError::IllegalBiome)
        }
    }

    pub fn set_biomes<I>(&mut self, mut biomes: I)
    where
        I: Iterator<Item = &'static Biome>,
        I: ExactSizeIterator
    {

        assert_eq!(biomes.len(), BIOMES_DATA_SIZE);
        let global_biomes = &self.env.biomes;

        self.biomes.replace(|v| {
            // SAFETY: Unwrap should be safe to use because we have checked length.
            let biome: &'static Biome = biomes.next().unwrap();
            // If the biome is invalid, we keep the old biome.
            global_biomes.get_sid_from(biome).map(|sid| sid as u64).unwrap_or(v)
        });

    }

}


#[derive(Debug, Clone, Copy)]
pub struct ChunkHeight {
    pub min: i8,
    pub max: i8
}

impl ChunkHeight {

    /// Return `true` if the given chunk Y coordinate is valid for the specified height.
    #[inline]
    pub fn contains(self, cy: i8) -> bool {
        return self.min <= cy && cy <= self.max;
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

}


#[inline]
fn calc_block_index(x: u8, y: u8, z: u8) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16, "x: {}, y: {}, z: {}", x, y, z);
    x as usize | ((z as usize) << 4) | ((y as usize) << 8)
}


#[inline]
fn calc_biome_index(x: u8, y: u8, z: u8) -> usize {
    debug_assert!(x < 4 && y < 4 && z < 4, "x: {}, y: {}, z: {}", x, y, z);
    x as usize | ((z as usize) << 2) | ((y as usize) << 4)
}
