use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::error::Error;

use hecs::EntityBuilder;
use thiserror::Error;

use super::chunk::{Chunk, ChunkHeight};
use crate::world::level::LevelEnv;
use crate::block::BlockState;


/// Common level source error.
#[derive(Error, Debug)]
pub enum LevelSourceError {
    #[error("The required chunk position is not supported by the source.")]
    UnsupportedChunkPosition,
    #[error("Chunk loading is not supported by the targeted source.")]
    UnsupportedChunkLoad,
    #[error("Chunk saving is not supported by the targeted source.")]
    UnsupportedChunkSave,
    #[error("Custom source error: {0}")]
    Custom(Box<dyn Error + Send>)
}

impl LevelSourceError {
    pub fn new_custom(err: impl Error + Send + 'static) -> Self {
        Self::Custom(Box::new(err))
    }
}


/// Level loader trait to implement for each different loader such as
/// disk or generator loaders, combine these two types of loaders to
/// save and load change in a chunk to avoid generating twice.
pub trait LevelSource {

    /// Request loading of the chunk at the given position. If you return an error, you must
    /// return back the given `ChunkInfo` together with the `LevelSourceError`. If you return
    /// `Ok(())` **you must** give a result later when calling `poll_chunk`.
    fn request_chunk_load(&mut self, info: ChunkInfo) -> Result<(), (LevelSourceError, ChunkInfo)> {
        Err((LevelSourceError::UnsupportedChunkLoad, info))
    }

    /// Poll the next loaded chunk that is ready to be inserted into the level's chunk storage.
    /// Every requested load chunk `request_chunk_load` method that returned `Ok(())` should
    /// return some some result here, even if it's an error.
    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>> {
        None
    }

    /// Request saving of the chunk at the given position.
    #[allow(unused_variables)]
    fn request_chunk_save(&mut self, chunk: Arc<RwLock<Chunk>>) -> Result<(), LevelSourceError> {
        Err(LevelSourceError::UnsupportedChunkSave)
    }

}


pub trait SyncLevelSource {
    fn chunk_load(info: ChunkInfo) -> Result<ProtoChunk, (LevelSourceError, ChunkInfo)>;
}


/// This structure is constructed by levels and passed to `LevelSource` when requesting for
/// chunk loading, the chunk must be constructed from the given data.
#[derive(Clone)]
pub struct ChunkInfo {
    pub env: Arc<LevelEnv>,
    pub height: ChunkHeight,
    pub cx: i32,
    pub cz: i32,
}

impl ChunkInfo {

    /// Build a chunk from this chunk info.
    pub fn build_chunk(&self) -> Chunk {
        Chunk::new(Arc::clone(&self.env), self.height, self.cx, self.cz)
    }

    pub fn build_proto_chunk(&self) -> ProtoChunk {
        ProtoChunk {
            inner: self.build_chunk(),
            entities: Vec::new()
        }
    }

}


/// A temporary chunk structure used to add entity builders that will be added to the level's ECS
/// later in sync when the source actually returns it.
pub struct ProtoChunk {
    pub(super) inner: Chunk,
    pub(super) entities: Vec<(EntityBuilder, Option<Vec<usize>>)>
}

impl ProtoChunk {

    /// Add an entity builder to this proto chunk, this builder will be added to the level when
    /// building the actual `Chunk`. **You must** ensure that this entity contains a `BaseEntity`
    /// component with an `entity_type` supported by the level's environment.
    ///
    /// This method also return the index of this entity within the proto chunk, this can be
    /// used to add passengers to this entity or make this entity ride another one.
    pub fn add_proto_entity(&mut self, entity_builder: EntityBuilder) -> usize {
        let idx = self.entities.len();
        self.entities.push((entity_builder, None));
        idx
    }

    pub fn add_proto_entity_passengers(&mut self, host_index: usize, passenger_index: usize) {
        self.entities[host_index].1.get_or_insert_with(|| Vec::new()).push(passenger_index);
    }

}

impl Deref for ProtoChunk {
    type Target = Chunk;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ProtoChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


/// Dummy chunk loader which do nothing.
pub struct NullLevelSource;
impl LevelSource for NullLevelSource {}


/// A primitive super-flat generator that only generate the terrain from given layers,
/// no structure is generated.
pub struct SuperFlatGenerator {
    layers: Vec<(&'static BlockState, i32, u32)>,
    chunks: VecDeque<ProtoChunk>
}

impl SuperFlatGenerator {

    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            chunks: VecDeque::new()
        }
    }

