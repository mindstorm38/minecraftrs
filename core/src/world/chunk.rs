use std::sync::{Arc, Weak, RwLock, RwLockReadGuard};

use crate::block::BlockState;
use crate::biome::Biome;

use super::level::Level;
use super::World;


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const BLOCKS_DATA_SIZE: usize = SIZE * SIZE * SIZE;
/// The total count of biomes samples for 3 dimensional biomes.
pub const BIOMES_2D_DATA_SIZE: usize = SIZE * SIZE;
/// The total count of biomes samples for 3 dimensional biomes.
pub const BIOMES_3D_DATA_SIZE: usize = 4 * 4 * 4;


/// Used to calculate the index in a data array of `DATA_SIZE`.
#[inline]
fn calc_block_index(x: usize, y: usize, z: usize) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16, "x: {}, y: {}, z: {}", x, y, z);
    (x & 15) | ((z & 15) << 4) | ((y & 15) << 8)
}


#[inline]
fn calc_biome_2d_index(x: usize, z: usize) -> usize {
    debug_assert!(x < 16 && z < 16, "x: {}, z: {}", x, z);
    (x & 15) | ((z & 15) << 4)
}


#[inline]
fn calc_biome_3d_index(x: usize, y: usize, z: usize) -> usize {
    debug_assert!(x < 4 && y < 4 && z < 4, "x: {}, y: {}, z: {}", x, y, z);
    (x & 3) | ((z & 3) << 2) | ((y & 3) << 4)
}


macro_rules! new_guard {
    ($self_id:ident, $struct_id:ident) => {
        {
            let arc = $self_id.world.upgrade().unwrap();
            $struct_id {
                owner: $self_id,
                world: unsafe { std::mem::transmute(arc.read().unwrap()) },
                _arc: arc,
            }
        }
    };
}


/// A vertical chunk, 16x16 blocks.
pub struct Chunk {
    level: Weak<RwLock<Level>>,
    world: Weak<RwLock<World>>,
    cx: i32,
    cz: i32,
    populated: bool,
    sub_chunks: Vec<SubChunk>,
    /// The legacy flat biomes array.
    biomes: [u16; BIOMES_2D_DATA_SIZE]
}

impl Chunk {

    pub fn new(level: Weak<RwLock<Level>>, cx: i32, cz: i32, sub_chunks_count: u8) -> Self {

        let world = level.upgrade().unwrap().read().unwrap().get_weak_world();

        Chunk {
            level,
            world: Weak::clone(&world),
            cx,
            cz,
            populated: false,
            sub_chunks: (0..sub_chunks_count).map(|_| SubChunk::new(Weak::clone(&world))).collect(),
            biomes: [0; BIOMES_2D_DATA_SIZE]
        }

    }

    /// Return a strong counted reference to the `Level` owning this chunk.
    ///
    /// # Panics
    ///
    /// This method panic if this chunk is no longer owned (should not happen).
    pub fn get_level(&self) -> Arc<RwLock<Level>> {
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

    /// Get a sub chunk reference at a specified index.
    pub fn get_sub_chunk(&self, cy: usize) -> &SubChunk {
        &self.sub_chunks[cy]
    }

    /// Get a sub chunk mutable reference at a specified index.
    pub fn get_sub_chunk_mut(&mut self, cy: usize) -> &mut SubChunk {
        &mut self.sub_chunks[cy]
    }

    /// Return the number of sub chunks in the height of this chunk.
    pub fn get_sub_chunks_count(&self) -> usize {
        self.sub_chunks.len()
    }

    /// Get the max number of blocks in the height of this chunk.
    pub fn get_max_height(&self) -> usize {
        self.get_sub_chunks_count() << 4
    }

    pub fn get_sub_chunks(&self) -> &Vec<SubChunk> {
        &self.sub_chunks
    }

    // RAW BLOCKS //

    /// Get the block id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn get_block_id(&self, x: usize, y: usize, z: usize) -> u16 {
        self.get_sub_chunk(y >> 4).get_block_id(x, y & 15, z)
    }

