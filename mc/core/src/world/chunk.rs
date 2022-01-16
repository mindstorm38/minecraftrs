use std::collections::HashSet;
use std::time::Instant;
use std::sync::Arc;

use thiserror::Error;
use hecs::Entity;

use crate::util::{PackedArray, Palette, Rect, cast_vec_ref_to_ptr};
use crate::heightmap::HeightmapType;
use crate::block::BlockState;
use crate::biome::Biome;
use crate::perf;

use super::level::LevelEnv;


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const BLOCKS_DATA_SIZE: usize = SIZE * SIZE * SIZE;
/// The total count of biomes samples for 3 dimensional biomes.
pub const BIOMES_DATA_SIZE: usize = 4 * 4 * 4;


/// Internal height map used for optimization of the heightmap computation, it allows to ignore
/// useless null blocks (usually air).
static HEIGHTMAP_NON_NULL: HeightmapType = HeightmapType {
    name: "INTERNAL_NON_NULL",
    predicate: |state, blocks| blocks.get_state_from(0).unwrap() != state
};


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
    #[error("You gave a heightmap type that is not supported by this chunk's level's env.")]
    IllegalHeightmap,
    #[error("You are trying to access an unloaded chunk.")]
    ChunkUnloaded,  // Made for LevelStorage
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
    /// The offset of the lower chunk.
    sub_chunks_offset: i8,
    /// Modern cube biomes array, this array does not use any palette since the global palette
    /// should be small enough to only take 7 to 8 bits per point. Since there 64 biomes in
    /// a sub chunk, this make only 64 octets to store a sub chunk.
    ///
    /// Biomes are stored separately from sub chunks because sub chunks are not necessarily
    /// existing for empty sub chunks even at valid Y positions. But biomes must always be
    /// defined for all the height.
    biomes: PackedArray,
    /// Array containing data for all registered heightmaps in the level environment. For memory
    /// efficiency we store all our heightmaps in the same packed array.
    heightmaps: PackedArray,
    /// The current generation status of this chunk.
    status: ChunkStatus,
    /// Total number of ticks players has been in this chunk, this increase faster when
    /// more players are in the chunk.
    inhabited_time: u64,
    /// A list of entity handles that are located in this vertical chunk.
    entities: HashSet<Entity>,
    /// Last save instant.
    last_save: Instant
}

impl Chunk {

    pub(super) fn new(env: Arc<LevelEnv>, height: ChunkHeight, cx: i32, cz: i32) -> Self {

        // This is 7 bits for current vanilla biomes, this only take 64 octets of storage.
        // Subtracting 1 because the maximum biome save ID is the maximum value.
        let biomes_byte_size = PackedArray::calc_min_byte_size(env.biomes.biomes_count() as u64 - 1);

        let heightmap_byte_size = PackedArray::calc_min_byte_size((height.len() * 16) as u64);
        // Added 256 for the additional internal non null heightmap HEIGHTMAP_NON_NULL.
        let heightmap_len = env.heightmaps.heightmaps_count() * 256 + 256;

        Chunk {
            cx,
            cz,
            env,
            status: ChunkStatus::Empty,
            sub_chunks: (0..height.len()).map(|_| None).collect(),
            sub_chunks_offset: height.min,
            biomes: PackedArray::new(height.len() * 64, biomes_byte_size, None),
            heightmaps: PackedArray::new(heightmap_len, heightmap_byte_size, None),
            inhabited_time: 0,
            entities: HashSet::new(),
            last_save: Instant::now()
        }

    }

    /// Get the chunk position `(x, z)`.
    #[inline]
    pub fn get_position(&self) -> (i32, i32) {
        (self.cx, self.cz)
    }

