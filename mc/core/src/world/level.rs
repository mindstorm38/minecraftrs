use std::sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use hecs::{World as EcsWorld, EntityBuilder, Entity, EntityRef};
use uuid::Uuid;

use crate::entity::{GlobalEntities, EntityType};
use crate::block::{GlobalBlocks, BlockState};
use crate::biome::GlobalBiomes;
use crate::pos::{EntityPos, BlockPos};
use crate::debug;

use super::chunk::{Chunk, ChunkHeight, ChunkResult, ChunkError};
use super::source::{LevelSource, ChunkInfo, LevelSourceError, ProtoChunk};


/// A structure that contains the static environment of a World, this can be used for multiple
/// `Level`s through an `Arc<LevelEnv>`.
pub struct LevelEnv {
    /// Global blocks palette.
    pub blocks: GlobalBlocks,
    /// Global biomes palette.
    pub biomes: GlobalBiomes,
    /// Global entity types palette.
    pub entities: GlobalEntities
}

impl LevelEnv {

    pub fn new(
        blocks: GlobalBlocks,
        biomes: GlobalBiomes,
        entities: GlobalEntities
    ) -> Self {
        LevelEnv {
            blocks,
            biomes,
            entities
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
    source: Box<dyn LevelSource>,
    /// The configured height of this level.
    height: ChunkHeight,
    /// Chunk storage.
    chunks: ChunkStorage,
    /// Entities storage.
    entities: EntityStorage
}

impl Level {

    pub fn new<S>(id: String, env: Arc<LevelEnv>, height: ChunkHeight, source: S) -> Self
    where
        S: LevelSource + 'static,
    {

        assert_ne!(env.blocks.states_count(), 0, "The given environment has no state, a level requires at least one block state.");
        assert_ne!(env.biomes.biomes_count(), 0, "The given environment has no biome, a level requires at least one biome.");

        Level {
            id,
            height,
            source: Box::new(source),
            chunks: ChunkStorage {
                chunks: HashMap::new()
            },
            entities: EntityStorage {
                ecs: EcsWorld::new(),
                builder: EntityBuilder::new()
            },
            env,
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
        matches!(self.source.request_chunk_load(ChunkInfo {
            env: self.get_env(),
            height: self.height,
            cx,
            cz
        }), Ok(_))
    }

    /// Poll loaded chunks from internal level source, all successfully loaded chunks
    /// are added to the underlying `LevelStorage`. The callback is called for each
    /// loaded chunks or loading error.
    pub fn load_chunks_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut((i32, i32, Result<&Arc<RwLock<Chunk>>, LevelSourceError>)),
    {
        while let Some(res) = self.source.poll_chunk() {
            match res {
                Ok(ProtoChunk {
                    inner: mut chunk,
                    mut proto_entities
                }) => {

                    let (cx, cz) = chunk.get_position();
                    debug!("Loaded chunk at {}/{}", cx, cz);

                    // This list retains all entity handles with the same order as proto chunk
                    // entities data.
                    let mut entities_handles = Vec::with_capacity(proto_entities.len());

                    // Then we only add entities without their passengers, but store their handles.
                    for (entity_builder, _) in &mut proto_entities {
                        unsafe {
                            let entity = self.entities.add_entity_unchecked(entity_builder);
                            chunk.add_entity_unchecked(entity);
                            entities_handles.push(entity);
                        }
                    }

                    // Now that we have created all our entities, we can set passengers.
                    for (i, (_, passengers)) in proto_entities.into_iter().enumerate() {
                        if let Some(passengers) = passengers {

                            // Here we don't check is passengers is empty because ProtoChunk only
                            // set 'Some' if there are passengers.

                            // SAFETY: Unwrap is safe because the entity was just created with `BaseEntity` component.
                            let mut base_entity = self.entities.ecs.get_mut::<BaseEntity>(entities_handles[i]).unwrap();
                            let mut passengers_handles = Vec::with_capacity(passengers.len());

                            for passenger_proto_index in passengers {
                                passengers_handles.push(entities_handles[passenger_proto_index]);
                            }

                            base_entity.passengers = Some(passengers_handles);

                        }
                    }

                    callback((cx, cz, Ok(self.chunks.insert_chunk(chunk))));

                },
                Err((err, chunk_info)) => {
                    // IDE shows an error for 'Display' not being implemented, but we use the
                    // crate 'thiserror' to implement it through a custom derive.
                    debug!("Failed to load chunk at {}/{}: {}", chunk_info.cx, chunk_info.cz, err);
                    callback((chunk_info.cx, chunk_info.cz, Err(err)));
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

    pub fn spawn_entity(&mut self, entity_type: &'static EntityType, pos: EntityPos) -> Option<Entity> {

        if !self.env.entities.has_entity_type(entity_type) {
            return None;
        }

        let chunk = self.chunks.get_chunk_at_block_mut(BlockPos::from(&pos));
        let entity = unsafe { self.entities.spawn_entity_unchecked(entity_type, pos) };

        if let Some(mut chunk) = chunk {
            unsafe {
                chunk.add_entity_unchecked(entity);
            }
        }

        Some(entity)

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
    pub fn get_chunk_mut(&self, cx: i32, cz: i32) -> Option<RwLockWriteGuard<Chunk>> {
        self.chunks.get(&(cx, cz)).map(|arc| arc.write().unwrap())
    }

    /// Get a chunk reference at specific blocks coordinates.
    #[inline]
    pub fn get_chunk_at(&self, x: i32, z: i32) -> Option<RwLockReadGuard<Chunk>> {
        self.get_chunk(x >> 4, z >> 4)
    }

    /// Get a mutable chunk reference at specific blocks coordinates.
    #[inline]
    pub fn get_chunk_at_mut(&self, x: i32, z: i32) -> Option<RwLockWriteGuard<Chunk>> {
        self.get_chunk_mut(x >> 4, z >> 4)
    }

    #[inline]
    pub fn get_chunk_at_block(&self, block_pos: BlockPos) -> Option<RwLockReadGuard<Chunk>> {
        self.get_chunk_at(block_pos.x, block_pos.z)
    }

    #[inline]
    pub fn get_chunk_at_block_mut(&self, block_pos: BlockPos) -> Option<RwLockWriteGuard<Chunk>> {
        self.get_chunk_at_mut(block_pos.x, block_pos.z)
    }

    // BLOCKS //

    pub fn set_block_at(&self, x: i32, y: i32, z: i32, block: &'static BlockState) -> ChunkResult<()> {
        if let Some(mut chunk) = self.get_chunk_at_mut(x, z) {
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
    /// Internal entity builder kept
    builder: EntityBuilder,
}

impl EntityStorage {

    /// Spawn an entity in the level owning this storage, you must give its type and position,
    /// its handle is returned. If the given entity type is not supported by the level's
    /// environment, `None` is returned.
    ///
    /// # Safety:
    /// This method is made for internal use because the entity type must be supported checked
    /// to be supported by level's environment. The returned entity handle must also be added
    /// in the the associated chunk if existing.
    ///
    /// # See:
    /// Use `Level::spawn_entity` instead of this method if you want to avoid safety issues.
    pub unsafe fn spawn_entity_unchecked(&mut self, entity_type: &'static EntityType, pos: EntityPos) -> Entity {

        self.builder.add(BaseEntity::new(entity_type, Uuid::new_v4(), pos));

        for &component in entity_type.codecs {
            component.default(&mut self.builder);
        }

        self.ecs.spawn(self.builder.build())

    }

    /// Add a raw entity from a builder, this method is unsafe because the caller must ensure
    /// that the builder contains a `BaseEntity` component with an entity
    pub unsafe fn add_entity_unchecked(&mut self, entity_builder: &mut EntityBuilder) -> Entity {
        self.ecs.spawn(entity_builder.build())
    }

    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        self.ecs.despawn(entity).is_ok()
    }

    pub fn get_entity_ref(&self, entity: Entity) -> Option<EntityRef> {
        self.ecs.entity(entity).ok()
    }

}

/// Base entity component, present in all entities of a level, must not be removed.
pub struct BaseEntity {
    pub entity_type: &'static EntityType,
    pub uuid: Uuid,
    pub pos: EntityPos,
    /// An optional list of entities that are on top of this one.
    passengers: Option<Vec<Entity>>
}

impl BaseEntity {

    pub fn new(entity_type: &'static EntityType, uuid: Uuid, pos: EntityPos) -> Self {
        Self {
            entity_type,
            uuid,
            pos,
            passengers: None
        }
    }

}