use std::sync::{Arc, Weak, RwLock};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

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


#[inline]
fn calc_biome_2d_index(x: u8, z: u8) -> usize {
    debug_assert!(x < 16 && z < 16, "x: {}, z: {}", x, z);
    x as usize | ((z as usize) << 4)
}


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
    sub_chunks: HashMap<i8, SubChunk<'env>>,
    /// The legacy flat biomes array.
    biomes: [u16; BIOMES_2D_DATA_SIZE]
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
            sub_chunks: HashMap::new(),
            biomes: [0; BIOMES_2D_DATA_SIZE]
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
    /// is out of the height of the level, `None` is returned.
    pub fn ensure_sub_chunk(&mut self, cy: i8) -> Option<&mut SubChunk<'env>> {
        if self.height.includes(cy) {
            Some(match self.sub_chunks.entry(cy) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(SubChunk::new(self.env))
            })
        } else {
            None
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

    // RAW BLOCKS //

    /// Get the block id at a specific position relative to this chunk.
    ///
    /// Returns `Err(ChunkError::SubChunkUnloaded)` if no sub chunk is loaded at the given
    /// coordinates.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn get_block_id(&self, x: u8, y: i32, z: u8) -> ChunkResult<u16> {
        match self.get_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::SubChunkUnloaded),
            Some(sub_chunk) => Ok(sub_chunk.get_block_id(x, (y & 15) as u8, z))
        }
    }

    /// Set the block id at a specific position relative to this chunk.
    ///
    /// Return `Ok(())` if the biome was successfully set, `Err(ChunkError::IllegalVerticalPos)` if
    /// the given Y coordinate is invalid for the level.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    ///
    /// # Safety
    /// This method is unsafe because you should ensure that the block ID is valid for the world's
    /// block register.
    pub unsafe fn set_block_id(&mut self, x: u8, y: i32, z: u8, block_id: u16) -> ChunkResult<()> {
        match self.ensure_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::IllegalVerticalPos),
            Some(sub_chunk) => {
                sub_chunk.set_block_id(x, (y & 15) as u8, z, block_id);
                Ok(())
            }
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
        // SAFETY: Here we unwrap because the save ID should be valid since the level
        //         environment is not mutable and a reference is kept in this Chunk.
        //         The save ID can only be set from `set_block` or `set_block_id`, the first
        //         get the save ID from the blocks registers, and the last is unsafe.
        self.get_block_id(x, y, z).map(|sid| self.env.get_blocks().get_state_from(sid).unwrap())
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
        match self.ensure_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::IllegalVerticalPos),
            Some(sub_chunk) => {
                sub_chunk.set_block(x, (y & 15) as u8, z, state)
            }
        }
    }

    // RAW BIOMES //

    /// Get the 2D biome id at specific position.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn get_biome_2d_id(&self, x: u8, z: u8) -> u16 {
        self.biomes[calc_biome_2d_index(x, z)]
    }

    /// Get the 3D biome id at specific position.
    ///
    /// Returns `None` if no sub chunk is loaded at the given coordinates.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn get_biome_3d_id(&self, x: u8, y: i32, z: u8) -> ChunkResult<u16> {
        match self.get_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::SubChunkUnloaded),
            Some(sub_chunk) => Ok(sub_chunk.get_biome_id(x, (y & 15) as u8, z))
        }
    }

    /// Set the 2D biome id at specific position.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    ///
    /// # Safety
    /// This method is unsafe because you should ensure that the biome ID is valid for the world's
    /// biome register.
    pub unsafe fn set_biome_2d_id(&mut self, x: u8, z: u8, biome_id: u16) {
        self.biomes[calc_biome_2d_index(x, z)] = biome_id;
    }

    /// Set the 3D biome id at specific position.
    ///
    /// Return `Ok(())` if the biome was successfully set, `Err(ChunkError::IllegalVerticalPos)` if
    /// the given Y coordinate is invalid for the level.
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    ///
    /// # Safety
    /// This method is unsafe because you should ensure that the biome ID is valid for the world's
    /// biome register.
    pub unsafe fn set_biome_3d_id(&mut self, x: u8, y: i32, z: u8, biome_id: u16) -> ChunkResult<()> {
        match self.ensure_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::IllegalVerticalPos),
            Some(sub_chunk) => {
                sub_chunk.set_biome_id(x, (y & 15) as u8, z, biome_id);
                Ok(())
            }
        }
    }

    // BIOMES //

    pub fn get_biome_2d(&self, x: u8, z: u8) -> &'static Biome {
        // SAFETY: Check safety comment of `get_block` for explanation of the unwrapping.
        self.env.get_biomes().get_biome_from(self.get_biome_2d_id(x, z)).unwrap()
    }

    pub fn get_biome_3d(&self, x: u8, y: i32, z: u8) -> ChunkResult<&'static Biome> {
        // SAFETY: Check safety comment of `get_block` for explanation of the unwrapping.
        self.get_biome_3d_id(x, y, z).map(|sid| self.env.get_biomes().get_biome_from(sid).unwrap())
    }

    pub fn set_biome_2d(&mut self, x: u8, z: u8, biome: &'static Biome) -> ChunkResult<()> {
        match self.env.get_biomes().get_sid_from(biome) {
            None => Err(ChunkError::IllegalBiome),
            Some(sid) => unsafe {
                self.set_biome_2d_id(x, z, sid);
                Ok(())
            }
        }
    }

    pub fn set_biome_3d(&mut self, x: u8, y: i32, z: u8, biome: &'static Biome) -> ChunkResult<()> {
        match self.ensure_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::IllegalVerticalPos),
            Some(sub_chunk) => {
                sub_chunk.set_biome(x, (y & 15) as u8, z, biome)
            }
        }
    }

    // GUARDS //

    /*fn update_world(&self) -> Arc<RwLock<World>> {
        self.world.upgrade().unwrap() // Should not panic
    }

    /// Get a read-only guard for rich access to this chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn read(&self) -> ChunkReadGuard {
        ChunkReadGuard::new(self, self.update_world())
    }

    /// Get a read/write guard for rich access to this chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn write(&mut self) -> ChunkWriteGuard {
        ChunkWriteGuard::new(self, self.update_world())
    }*/

    // BIOMES CONVERSIONS //

    /// Expand the 2D legacy biome grid to the 3D biome grid of 3D of all sub-chunks.
    pub fn set_biome_3d_auto(&mut self) {
        unsafe {
            for x in 0..4 {
                for z in 0..4 {
                    // Maybe we can choose the most represented biome in the 4x4 cube
                    let biome = self.get_biome_2d_id((x << 2) + 1, (z << 2) + 1);
                    for cy in self.height {
                        let sub_chunk = self.ensure_sub_chunk(cy).unwrap();
                        for y in 0..4 {
                            sub_chunk.set_biome_id(x << 2, y << 2, z << 2, biome);
                        }
                    }
                }
            }
        }
    }

}