    #[inline]
    pub fn get_env(&self) -> &Arc<LevelEnv> {
        &self.env
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

    #[inline]
    pub fn get_last_save(&self) -> Instant {
        self.last_save
    }

    #[inline]
    pub fn update_last_save(&mut self) {
        self.last_save = Instant::now();
    }

    // Height //

    /// Return the configured height for the level owning this chunk.
    #[inline]
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
    /// offset might be out of the chunk's height. **It is made for internal use.**
    fn calc_sub_chunk_offset(&self, cy: i8) -> Option<usize> {
        cy.checked_sub(self.sub_chunks_offset).map(|v| v as usize)
    }

    /// Return the linear non-negative offset of the chunk at the given position,
    /// the position can be negative. None is returned if the chunk position is
    /// not within the chunk's height.
    pub fn get_sub_chunk_offset(&self, cy: i8) -> Option<usize> {
        match self.calc_sub_chunk_offset(cy) {
            Some(off) if off < self.sub_chunks.len() => Some(off),
            _ => None
        }
    }

    // Sub chunks //

    /// Ensure that a sub chunk is existing at a specific chunk-Y coordinate, if this coordinate
    /// is out of the height of the level, `Err(ChunkError::SubChunkOutOfRange)` is returned.
    pub fn ensure_sub_chunk(&mut self, cy: i8) -> ChunkResult<&mut SubChunk> {

        let offset = self.calc_sub_chunk_offset(cy).ok_or(ChunkError::SubChunkOutOfRange)?;

        match self.sub_chunks.get_mut(offset) {
            Some(Some(sub_chunk)) => Ok(sub_chunk),
            Some(sub_chunk ) => {
                Ok(sub_chunk.insert(SubChunk::new(Arc::clone(&self.env))))
            },
            None => Err(ChunkError::SubChunkOutOfRange)
        }

    }

    #[inline]
    pub fn ensure_sub_chunk_at(&mut self, y: i32) -> ChunkResult<&mut SubChunk> {
        self.ensure_sub_chunk((y >> 4) as i8)
    }

    /// Replace the sub chunk at the given Y chunk coordinate by the given one.
    ///
    /// # Panics (debug only):
    /// The method panics if the given sub chunk has not the same sub chunk environment as self.
    pub fn replace_sub_chunk(&mut self, cy: i8, sub_chunk: SubChunk) -> ChunkResult<&mut SubChunk> {
        debug_assert!(Arc::ptr_eq(&self.env, &sub_chunk.env));
        let offset = self.calc_sub_chunk_offset(cy).ok_or(ChunkError::SubChunkOutOfRange)?;
        let container = self.sub_chunks.get_mut(offset).ok_or(ChunkError::SubChunkOutOfRange)?;
        Ok(container.insert(sub_chunk))
    }

    /// Get a sub chunk reference at a specified index.
    pub fn get_sub_chunk(&self, cy: i8) -> Option<&SubChunk> {
        let offset = self.calc_sub_chunk_offset(cy)?;
        match self.sub_chunks.get(offset) {
            Some(Some(chunk)) => Some(chunk),
            _ => None
        }
    }

    #[inline]
    pub fn get_sub_chunk_at(&self, y: i32) -> Option<&SubChunk> {
        self.get_sub_chunk((y >> 4) as i8)
    }

    /// Get a sub chunk mutable reference at a specified index.
    pub fn get_sub_chunk_mut(&mut self, cy: i8) -> Option<&mut SubChunk> {
        let offset = self.calc_sub_chunk_offset(cy)?;
        match self.sub_chunks.get_mut(offset) {
            Some(Some(chunk)) => Some(chunk),
            _ => None
        }
    }

    #[inline]
    pub fn get_sub_chunk_at_mut(&mut self, y: i32) -> Option<&mut SubChunk> {
        self.get_sub_chunk_mut((y >> 4) as i8)
    }


    /// Return the number of sub chunks in the height of this chunk.
    pub fn get_sub_chunks_count(&self) -> usize {
        self.sub_chunks.len()
    }

    /// Iterator over all sub chunks with their Y coordinate, sub chunks may be not loaded (`None`).
    pub fn iter_sub_chunks(&self) -> impl Iterator<Item = (i8, Option<&'_ SubChunk>)> + '_ {
        let min_y = self.sub_chunks_offset;
        self.sub_chunks.iter()
            .enumerate()
            .map(move |(idx, opt)| {
                (idx as i8 + min_y, opt.as_ref())
            })
    }

    /// Iterator only over loaded sub chunks.
    pub fn iter_loaded_sub_chunks(&self) -> impl Iterator<Item = (i8, &'_ SubChunk)> + '_ {
        let min_y = self.sub_chunks_offset;
        self.sub_chunks.iter()
            .enumerate()
            .filter_map(move |(idx, opt)| {
                opt.as_ref().map(move |sc| (idx as i8 + min_y, sc))
            })
    }

    /// Return the index of the first sub chunk (starting from the top sub chunk) that is loaded
    /// AND contains a non null block.
    pub fn get_highest_non_null_sub_chunk(&self) -> i8 {
        self.sub_chunks.iter()
            .rposition(|sc| {
                match sc {
                    Some(sc) => sc.has_non_null_block(),
                    None => false
                }
            }).unwrap_or(0) as i8 + self.sub_chunks_offset
    }

    // BLOCKS //

    /// Get the block at a specific position.
    ///
    /// Returns `Ok(&BlockState)` if the block is found and valid,
    /// `Err(ChunkError::SubChunkOutOfRange)` if the given y coordinate is out of the chunk
    /// height, if the Y coordinate is valid but the sub chunk is yet loaded, the "null-block"
    /// is returned.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn get_block(&self, x: u8, y: i32, z: u8) -> ChunkResult<&'static BlockState> {

        let offset = self.calc_sub_chunk_offset((y >> 4) as i8)
            .ok_or(ChunkError::SubChunkOutOfRange)?;

        match self.sub_chunks.get(offset) {
            Some(Some(sub_chunk)) => Ok(sub_chunk.get_block(x, (y & 15) as u8, z)),
            Some(None) => Ok(self.env.blocks.get_state_from(0).unwrap()),
            _ => Err(ChunkError::SubChunkOutOfRange)
        }


        /*match self.get_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::SubChunkUnloaded),
            Some(sub_chunk) => Ok(sub_chunk.get_block(x, (y & 15) as u8, z))
        }*/

    }

    /// Same description as `get_block` but accept level coordinates instead of relative ones. This
    /// method ignore if the given coordinates are not actually pointing to this chunk, it only
    /// take the 4 least significant bits.
    #[inline]
    pub fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState> {
        self.get_block((x & 15) as u8, y, (z & 15) as u8)
    }

    /// Set the block at a specific position relative to this chunk.
    ///
    /// Return `Ok(())` if the biome was successfully set, `Err(ChunkError::SubChunkOutOfRange)` if
    /// the given Y coordinate is invalid for the level or `Err(ChunkError::IllegalBlock)` if the
    /// given block state is not registered in the current world.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn set_block(&mut self, x: u8, y: i32, z: u8, state: &'static BlockState) -> ChunkResult<()> {
        let sub_chunk = self.ensure_sub_chunk_at(y)?;
        match sub_chunk.set_block(x, (y & 15) as u8, z, state) {
            Ok(()) => {
                self.update_heightmap_column(x, y, z, state);
                Ok(())
            },
            e => e
        }
    }

    /// Same description as `set_block` but accept level coordinates instead of relative ones. This
    /// method ignore if the given coordinates are not actually pointing to this chunk, it only
    /// take the 4 least significant bits.
    #[inline]
    pub fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()> {
        self.set_block((x & 15) as u8, y, (z & 15) as u8, state)
    }

    // BIOMES //

    fn calc_biome_offset(&self, x: u8, y: i32, z: u8) -> usize {
        calc_biome_index(x, (y - self.sub_chunks_offset as i32 * 4) as usize, z)
    }

    /// Get a biome at specific biome coordinates, biome coordinates are different from block
    /// coordinates because there is only 4x4x4 biomes samples for each sub chunk.
    /// Given X and Z coordinates must be lower than 4 and given Y coordinate must be valid
    /// for the chunk height.
    ///
    /// Returns `Ok(biome)` or `Err(ChunkError::SubChunkOutOfRange)` if the given Y coordinate
    /// is not supported by this chunk's height.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 3.
    pub fn get_biome(&self, x: u8, y: i32, z: u8) -> ChunkResult<&'static Biome> {
        let offset = self.calc_biome_offset(x, y, z);
        let sid = self.biomes.get(offset).ok_or(ChunkError::SubChunkOutOfRange)? as u16;
        Ok(self.env.biomes.get_biome_from(sid).unwrap())
    }

