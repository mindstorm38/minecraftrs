use std::sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use hecs::{
    World as EcsWorld,
    EntityBuilder, Entity,
    Ref as EntityRef, RefMut as EntityRefMut
};

use uuid::Uuid;

use crate::block::{GlobalBlocks, BlockState};
use crate::biome::GlobalBiomes;
use crate::entity::EntityType;
use crate::util::OpaquePtr;
use crate::pos::EntityPos;
use crate::debug;

use super::source::{LevelSource, LevelSourceBuilder, ChunkBuilder, LevelSourceError};
use super::chunk::{Chunk, ChunkHeight, ChunkResult, ChunkError};


/// A structure that contains the static environment of a World, this can be used for multiple
/// `Level`s through an `Arc<LevelEnv>`.
pub struct LevelEnv {
    /// Actual blocks register.
    pub blocks: GlobalBlocks,
    /// Actual biomes register.
    pub biomes: GlobalBiomes
}

impl LevelEnv {

    pub fn new(
        blocks: GlobalBlocks,
        biomes: GlobalBiomes
    ) -> Self {
        LevelEnv {
            blocks,
            biomes
        }
    }

}


/// Main storage for a level, part of a World. This structure is intentionally not `Sync + Send`,
/// however each chunk is stored in a `RwLock` in order to make them shared across threads if
/// you want.
pub struct Level {
    /// The unique ID of this level (among all levels of the world).
    id: String,
    /// The global environment used by this level, this environment should not be mutated afterward.
    /// It contains the global blocks and biomes palettes, it also contains
    env: Arc<LevelEnv>,
    /// The level loader used to load uncached chunks either from a generator or from an anvil file
    /// system loader.
    /// TODO: Check if this can be moved to mc-runtime? Worth it?
    source: Box<dyn LevelSource>,
    /// The configured height of this level.
    height: ChunkHeight,
    /// Chunk storage.
    chunks: ChunkStorage,
    /// Entities storage.
    entities: EntityStorage
}

impl Level {

    pub fn new<S, B>(id: String, env: Arc<LevelEnv>, source: B) -> Self
    where
        S: LevelSource + 'static,
        B: LevelSourceBuilder<S>
    {

        assert_ne!(env.blocks.states_count(), 0, "The given environment has no state, a level requires at least one block state.");
        assert_ne!(env.biomes.biomes_count(), 0, "The given environment has no biome, a level requires at least one biome.");

        let height = source.get_height();
        let builder = ChunkBuilder {
            env: Arc::clone(&env),
            height
        };

        Level {
            id,
            env,
            height,
            source: Box::new(source.build(builder)),
            chunks: ChunkStorage {
                chunks: HashMap::new()
            },
            entities: EntityStorage {
                ecs: EcsWorld::new(),
                builder_cache: EntityBuilderCache::new()
            }
        }

    }

    /// Return the unique ID (unique in the owning world).
    pub fn get_id(&self) -> &String {
        &self.id
    }

    /// Return the level environment used by this level.
    pub fn get_env(&self) -> Arc<LevelEnv> {
        Arc::clone(&self.env)
    }

    /// Return the minimum and maximum chunks position allowed in this world.
    /// The limits can -128 to 127, it is more than enough.
    pub fn get_height(&self) -> ChunkHeight {
        self.height
    }

    // CHUNKS LOADING (FROM SOURCE //

    /// Request internal level source to load the given chunk.
    pub fn request_chunk(&mut self, cx: i32, cz: i32) -> bool {
        debug!("Request chunk load at {}/{}", cx, cz);
        matches!(self.source.request_chunk_load(cx, cz), Ok(_))
    }

    /// Poll loaded chunks from internal level source, all successfully loaded chunks
    /// are added to the underlying `LevelStorage`. The callback is called for each
    /// loaded chunks or loading error.
    pub fn load_chunks_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut((i32, i32, Result<&Arc<RwLock<Chunk>>, LevelSourceError>)),
    {
        while let Some(((cx, cz), res)) = self.source.poll_chunk() {
            match res {
                Ok(mut chunk) => {
                    debug!("Loaded chunk at {}/{}", cx, cz);
                    callback((cx, cz, Ok(self.chunks.insert_chunk(chunk))));
                },
                Err(err) => {
                    // IDE shows an error for 'Display' not being implemented, but we use the
                    // crate 'thiserror' to implement it through a custom derive.
                    debug!("Failed to load chunk at {}/{}: {}", cx, cz, err);
                    callback((cx, cz, Err(err)));
                }
            }
        }
    }