    /// Set the block id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub unsafe fn set_block_id(&mut self, x: usize, y: usize, z: usize, block_id: u16) {
        self.get_sub_chunk_mut(y >> 4).set_block_id(x, y & 15, z, block_id);
    }

    // RAW BIOMES //

    pub fn get_biome_2d_id(&self, x: usize, z: usize) -> u16 {
        self.biomes[calc_biome_2d_index(x, z)]
    }

    pub fn get_biome_3d_id(&self, x: usize, y: usize, z: usize) -> u16 {
        self.get_sub_chunk(y >> 4).get_biome_id(x, y & 15, z)
    }

    pub unsafe fn set_biome_2d_id(&mut self, x: usize, z: usize, id: u16) {
        self.biomes[calc_biome_2d_index(x, z)] = id;
    }

    pub unsafe fn set_biome_3d_id(&mut self, x: usize, y: usize, z: usize, id: u16) {
        self.get_sub_chunk_mut(y >> 4).set_biome_id(x, y & 15, z, id);
    }

    // GUARDS //

    /// Get a read-only guard for rich access to this chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn read(&self) -> ChunkReadGuard {
        new_guard!(self, ChunkReadGuard)
    }

    /// Get a read/write guard for rich access to this chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn write(&mut self) -> ChunkWriteGuard {
        new_guard!(self, ChunkWriteGuard)
    }

    // BIOMES CONVERSIONS //

    /// Expand the 2D legacy biome grid to the 3D biome grid of 3D of all sub-chunks.
    pub fn set_biome_3d_auto(&mut self) {
        unsafe {
            for x in 0..4 {
                for z in 0..4 {
                    // Maybe we can choose the most represented biome in the 4x4 cube
                    let biome = self.get_biome_2d_id((x << 2) + 1, (z << 2) + 1);
                    for sub_chunk in &mut self.sub_chunks {
                        for y in 0..4 {
                            sub_chunk.set_biome_id(x << 2, y << 2, z << 2, biome);
                        }
                    }
                }
            }
        }
    }

}

/// Used for rich read-only access to a `SubChunk`.
///
/// Documentation of this structure is the same
pub struct ChunkReadGuard<'a> {
    owner: &'a Chunk,
    /// This Arc is kept in like a guard for the owned world. Check `world` for explanation.
    _arc: Arc<RwLock<World>>,
    /// This guard is used to keep a read access to the world.
    /// The guard is transmuted in order to get a static lifetime, the real lifetime is used
    /// because this guard is bound to the object owned by `_arc` (in heap). The guard must
    /// not be leaked out of this structure! Instead, it must be kept into this struct in order
    /// to be dropped together with `_arc`.
    world: RwLockReadGuard<'static, World>
}

/// Used for rich read/write access to a `SubChunk`.
pub struct ChunkWriteGuard<'a> {
    owner: &'a mut Chunk,
    _arc: Arc<RwLock<World>>,
    world: RwLockReadGuard<'static, World>
}

macro_rules! impl_sub_chunk_guard_read {
    () => {

        pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&'static BlockState> {
            self.world.get_blocks().get_state_from(self.owner.get_sub_chunk(y >> 4).get_block_id(x, y & 15, z))
        }

        pub fn get_biome_2d(&self, x: usize, z: usize) -> &'static Biome {
            self.world.get_biomes().get_biome_from(self.owner.get_biome_2d_id(x, z))
                .expect("Chunk can't have undefined biome.")
        }

        pub fn get_biome_3d(&self, x: usize, y: usize, z: usize) -> &'static Biome {
            self.world.get_biomes().get_biome_from(self.owner.get_biome_3d_id(x, y, z))
                .expect("Chunk can't have undefined biome.")
        }

    }
}

impl<'a> ChunkReadGuard<'a> {
    impl_sub_chunk_guard_read!();
}

impl<'a> ChunkWriteGuard<'a> {

    impl_sub_chunk_guard_read!();

    /// Set the block at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, state: Option<&'static BlockState>) {
        unsafe {
            self.owner.get_sub_chunk_mut(y >> 4).set_block_id(x, y & 15, z, match state {
                Some(state) => self.world.get_blocks().get_sid_from(state)
                    .expect("This block state is not supported by the world."),
                None => 0
            })
        }
    }

    pub fn set_biome_2d(&mut self, x: usize, z: usize, biome: &'static Biome) {
        unsafe {
            self.owner.set_biome_2d_id(x, z, self.world.get_biomes().get_sid_from(biome)
                .expect("This biome is not supported by the world."));
        }
    }

    pub fn set_biome_3d(&mut self, x: usize, y: usize, z: usize, biome: &'static Biome) {
        unsafe {
            self.owner.set_biome_3d_id(x, y, z, self.world.get_biomes().get_sid_from(biome)
                .expect("This biome is not supported by the world."));
        }
    }

}


/// A sub chunk, 16x16x16 blocks.
pub struct SubChunk {
    /// Weak counted reference to the `World` owning this sub chunk,
    /// it's used to return valid blocks references.
    world: Weak<RwLock<World>>,
    /// Cube blocks array.
    blocks: [u16; BLOCKS_DATA_SIZE],
    /// Modern cube biomes array.
    biomes: [u16; BIOMES_3D_DATA_SIZE]
}