    #[inline]
    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static Biome> {
        self.get_biome(((x >> 2) & 3) as u8, y >> 2, ((z >> 2) & 3) as u8)
    }

    /// Get a biome at specific biome coordinates, biome coordinates are different from block
    /// coordinates because there is only 4x4x4 biomes samples for each sub chunk.
    /// Given X and Z coordinates must be lower than 4 and given Y coordinate must be valid
    /// for the chunk height.
    ///
    /// Returns `Ok(biome)` or `Err(ChunkError::SubChunkOutOfRange)` if the given Y coordinate
    /// is not supported by this chunk's height.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 3.
    pub fn set_biome(&mut self, x: u8, y: i32, z: u8, biome: &'static Biome) -> ChunkResult<()> {
        let offset = self.calc_biome_offset(x, y, z);
        let sid = self.env.biomes.get_sid_from(biome).ok_or(ChunkError::IllegalBiome)?;
        if offset < self.biomes.len() {
            self.biomes.set(offset, sid as u64);
            Ok(())
        } else {
            Err(ChunkError::SubChunkOutOfRange)
        }
    }

    #[inline]
    pub fn set_biome_at(&mut self, x: i32, y: i32, z: i32, biome: &'static Biome) -> ChunkResult<()> {
        self.set_biome(((x >> 2) & 3) as u8, y >> 2, ((z >> 2) & 3) as u8, biome)
    }

    /// Set all biome quads in this chunk according a to a 2D biomes rectangle (16x16).
    /// The implementation currently take the lower coordinates biome in each 4x4 rectangle
    /// and set it to the whole chunk height.
    #[deprecated]
    pub fn set_biomes_2d(&mut self, biomes: &Rect<&'static Biome>) -> ChunkResult<()> {

        assert!(biomes.x_size >= 16 && biomes.z_size >= 16, "Given biomes rectangle is too small.");

        let mut layer_biomes = [0; 16];

        for z in 0..4 {
            for x in 0..4 {
                let idx = x + z * 4;
                let biome = biomes.data[idx * 4];
                layer_biomes[idx] = self.env.biomes.get_sid_from(biome).ok_or(ChunkError::IllegalBiome)?;
            }
        }

        self.biomes.replace(move |i, _| layer_biomes[i % 16] as u64);
        Ok(())

    }