    /// Poll loaded chunks from internal level source, all successfully loaded chunks
    /// are added to the underlying `LevelStorage`.
    #[inline]
    pub fn load_chunks(&mut self) {
        self.load_chunks_with_callback(|_| {});
    }

    // ENTITIES //

    #[inline]
    pub fn get_entities(&self) -> &EntityStorage {
        &self.entities
    }

    #[inline]
    pub fn get_entities_mut(&mut self) -> &mut EntityStorage {
        &mut self.entities
    }

}


pub struct ChunkStorage {
    /// Storing all cached chunks that were loaded from source.
    pub chunks: HashMap<(i32, i32), Arc<RwLock<Chunk>>>,
}

impl ChunkStorage {

    // CHUNKS //

    /// Insert a chunk at a specific position.
    pub fn insert_chunk(&mut self, chunk: Chunk) -> &Arc<RwLock<Chunk>> {
        let pos = chunk.get_position();
        let arc = Arc::new(RwLock::new(chunk));
        match self.chunks.entry(pos) {
            Entry::Occupied(mut o) => {
                o.insert(arc);
                o.into_mut()
            },
            Entry::Vacant(v) => {
                v.insert(arc)
            }
        }
    }

    /// Return true if a chunk is loaded at a specific position.
    pub fn is_chunk_loaded(&self, cx: i32, cz: i32) -> bool {
        self.chunks.contains_key(&(cx, cz))
    }

    /// Get a chunk reference at specific coordinates.
    pub fn get_chunk(&self, cx: i32, cz: i32) -> Option<RwLockReadGuard<Chunk>> {
        self.chunks.get(&(cx, cz)).map(|arc| arc.read().unwrap())
    }

    /// Get a mutable chunk reference at specific coordinates.
    pub fn mut_chunk(&self, cx: i32, cz: i32) -> Option<RwLockWriteGuard<Chunk>> {
        self.chunks.get(&(cx, cz)).map(|arc| arc.write().unwrap())
    }

    /// Get a chunk reference at specific blocks coordinates.
    pub fn get_chunk_at(&self, x: i32, z: i32) -> Option<RwLockReadGuard<Chunk>> {
        self.get_chunk(x >> 4, z >> 4)
    }

    /// Get a mutable chunk reference at specific blocks coordinates.
    pub fn mut_chunk_at(&self, x: i32, z: i32) -> Option<RwLockWriteGuard<Chunk>> {
        self.mut_chunk(x >> 4, z >> 4)
    }

    // BLOCKS //

    pub fn set_block_at(&self, x: i32, y: i32, z: i32, block: &'static BlockState) -> ChunkResult<()> {
        if let Some(mut chunk) = self.mut_chunk_at(x, z) {
            chunk.set_block_at(x, y, z, block)
        } else {
            Err(ChunkError::ChunkUnloaded)
        }
    }

    pub fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState> {
        if let Some(chunk) = self.get_chunk_at(x, z) {
            chunk.get_block_at(x, y, z)
        } else {
            Err(ChunkError::ChunkUnloaded)
        }
    }

}


pub struct EntityStorage {
    /// The ECS storing all entities in the level.
    pub ecs: EcsWorld,
    /// Cached `EntityBuilder`s for each static entity type.
    builder_cache: EntityBuilderCache
}

impl EntityStorage {

    fn spawn_entity_uninit(&mut self, entity_type: &'static EntityType) -> Entity {
        let builder = self.builder_cache.ensure(entity_type);
        self.ecs.spawn(builder.build())
    }

    pub fn spawn_entity(&mut self, entity_type: &'static EntityType) {
        unsafe {
            let entity = self.spawn_entity_uninit(entity_type);
            let identity = self.ecs.get_unchecked_mut::<EntityIdentity>(entity).unwrap();
            identity.uuid = Uuid::new_v4();
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        self.ecs.despawn(entity).is_ok()
    }

    pub fn get_entity_identity(&self, entity: Entity) -> EntityRef<EntityIdentity> {
        self.ecs.get(entity).unwrap()
    }

}


struct EntityBuilderCache(HashMap<OpaquePtr<EntityType>, EntityBuilder>);

impl EntityBuilderCache {

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn ensure(&mut self, entity_type: &'static EntityType) -> &mut EntityBuilder {
        match self.0.entry(OpaquePtr::new(entity_type)) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let mut builder = EntityBuilder::new();
                builder.add(EntityIdentity {
                    typ: entity_type,
                    uuid: Uuid::nil(),
                    pos: EntityPos::nil()
                });
                (entity_type.builder)(&mut builder);
                v.insert(builder)
            }
        }
    }

}


pub struct EntityIdentity {
    typ: &'static EntityType,
    uuid: Uuid,
    pos: EntityPos,
}