/// A sub chunk, 16x16x16 blocks.
pub struct SubChunk<'env> {
    env: &'env LevelEnv,
    /// Cube blocks array.
    blocks: [u16; BLOCKS_DATA_SIZE],
    /// Modern cube biomes array.
    biomes: [u16; BIOMES_3D_DATA_SIZE]
}

impl<'env> SubChunk<'env> {

    fn new(env: &'env LevelEnv) -> Self {
        SubChunk {
            env,
            blocks: [0; BLOCKS_DATA_SIZE],
            biomes: [0; BIOMES_3D_DATA_SIZE]
        }
    }

    // RAW BLOCKS //

    /// Get the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`.
    ///
    /// **This function is intended for internal uses, but is still public to allow
    /// low level manipulation, be careful!**
    pub fn get_block_id(&self, x: u8, y: u8, z: u8) -> u16 {
        self.blocks[calc_block_index(x, y, z)]
    }

    /// Set the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`. The block id must valid for the registry of the world
    /// this chunk belongs to.
    ///
    /// # Safety
    /// This method is unsafe because you should ensure that the block ID is valid for the world's
    /// block register.
    pub unsafe fn set_block_id(&mut self, x: u8, y: u8, z: u8, block_id: u16) {
        self.blocks[calc_block_index(x, y, z)] = block_id;
    }