    #[deprecated]
    pub fn set_biomes_3d(&mut self, biomes: &[&'static Biome]) -> ChunkResult<()> {
        assert_eq!(biomes.len(), self.sub_chunks.len() * 64, "Given biomes array must be {} biomes long.", self.sub_chunks.len() * 64);
        let env_biomes = &self.env.biomes;
        self.biomes.replace(move |i, old| {
            env_biomes.get_sid_from(biomes[i]).map(|v| v as u64).unwrap_or(old)
        });
        Ok(())
    }

    /// Set biomes efficiently from a given palette and an indices iterator. You can change the
    /// offset from which you want to change internal biomes, there are 64 biomes for each sub
    /// chunk. **Values before the offset and after the iterator exhausted are kept the same.**
    ///
    /// # Safety:
    /// You must ensure that the given palette contains only valid biomes for this chunk's level's
    /// environment.
    ///
    /// The biomes iterator must return only valid indices for the given palette.
    pub unsafe fn set_biomes_raw<I>(&mut self, offset: usize, palette: Vec<&'static Biome>, mut biomes: I)
    where
        I: Iterator<Item = usize>
    {
        let env_biomes = &self.env.biomes;
        self.biomes.replace(move |i, old| {
            if i >= offset {
                if let Some(biome_idx) = biomes.next() {
                    env_biomes.get_sid_from(palette[biome_idx]).unwrap() as u64
                } else {
                    old
                }
            } else {
                old
            }
        });
    }

    pub fn get_biomes_count(&self) -> usize {
        self.biomes.len()
    }

    /// Expose internal biomes storage, retuning an iterator with each biomes in this chunk,
    /// ordered by X, Z then Y.
    pub fn iter_biomes(&self) -> impl Iterator<Item = &'static Biome> + '_ {
        let biomes = &self.env.biomes;
        self.biomes.iter().map(move |v| biomes.get_biome_from(v as u16).unwrap())
    }

    // HEIGHTMAPS //

    /// Internal method that returns the offset of a given heightmap, if the heightmap is not
    /// known by the environment's heightmaps, `Err(ChunkError::IllegalHeightmap)` is returned.
    fn get_heightmap_column_index(&self, heightmap_type: &'static HeightmapType, x: u8, z: u8) -> ChunkResult<usize> {
        self.env.heightmaps.get_heightmap_index(heightmap_type)
            .map(move |offset| 256 + offset * 256 + calc_heightmap_index(x, z))
            .ok_or(ChunkError::IllegalHeightmap)
    }

    /// Set the value of a specific heightmap at specific coordinates. This is very unsafe to do
    /// that manually, you should ensure that the condition of the given heightmap type are kept.
    pub fn set_heightmap_column(&mut self, heightmap_type: &'static HeightmapType, x: u8, z: u8, y: i32) -> ChunkResult<()> {
        let column_index = self.get_heightmap_column_index(heightmap_type, x, z)?;
        self.heightmaps.set(column_index, (y - self.get_height().get_min_block()) as u64);
        Ok(())
    }

    #[inline]
    pub fn set_heightmap_column_at(&mut self, heightmap_type: &'static HeightmapType, x: i32, z: i32, y: i32) -> ChunkResult<()> {
        self.set_heightmap_column(heightmap_type, (x & 15) as u8, (z & 15) as u8, y)
    }

    /// Get the value of a specific heightmap at specific coordinates.
    pub fn get_heightmap_column(&self, heightmap_type: &'static HeightmapType, x: u8, z: u8) -> ChunkResult<i32> {
        let column_index = self.get_heightmap_column_index(heightmap_type, x, z)?;
        // SAFETY: We unwrap because the column index is checked into `get_heightmap_column_index`
        //         and if x or z are wrong, this panics in debug mode.
        Ok(self.heightmaps.get(column_index).unwrap() as i32 + self.get_height().get_min_block())
    }

    #[inline]
    pub fn get_heightmap_column_at(&self, heightmap_type: &'static HeightmapType, x: i32, z: i32) -> ChunkResult<i32> {
        self.get_heightmap_column(heightmap_type, (x & 15) as u8, (z & 15) as u8)
    }

    /// Efficiently update all heightmaps for a specific column according to a block update.
    /// If you don't have a state change, you can use `recompute_heightmap_column` instead.
    fn update_heightmap_column(&mut self, x: u8, y: i32, z: u8, state: &'static BlockState) {

        let min_block_y = self.get_height().get_min_block();
        let column_index = calc_heightmap_index(x, z);

        for (idx, heightmap_type) in std::iter::once(&HEIGHTMAP_NON_NULL)
            .chain(self.env.heightmaps.iter_heightmap_types())
            .enumerate()
        {
            let column_index = idx * 256 + column_index;
            let current_y = self.heightmaps.get(column_index).unwrap() as i32 + min_block_y;
            if heightmap_type.check_block(state, &self.env.blocks) {
                // For example, if we have `current_y = 0`, `y = 0`, then we set `1`.
                if y >= current_y {
                    self.heightmaps.set(column_index, (y + 1 - min_block_y) as u64);
                }
            } else {
                // For example, if we have `current_y = 2`, `y = 1`, then we recompute.
                // But if we have `y = 0`, then we do nothing because top block is unchanged.
                if y + 1 == current_y {
                    let height = self.recompute_heightmap_column_internal(heightmap_type, x, z, y - 1);
                    self.heightmaps.set(column_index, height);
                }
            }
        }

    }

    /// Public method that you can use to recompute a single column for all heightmaps.
    pub fn recompute_heightmap_column(&mut self, x: u8, z: u8) {

        perf::push("Chunk::recompute_heightmap_column");

        let min_block_y = self.get_height().get_min_block();
        let max_block_y = self.get_height().get_max_block();
        let column_index = calc_heightmap_index(x, z);

        perf::push("non_null_heightmap");
        let non_null_height = self.recompute_heightmap_column_internal(&HEIGHTMAP_NON_NULL, x, z, max_block_y);
        self.heightmaps.set(column_index, non_null_height);
        perf::pop();

        // Let's recompute each heightmap only from this y coordinate.
        let from_y = non_null_height as i32 + min_block_y - 1;

        for (idx, heightmap_type) in self.env.heightmaps.iter_heightmap_types().enumerate() {
            perf::push("heightmap_type");
            let column_index = 256 + idx * 256 + column_index;
            let height = self.recompute_heightmap_column_internal(heightmap_type, x, z, from_y);
            self.heightmaps.set(column_index, height);
            perf::pop();
        }

        perf::pop();

    }

    /// Internal method that recompute a single column for a specific type of heightmap.
    /// **You must ensure that given coordinates are valid for this chunk, the only value
    /// that is allowed out of bounds is if the `from_y` value is lower than minimum.**
    fn recompute_heightmap_column_internal(&self, heightmap_type: &'static HeightmapType, x: u8, z: u8, from_y: i32) -> u64 {

        perf::push("Chunk::recompute_heightmap_column_internal");

        let sub_chunk_index = (from_y >> 4) as i8 - self.sub_chunks_offset;
        if sub_chunk_index < 0 {
            perf::pop();
            return 0; // If the given 'from_y' is out of bounds.
        }

        let mut sub_chunk_index = sub_chunk_index as usize;
        let mut by = (from_y & 15) as u8 + 1;

        loop {

            let sub_chunk = loop {
                match self.sub_chunks[sub_chunk_index] {
                    Some(ref sub_chunk) if sub_chunk.has_non_null_block() => break sub_chunk,
                    _ => {
                        if sub_chunk_index == 0 {
                            perf::pop();
                            return 0; // If the lowest sub chunk is absent.
                        } else {
                            sub_chunk_index -= 1;
                        }
                    }
                }
            };

            while by > 0 {
                by -= 1;
                perf::push("check_block");
                let state = sub_chunk.get_block(x, by, z);
                if heightmap_type.check_block(state, &self.env.blocks) {
                    perf::pop();
                    perf::pop();
                    return (sub_chunk_index << 4) as u64 + by as u64 + 1;
                }
                perf::pop();
            }

            if sub_chunk_index == 0 {
                perf::pop();
                return 0; // We reached the lowest sub chunk.
            } else {
                sub_chunk_index -= 1;
                by = 16;
            }

        }

    }

    /// Direct access method to internal packed array, returning each of the 256 values from the
    /// given heightmap type if it exists, ordered by X then Z.
    pub fn iter_heightmap_raw_columns(&self, heightmap_type: &'static HeightmapType) -> Option<(u8, impl Iterator<Item = u64> + '_)> {
        let offset = 256 + self.env.heightmaps.get_heightmap_index(heightmap_type)? * 256;
        Some((self.heightmaps.byte_size(), self.heightmaps.iter().skip(offset).take(256)))
    }

    // ENTITIES //

    #[inline]
    pub unsafe fn add_entity_unchecked(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    #[inline]
    pub unsafe fn remove_entity_unchecked(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    #[inline]
    pub fn has_entity(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }

}


/// Internal sub chunks blocks palette.
enum SubChunkBlocks {
    /// Local blocks palette.
    Local {
        /// Blocks palette. It is limited to 128 blocks in order to support most blocks in a natural
        /// generation, which is the case of most of the sub chunks that are untouched by players.
        /// In case of "artificial chunks" made by players, the block palette is likely to overflow
        /// the 128 block states limit, in this case it switch to the global palette (`GlobalBlocks`
        /// in the level environment).
        /// The palette uses a raw pointer in order to use the pointer equality instead of value eq.
        palette: Palette<*const BlockState>,
        /// Save ID of the null block in the palette. Set to `u32::MAX` if the null block is not
        /// present in the local palette. This value is safe because the palette has a limited
        /// capacity of `BLOCKS_PALETTE_CAPACITY` block states.
        null_block_sid: u32
    },
    /// Global blocks palette.
    Global
}

impl SubChunkBlocks {

    /// Compute and return the effective null block save ID from the enum variant.
    fn get_null_block_sid(&self) -> Option<u32> {
        match self {
            Self::Local { null_block_sid: u32::MAX, .. } => None,
            Self::Local { null_block_sid, .. } => Some(*null_block_sid),
            Self::Global => Some(0)
        }
    }

}

// Implemented because palette of '*const BlockState' prevent it by default.
unsafe impl Send for SubChunkBlocks {}
unsafe impl Sync for SubChunkBlocks {}


pub enum Light {
    Block = 0,
    Sky = 1
}


/// A sub chunk, 16x16x16 blocks.
pub struct SubChunk {
    /// A local shared pointer to the level environment.
    env: Arc<LevelEnv>,
    /// Local of global blocks palette.
    blocks_palette: SubChunkBlocks,
    /// Cube blocks array.
    blocks: PackedArray,
    /// Both block and sky lights combined, 4 bits per block.
    lights: PackedArray,
    /// Non-null blocks count. "Null block" is a shorthand for the block state at save ID 0 in the
    /// global palette, likely to be an 'air' block state for example in vanilla. This number must
    /// be between 0 (inclusive) and 4096 (exclusive), other values are unsafe.
    non_null_blocks_count: u16,
}

// Palette capacity must ensure that most of the chunks will be supported AND that the palette
// lookup is fast, but because the lookup is O(N) we have to find a balanced capacity.
const BLOCKS_PALETTE_CAPACITY: usize = 128;

// Minimum size (in bits) of bytes representing block indices to local or global palette.
const BLOCKS_ARRAY_MIN_BYTE_SIZE: u8 = 4;

// Fixed size of bytes representing a single block light.
const LIGHTS_ARRAY_BYTE_SIZE: u8 = 4;


impl SubChunk {

    /// Construct a new sub chunk with the given level environment. The default block is
    /// the "null-block" which is the block with save ID 0 in the global blocks palette
    /// (which is air in vanilla global palette).
    pub fn new(env: Arc<LevelEnv>) -> Self {

        let null_block = env.blocks.get_state_from(0)
            .expect("An global blocks palette is not supported by SubChunk.");

        // The palettes are initialized with an initial state and biome (usually air and void, if
        // vanilla environment), this is required because the packed array has default values of 0,
        // and they must have a corresponding valid value in palettes, at least at the beginning.
        SubChunk {
            env,
            blocks_palette: SubChunkBlocks::Local {
                palette: Palette::with_default(null_block, BLOCKS_PALETTE_CAPACITY),
                null_block_sid: 0
            },
            blocks: PackedArray::new(BLOCKS_DATA_SIZE, BLOCKS_ARRAY_MIN_BYTE_SIZE, None),
            lights: PackedArray::new(BLOCKS_DATA_SIZE * 2, LIGHTS_ARRAY_BYTE_SIZE, None),
            non_null_blocks_count: 0,
        }

    }

    #[inline]
    fn get_blocks_byte_size(env: &LevelEnv) -> u8 {
        PackedArray::calc_min_byte_size(env.blocks.states_count() as u64 - 1)
            .max(BLOCKS_ARRAY_MIN_BYTE_SIZE)
    }

    // BLOCKS //

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> &'static BlockState {
        // SAFETY: The unwrap should be safe because the block index is expected to be right,
        //         moreover it is checked in debug mode.
        let sid = self.blocks.get(calc_block_index(x, y, z)).unwrap() as u32;
        self.get_block_from_sid(sid)
    }

    /// # Caution
    /// This method will not update heightmaps of the owner chunk.
    pub fn set_block(&mut self, x: u8, y: u8, z: u8, state: &'static BlockState) -> ChunkResult<()> {
        let idx = calc_block_index(x, y, z);
        match self.ensure_block_sid(state) {
            Some(sid) => {
                let old_sid = self.blocks.set(idx, sid as u64) as u32;
                if let Some(null_block_sid) = self.blocks_palette.get_null_block_sid() {
                    let was_null = old_sid == null_block_sid;
                    let is_null = sid == null_block_sid;
                    self.non_null_blocks_count = (self.non_null_blocks_count as i16 + was_null as i16 - is_null as i16) as u16;
                }
                Ok(())
            },
            None => Err(ChunkError::IllegalBlock)
        }
    }

    /// Internal method to get the block state reference from its save ID. It is internal because
    /// calling this method with invalid save ID for the sub chunk will panic.
    #[inline]
    fn get_block_from_sid(&self, sid: u32) -> &'static BlockState {
        // SAFETY: The unwraps should be safe in this case because the `set_block` ensure
        //         that the palette is updated according to the blocks contents. If the user
        //         directly manipulate the contents of palette or blocks array it MUST ensure
        //         that is correct, if not this will panic here.
        match self.blocks_palette {
            SubChunkBlocks::Local { ref palette, .. } => unsafe {
                // Here we transmute a `*const BlockState` to `&'static BlockState`, this is safe.
                std::mem::transmute(palette.get_item(sid as usize).unwrap())
            },
            SubChunkBlocks::Global => self.env.blocks.get_state_from(sid).unwrap()
        }
    }

    /// Internal method to get the save ID of a given block state within this chunk. If this
    /// block state is not currently registered, a new save ID will be allocated. If there
    /// are more than `BLOCKS_PALETTE_CAPACITY` blocks in the local palette, the whole blocks
    /// packed array will be resized in order to store global save ID instead of local ones.
    fn ensure_block_sid(&mut self, state: &'static BlockState) -> Option<u32> {

        if let SubChunkBlocks::Local {
            ref mut palette,
            ..
        } = self.blocks_palette {

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
        if let SubChunkBlocks::Local {
            ref palette,
            ..
        } = self.blocks_palette {
            let new_byte_size = Self::get_blocks_byte_size(&self.env);
            let global_palette = &self.env.blocks;
            self.blocks.resize_byte_and_replace(new_byte_size, move |_, sid| {
                let state = palette.get_item(sid as usize).unwrap();
                global_palette.get_sid_from(unsafe { std::mem::transmute(state) }).unwrap() as u64
            });
            self.blocks_palette = SubChunkBlocks::Global;
        }
    }

    /// Force fill all the sub chunk with the given block state.
    pub fn fill_block(&mut self, state: &'static BlockState) -> ChunkResult<()> {

        let null_block_sid = match self.env.blocks.get_sid_from(state) {
            None => return Err(ChunkError::IllegalBlock),
            Some(0) => {
                self.non_null_blocks_count = 0;
                0
            }
            Some(_) => {
                self.non_null_blocks_count = 4096;
                u32::MAX
            }
        };

        self.blocks_palette = SubChunkBlocks::Local {
            palette: Palette::with_default(state, BLOCKS_PALETTE_CAPACITY),
            null_block_sid
        };

        unsafe {
            // SAFETY: We don't care of the old content, and clearing all cells will set every
            // value to 0, which points to the new state at save ID 0 in the palette.
            self.blocks.resize_raw(BLOCKS_ARRAY_MIN_BYTE_SIZE);
            self.blocks.clear_cells();
        }

        Ok(())

    }

    // Lights //

    pub fn get_light(&self, x: u8, y: u8, z: u8, typ: Light) -> u8 {
        let light_idx = calc_block_index(x, y, z) + typ as usize * BLOCKS_DATA_SIZE;
        self.lights.get(light_idx).unwrap() as u8
    }

    pub fn set_light(&mut self, x: u8, y: u8, z: u8, typ: Light, level: u8) {
        assert!(level < 16, "Maximum light level is 15.");
        let light_idx = calc_block_index(x, y, z) + typ as usize * BLOCKS_DATA_SIZE;
        self.lights.set(light_idx, level as u64);
    }

    // Raw manipulations //

    /// # Safety:
    /// You must ensure that the given palette contains only valid states for this
    /// chunk's level's environment.
    ///
    /// The blocks iterator must return only valid indices for the given palette.
    pub unsafe fn set_blocks_raw<I>(&mut self, palette: Vec<&'static BlockState>, mut blocks: I)
    where
        I: Iterator<Item = usize>
    {

        assert_ne!(palette.len(), 0, "Palette length is zero.");

        if palette.len() <= BLOCKS_PALETTE_CAPACITY {

            let palette = cast_vec_ref_to_ptr(palette);
            let byte_size = PackedArray::calc_min_byte_size(palette.len() as u64 - 1)
                .max(BLOCKS_ARRAY_MIN_BYTE_SIZE);

            self.blocks_palette = SubChunkBlocks::Local {
                null_block_sid: {
                    let null_block = self.env.blocks.get_state_from(0).unwrap();
                    palette.iter()
                        .position(move |&v| v == null_block)
                        .map(|idx| idx as u32)
                        .unwrap_or(u32::MAX)
                },
                palette: Palette::from_raw(palette, BLOCKS_PALETTE_CAPACITY)
            };

            // We resize raw because we don't care of the old content, and the new byte size might
            // be smaller than the old one.
            self.blocks.resize_raw(byte_size);
            self.blocks.replace(move |_, _| {
                blocks.next().unwrap_or(0) as u64
            });

        } else {

            self.blocks_palette = SubChunkBlocks::Global;

            let byte_size = Self::get_blocks_byte_size(&self.env);
            let global_blocks = &self.env.blocks;

            self.blocks.resize_raw(byte_size);
            self.blocks.replace(move |_, _| {
                let palette_idx = blocks.next().unwrap_or(0);
                let state = palette[palette_idx];
                // SAFETY: We can unwrap because this is an safety condition of the method.
                global_blocks.get_sid_from(state).unwrap() as u64
            });

        }

        self.refresh_non_null_blocks_count();

    }

    /// Iterate over all blocks in this chunk, ordered by X, Z then Y.
    pub fn iter_blocks(&self) -> impl Iterator<Item = &'static BlockState> + '_ {
        SubChunkBlocksIter {
            inner: self.blocks.iter(),
            sub_chunk: self,
            last_block: None
        }
    }

    fn refresh_non_null_blocks_count(&mut self) {
        if let Some(null_block_sid) = self.blocks_palette.get_null_block_sid() {
            self.non_null_blocks_count = self.blocks.iter()
                .filter(move |&sid| sid != null_block_sid as u64)
                .count() as u16;
        } else {
            self.non_null_blocks_count = 4096; // The is no null block.
        }
    }

    // Meta info //

    #[inline]
    pub fn non_null_blocks_count(&self) -> u16 {
        self.non_null_blocks_count
    }

    #[inline]
    pub fn null_blocks_count(&self) -> u16 {
        4096 - self.non_null_blocks_count
    }

    #[inline]
    pub fn has_non_null_block(&self) -> bool {
        self.non_null_blocks_count != 0
    }

}


/// An efficient iterator over all blocks in a sub chunk.
pub struct SubChunkBlocksIter<'a, I> {
    inner: I,
    sub_chunk: &'a SubChunk,
    last_block: Option<(u64, &'static BlockState)>
}

impl<'a, I> Iterator for SubChunkBlocksIter<'a, I>
where
    I: Iterator<Item = u64>
{

    type Item = &'static BlockState;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some(sid) => {
                match self.last_block {
                    Some((last_sid, state)) if last_sid == sid => Some(state),
                    ref mut last_block => {
                        Some(&last_block.insert((sid, self.sub_chunk.get_block_from_sid(sid as u32))).1)
                    }
                }
            }
        }
    }

}


/// A little structure that stores the height of a level (and so a chunk).
#[derive(Debug, Clone, Copy)]
pub struct ChunkHeight {
    /// Inclusive lower bound.
    pub min: i8,
    /// Inclusive upper bound.
    pub max: i8,
}

impl ChunkHeight {