impl SubChunk {

    fn new(world: Weak<RwLock<World>>) -> Self {
        SubChunk {
            world,
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
    pub fn get_block_id(&self, x: usize, y: usize, z: usize) -> u16 {
        self.blocks[calc_block_index(x, y, z)]
    }

    /// Set the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`. The block id must valid for the registry of the world
    /// this chunk belongs to.
    ///
    /// **This function is intended for internal uses, but is still public to allow
    /// low level manipulation, be careful!**
    pub unsafe fn set_block_id(&mut self, x: usize, y: usize, z: usize, block_id: u16) {
        self.blocks[calc_block_index(x, y, z)] = block_id;
    }

    // RAW BIOMES //

    /// Returns the internal biome ID stored at specific position. This value will always be
    /// valid to set back to any sub-chunk owned by the same world as this sub-chunk.
    ///
    /// This method may override the underlying biome array because in the current implementation
    /// a 3D biome sample take 4x4x4 blocs.
    pub fn get_biome_id(&self, x: usize, y: usize, z: usize) -> u16 {
        // Note: Shifting position because biomes 3D cube is 4x4x4 and
        // we don't want to expose this details to the API.
        self.biomes[calc_biome_3d_index(x >> 2, y >> 2, z >> 2)]
    }

    /// Set the internal biome ID stored at specific position.
    ///
    /// # Safety
    ///
    /// You must ensure that this ID is a valid biome SID for the world
    /// owning this sub-chunk. If the SID is invalid, subsequent calls to `get_biome`
    /// will panic.
    pub unsafe fn set_biome_id(&mut self, x: usize, y: usize, z: usize, id: u16) {
        self.biomes[calc_biome_3d_index(x >> 2, y >> 2, z >> 2)] = id;
    }

    // GUARDS //

    /// Get a read-only guard for rich access to this sub chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn read(&self) -> SubChunkReadGuard {
        new_guard!(self, SubChunkReadGuard)
    }

    /// Get a read/write guard for rich access to this sub chunk.
    /// This means that you can use the safe methods to directly get
    /// blocks and biomes from their static registers.
    pub fn write(&mut self) -> SubChunkWriteGuard {
        new_guard!(self, SubChunkWriteGuard)
    }

}


/// Used for rich read-only access to a `SubChunk`.
pub struct SubChunkReadGuard<'a> {
    owner: &'a SubChunk,
    _arc: Arc<RwLock<World>>,
    world: RwLockReadGuard<'static, World>
}

/// Used for rich read/write access to a `SubChunk`.
pub struct SubChunkWriteGuard<'a> {
    owner: &'a mut SubChunk,
    _arc: Arc<RwLock<World>>,
    world: RwLockReadGuard<'static, World>
}

macro_rules! impl_sub_chunk_guard_read {
    () => {

        /// Get the actual block state at specific position.
        pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&'static BlockState> {
            self.world.get_blocks().get_state_from(self.owner.get_block_id(x, y, z))
        }

        /// Returns the actual biome reference.
        pub fn get_biome(&self, x: usize, y: usize, z: usize) -> &'static Biome {
            self.world.get_biomes().get_biome_from(self.owner.get_biome_id(x, y, z)).unwrap()
        }

    };
}

impl<'a> SubChunkReadGuard<'a> {
    impl_sub_chunk_guard_read!();
}

impl<'a> SubChunkWriteGuard<'a> {

    impl_sub_chunk_guard_read!();

    /// Set the actual block state at specific position, the position is relative to
    /// this chunk `(0 <= x/y/z < 16)`. The block can be `None` to remove the block,
    /// if `Some`, the state must be supported by the world this chunk belongs to.
    ///
    /// # Panics
    ///
    /// Panics if the given `Some(state)` is not supported by the world.
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, state: Option<&'static BlockState>) {
        unsafe {
            self.owner.set_block_id(x, y, z, match state {
                Some(state) => self.world.get_blocks().get_sid_from(state)
                    .expect("This block state is not supported by the world."),
                None => 0
            });
        }
    }

    /// Set the actual biome at specific position, check [`SubChunk::get_biome_id`][get_biome_id]
    /// to understand specificities of biome position.
    ///
    /// [get_biome_id]: SubChunk::get_biome_id
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: &'static Biome) {
        unsafe {
            self.owner.set_biome_id(x, y, z, self.world.get_biomes().get_sid_from(biome)
                .expect("This biome is not supported by the world."));
        }
    }

}