    // BLOCKS //

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> &'static BlockState {
        // SAFETY: Check safety comment of `Chunk::get_block` for explanation of the unwrapping.
        self.env.get_blocks().get_state_from(self.get_block_id(x, y, z)).unwrap()
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, state: &'static BlockState) -> ChunkResult<()> {
        match self.env.get_blocks().get_sid_from(state) {
            None => Err(ChunkError::IllegalBlock),
            Some(sid) => unsafe {
                self.set_block_id(x, y, z, sid);
                Ok(())
            }
        }
    }

    // RAW BIOMES //

    /// Returns the internal biome ID stored at specific position. This value will always be
    /// valid to set back to any sub-chunk owned by the same world as this sub-chunk.
    ///
    /// This method may override the underlying biome array because in the current implementation
    /// a 3D biome sample take 4x4x4 blocs.
    pub fn get_biome_id(&self, x: u8, y: u8, z: u8) -> u16 {
        // Note: Shifting position because biomes 3D cube is 4x4x4 and
        // we don't want to expose this details to the API.
        self.biomes[calc_biome_3d_index(x >> 2, y >> 2, z >> 2)]
    }

    /// Set the internal biome ID stored at specific position.
    ///
    /// # Safety
    /// This method is unsafe because you should ensure that the biome ID is valid for the world's
    /// biome register.
    pub unsafe fn set_biome_id(&mut self, x: u8, y: u8, z: u8, biome_id: u16) {
        self.biomes[calc_biome_3d_index(x >> 2, y >> 2, z >> 2)] = biome_id;
    }

    // BIOMES //

    pub fn get_biome(&self, x: u8, y: u8, z: u8) -> &'static Biome {
        // SAFETY: Check safety comment of `Chunk::get_block` for explanation of the unwrapping.
        self.env.get_biomes().get_biome_from(self.get_biome_id(x, y, z)).unwrap()
    }

    pub fn set_biome(&mut self, x: u8, y: u8, z: u8, biome: &'static Biome) -> ChunkResult<()> {
        match self.env.get_biomes().get_sid_from(biome) {
            None => Err(ChunkError::IllegalBiome),
            Some(sid) => unsafe {
                self.set_biome_id(x, y, z, sid);
                Ok(())
            }
        }
    }

    // GUARDS //

    /*fn update_world(&self) -> Arc<RwLock<World>> {
        self.world.upgrade().unwrap() // Should never panic
    }

    /// Get a read-only guard for rich access to this sub chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn read(&self) -> SubChunkReadGuard {
        SubChunkReadGuard::new(self, self.update_world())
    }

    /// Get a read/write guard for rich access to this sub chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn write(&mut self) -> SubChunkWriteGuard {
        SubChunkWriteGuard::new(self, self.update_world())
    }*/

}