    pub fn new(min: i8, max: i8) -> Self {
        Self { min, max }
    }

    /// Returns the inclusive lower bound in blocks coordinates.
    #[inline]
    pub fn get_min_block(self) -> i32 {
        (self.min as i32) * 16
    }

    /// Returns the inclusive upper bound in blocks coordinates.
    #[inline]
    pub fn get_max_block(self) -> i32 {
        (self.max as i32) * 16 + 15
    }

    /// Return `true` if the given chunk Y coordinate is valid for the specified height.
    #[inline]
    pub fn contains(self, cy: i8) -> bool {
        return self.min <= cy && cy <= self.max;
    }

    /// Return the number of chunks in this chunk height.
    #[inline]
    pub fn len(self) -> usize {
        (self.max - self.min + 1) as usize
    }

    /// Iterate over all chunk Y coordinates in this chunk height.
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = i8> {
        self.min..=self.max
    }

}


#[inline]
fn calc_block_index(x: u8, y: u8, z: u8) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16, "x: {}, y: {}, z: {}", x, y, z);
    x as usize | ((z as usize) << 4) | ((y as usize) << 8)
}


#[inline]
fn calc_biome_index(x: u8, y: usize, z: u8) -> usize {
    debug_assert!(x < 4 && z < 4, "x: {}, z: {}", x, z);
    x as usize | ((z as usize) << 2) | (y << 4)
}