    pub fn add_layer(&mut self, state: &'static BlockState, y: i32, height: u32) {
        self.layers.push((state, y, height));
    }

    /// Internal method to generate super-flat chunk.
    fn generate_chunk(&self, chunk: &mut ProtoChunk) {
        for &(state, y, height) in &self.layers {
            for y in y..(y + height as i32) {
                // TODO: This algorithm is not optimized, we can optimize it if we add
                //  a "fill_blocks" method in "Chunk".
                for x in 0..16 {
                    for z in 0..16 {
                        chunk.set_block(x, y, z, state);
                    }
                }
            }
        }
    }

}

impl LevelSource for SuperFlatGenerator {

    fn request_chunk_load(&mut self, info: ChunkInfo) -> Result<(), (LevelSourceError, ChunkInfo)> {
        let mut chunk = info.build_proto_chunk();
        self.generate_chunk(&mut chunk);
        self.chunks.push_back(chunk);
        Ok(())
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>> {
        self.chunks.pop_front().map(|chunk| Ok(chunk))
    }

}


/// A load or generate LevelSource variant.
///
/// This can be used for exemple with an anvil region source as the loader and a super-flat
/// generator as generator. In this case the generator will be called only for chunks that
/// are not supported by the loader (returned UnsupportedChunkPosition error).
pub struct LoadOrGenLevelSource<L, G> {
    loader: L,
    generator: G
}

impl<L, G> LoadOrGenLevelSource<L, G>
where
    L: LevelSource,
    G: LevelSource,
{

    /// Construct a new load or generate `LevelSource`. You should ensure that the given
    /// sources does not return `UnsupportedOperation` for `request_chunk_load`. If they does,
    /// this source will also return this type of error when requesting chunk load and then
    /// will be unusable.
    pub fn new(loader: L, generator: G) -> Self {
        Self {
            loader,
            generator
        }
    }

}

impl<L, G> LevelSource for LoadOrGenLevelSource<L, G>
where
    L: LevelSource,
    G: LevelSource,
{

    fn request_chunk_load(&mut self, info: ChunkInfo) -> Result<(), (LevelSourceError, ChunkInfo)> {
        match self.loader.request_chunk_load(info) {
            Err((LevelSourceError::UnsupportedChunkPosition, info)) => {
                // If the loader does not support this chunk, directly request the generator.
                self.generator.request_chunk_load(info)
            }
            Err(e) => Err(e),
            _ => Ok(())
        }
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>> {

        // We check the loader first.
        while let Some(res) = self.loader.poll_chunk() {
            match res {
                // If the source error is an unsupported position, just delegate to the generator.
                Err((LevelSourceError::UnsupportedChunkPosition, chunk_info)) => {
                    match self.generator.request_chunk_load(chunk_info) {
                        Err(e) => return Some(Err(e)),
                        Ok(_) => {}
                    }
                },
                // If this is not an unsupported position, Ok or other Err, just return it.
                res => return Some(res)
            }
        }

        // Then we poll chunks from the generator.
        self.generator.poll_chunk()

    }

}

use crossbeam_channel::{Sender, Receiver, RecvTimeoutError, unbounded, bounded};

pub struct ThreadedLevelSource {
    request_sender: Sender<ChunkInfo>,
    result_receiver: Receiver<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>>,
}

impl ThreadedLevelSource {

    pub fn spawn<S>(source: S) -> Self
    where
        S: LevelSource
    {

        let (
            request_sender,
            request_receiver
        ) = unbounded();

        Self {
            request_sender,
            result_receiver: ThreadedLevelSourceWorker::spawn(source, request_receiver)
        }

    }

}


struct ThreadedLevelSourceWorker<S: LevelSource> {
    source: S,
    request_receiver: Receiver<ChunkInfo>,
    result_sender: Sender<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>>,
}

impl<S: LevelSource> ThreadedLevelSourceWorker<S> {

    fn spawn(source: S, request_receiver: Receiver<ChunkInfo>) -> Receiver<Result<ProtoChunk, (LevelSourceError, ChunkInfo)>> {

        let (
            result_sender,
            result_receiver
        ) = bounded(128);

        let worker = Self {
            source,
            request_receiver,
            result_sender
        };

        std::thread::spawn(move || worker.run());

        result_receiver

    }

    fn run(mut self) {

    }

}