/*/// Internal base structure for rich access guards for `Chunk`s and `SubChunk`s.
pub struct RichGuard<O> {
    owner: O,
    /// This Arc is kept in like a guard for the owned world. Check `world` for explanation.
    _arc: Arc<RwLock<World>>,
    /// ***TW Hacky Tricks***<br>
    /// This guard is used to keep a read access to the world.
    /// The guard is transmuted in order to get a static lifetime, the real lifetime is ignored
    /// because this guard is bound to the object owned by `_arc` (in heap) which will live as long
    /// as this structure. **The guard must not be leaked out of this structure!** Instead, it must
    /// be kept into this struct in order to be dropped together with `_arc`.
    world: RwLockReadGuard<'static, World>
}

impl<O> RichGuard<O> {

    fn new(owner: O, arc: Arc<RwLock<World>>) -> Self {
        Self {
            owner,
            world: unsafe { std::mem::transmute(arc.read().unwrap()) },
            _arc: arc,
        }
    }

}


/// Type alias for rich read-only guard on `Chunk`s.
pub type ChunkReadGuard<'a> = RichGuard<&'a Chunk>;
/// Type alias for rich read/write guard on `Chunk`s.
pub type ChunkWriteGuard<'a> = RichGuard<&'a mut Chunk>;
/// Type alias for rich read-only guard on `SubChunk`s.
pub type SubChunkReadGuard<'a> = RichGuard<&'a SubChunk>;
/// Type alias for rich read/write guard on `SubChunk`s.
pub type SubChunkWriteGuard<'a> = RichGuard<&'a mut SubChunk>;

macro_rules! impl_chunk_guard {
    ($t:ident) => {

        impl $t<'_> {

            pub fn get_block(&self, x: u8, y: i32, z: u8) -> Option<&'static BlockState> {
                self.world.get_blocks().get_state_from(self.owner.get_sub_chunk(y >> 4).get_block_id(x, y & 15, z))
            }

            pub fn get_biome_2d(&self, x: u8, z: u8) -> &'static Biome {
                self.world.get_biomes().get_biome_from(self.owner.get_biome_2d_id(x, z))
                    .expect("Chunk can't have undefined biome.")
            }

            pub fn get_biome_3d(&self, x: u8, y: i32, z: u8) -> &'static Biome {
                self.world.get_biomes().get_biome_from(self.owner.get_biome_3d_id(x, y, z))
                    .expect("Chunk can't have undefined biome.")
            }

        }

    }
}

macro_rules! impl_sub_chunk_guard {
    ($t:ident) => {

        impl $t<'_> {

            /// Get the actual block state at specific position.
            pub fn get_block(&self, x: u8, y: u8, z: u8) -> Option<&'static BlockState> {
                self.world.get_blocks().get_state_from(self.owner.get_block_id(x, y, z))
            }

            /// Returns the actual biome reference.
            pub fn get_biome(&self, x: u8, y: u8, z: u8) -> &'static Biome {
                self.world.get_biomes().get_biome_from(self.owner.get_biome_id(x, y, z)).unwrap()
            }

        }

    }
}

impl_chunk_guard!(ChunkReadGuard);
impl_chunk_guard!(ChunkWriteGuard);
impl_sub_chunk_guard!(SubChunkReadGuard);
impl_sub_chunk_guard!(SubChunkWriteGuard);


impl ChunkWriteGuard<'_> {

    /// Set the block at a specific position relative to this chunk.
    ///
    /// Return `Ok(())` if the biome was successfully set, `Err(ChunkError::InvalidLevel(y))` if the
    /// given Y coordinate is invalid for the level or `Err(ChunkError::UnknownBlock(state))` if the
    /// given block state is not registered in the current world..
    ///
    /// # Panics (debug-only)
    /// This method panics if either X or Z is higher than 15.
    pub fn set_block(&mut self, x: u8, y: i32, z: u8, state: &'static BlockState) -> Result<(), ChunkError> {
        match self.owner.ensure_sub_chunk((y >> 4) as i8) {
            None => Err(ChunkError::InvalidLevel(y)),
            Some(chunk) => {
                match self.world.get_blocks().get_sid_from(state) {
                    Some(sid) => unsafe {
                        chunk.set_block_id(x, (y & 15) as u8, z, sid);
                        Ok(())
                    },
                    None => Err(ChunkError::UnknownBlock(state))
                }

            }
        }
    }

    pub fn set_biome_2d(&mut self, x: u8, z: u8, biome: &'static Biome) -> Result<(), ChunkError> {
        /*match self.world.get_biomes().get_sid_from(biome) {
            Some(sid) => unsafe {
                self.owner.set_block_id()
            }
        }*/
        unsafe {
            self.owner.set_biome_2d_id(x, z, self.world.get_biomes().get_sid_from(biome)
                .expect("This biome is not supported by the world."));
        }
    }

    pub fn set_biome_3d(&mut self, x: u8, y: i32, z: u8, biome: &'static Biome) -> bool {
        unsafe {
            self.owner.set_biome_3d_id(x, y, z, self.world.get_biomes().get_sid_from(biome)
                .expect("This biome is not supported by the world."))
        }
    }

}


impl SubChunkWriteGuard<'_> {

    /// Set the actual block state at specific position, the position is relative to
    /// this chunk `(0 <= x/y/z < 16)`. The block can be `None` to remove the block,
    /// if `Some`, the state must be supported by the world this chunk belongs to.
    ///
    /// # Panics
    /// Panics if the given state is not supported by the world.
    pub fn set_block(&mut self, x: u8, y: u8, z: u8, state: &'static BlockState) {
        unsafe {
            self.owner.set_block_id(x, y, z, self.world.get_blocks().get_sid_from(state)
                .expect("This block state is not supported by the world."));
        }
    }

    /// Set the actual biome at specific position, check [`SubChunk::get_biome_id`][get_biome_id]
    /// to understand specificities of biome position.
    ///
    /// [get_biome_id]: SubChunk::get_biome_id
    pub fn set_biome(&mut self, x: u8, y: u8, z: u8, biome: &'static Biome) {
        unsafe {
            self.owner.set_biome_id(x, y, z, self.world.get_biomes().get_sid_from(biome)
                .expect("This biome is not supported by the world."));
        }
    }

}
*/