#[inline]
fn calc_heightmap_index(x: u8, z: u8) -> usize {
    debug_assert!(x < 16 && z < 16, "x: {}, z: {}", x, z);
    x as usize | ((z as usize) << 4)
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::block::GlobalBlocks;
    use crate::biome::GlobalBiomes;
    use crate::entity::GlobalEntities;
    use crate::heightmap::GlobalHeightmaps;

    crate::blocks!(TEST_BLOCKS "test" [
        AIR "air",
        STONE "stone",
        DIRT "dirt"
    ]);

    crate::biomes!(TEST_BIOMES "test" [
        VOID "void" 0,
    ]);

    fn heightmap_test(state: &'static BlockState, _blocks: &GlobalBlocks) -> bool {
        state == STONE.get_default_state()
    }

    crate::heightmaps!(TEST_HEIGHTMAPS [
        TEST "TEST" heightmap_test
    ]);

    fn build_chunk() -> Chunk {
        let env = Arc::new(LevelEnv::new(
            GlobalBlocks::with_all(&TEST_BLOCKS).unwrap(),
            GlobalBiomes::with_all(&TEST_BIOMES).unwrap(),
            GlobalEntities::new(),
            GlobalHeightmaps::with_all(&TEST_HEIGHTMAPS)
        ));
        Chunk::new(env, ChunkHeight::new(-1, 2), 0, 0)
    }

    #[test]
    fn valid_height() {
        let mut chunk = build_chunk();
        assert!(matches!(chunk.ensure_sub_chunk(-2), Err(ChunkError::SubChunkOutOfRange)));
        assert!(matches!(chunk.ensure_sub_chunk(-1), Ok(_)));
        assert!(matches!(chunk.ensure_sub_chunk(0), Ok(_)));
        assert!(matches!(chunk.ensure_sub_chunk(1), Ok(_)));
        assert!(matches!(chunk.ensure_sub_chunk(2), Ok(_)));
        assert!(matches!(chunk.ensure_sub_chunk(3), Err(ChunkError::SubChunkOutOfRange)));
    }

    #[test]
    fn valid_set_get() {
        let mut chunk = build_chunk();
        chunk.ensure_sub_chunk(-1).unwrap();
        assert_eq!(chunk.get_block(0, -16, 0).unwrap(), AIR.get_default_state());
        assert_eq!(chunk.get_block(0, 0, 0).unwrap(), AIR.get_default_state());
        assert!(matches!(chunk.get_block(0, -17, 0), Err(ChunkError::SubChunkOutOfRange)));
        assert!(matches!(chunk.set_block(0, 0, 0, STONE.get_default_state()), Ok(_)));
        assert_eq!(chunk.get_block(0, 0, 0).unwrap(), STONE.get_default_state());
    }

    #[test]
    fn valid_heightmap() {
        let mut chunk = build_chunk();
        assert!(matches!(chunk.get_heightmap_column(&TEST, 0, 0), Ok(-16)));
        chunk.set_block(0, -16, 0, STONE.get_default_state()).unwrap();
        assert!(matches!(chunk.get_heightmap_column(&TEST, 0, 0), Ok(-15)));
        chunk.set_block(0, 0, 0, STONE.get_default_state()).unwrap();
        chunk.set_block(0, -1, 0, STONE.get_default_state()).unwrap();
        assert!(matches!(chunk.get_heightmap_column(&TEST, 0, 0), Ok(1)));
        chunk.set_block(0, 0, 0, AIR.get_default_state()).unwrap();
        assert!(matches!(chunk.get_heightmap_column(&TEST, 0, 0), Ok(0)));
        chunk.set_block(0, -1, 0, AIR.get_default_state()).unwrap();
        assert!(matches!(chunk.get_heightmap_column(&TEST, 0, 0), Ok(-15)));
        chunk.set_block(0, -16, 0, AIR.get_default_state()).unwrap();
        assert!(matches!(chunk.get_heightmap_column(&TEST, 0, 0), Ok(-16)));
        chunk.set_block(0, 47, 0, STONE.get_default_state()).unwrap();
        assert!(matches!(chunk.get_heightmap_column(&TEST, 0, 0), Ok(48)));
    }

    #[test]
    fn valid_non_null_blocks_count() {

        let mut chunk = build_chunk();
        let sub_chunk = chunk.ensure_sub_chunk(0).unwrap();
        assert_eq!(sub_chunk.null_blocks_count(), 4096);
        assert_eq!(sub_chunk.non_null_blocks_count(), 0);
        sub_chunk.set_block(0, 0, 0, STONE.get_default_state()).unwrap();
        assert_eq!(sub_chunk.non_null_blocks_count(), 1);
        sub_chunk.set_block(0, 1, 0, STONE.get_default_state()).unwrap();
        assert_eq!(sub_chunk.non_null_blocks_count(), 2);
        sub_chunk.set_block(0, 1, 0, DIRT.get_default_state()).unwrap();
        assert_eq!(sub_chunk.non_null_blocks_count(), 2);
        sub_chunk.set_block(0, 0, 0, AIR.get_default_state()).unwrap();
        assert_eq!(sub_chunk.non_null_blocks_count(), 1);

        sub_chunk.fill_block(AIR.get_default_state()).unwrap();
        assert_eq!(sub_chunk.non_null_blocks_count(), 0);
        sub_chunk.fill_block(DIRT.get_default_state()).unwrap();
        assert_eq!(sub_chunk.null_blocks_count(), 0);

        unsafe {

            sub_chunk.set_blocks_raw(vec![STONE.get_default_state()], (0..4096).map(|_| 0));
            assert_eq!(sub_chunk.null_blocks_count(), 0);

            sub_chunk.set_blocks_raw(vec![AIR.get_default_state()], (0..4096).map(|_| 0));
            assert_eq!(sub_chunk.non_null_blocks_count(), 0);

        }

    }

}