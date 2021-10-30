use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Instant, Duration};
use std::error::Error;
use std::fmt::{Debug, Formatter};

use crossbeam_channel::{Sender, Receiver, unbounded, bounded};
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
    /// `Ok(())` **you must** give a result later when calling `poll_chunk`. **This operation
    /// must be non-blocking.**
    fn request_chunk_load(&mut self, req: ChunkLoadRequest) -> Result<(), (LevelSourceError, ChunkLoadRequest)> {
        Err((LevelSourceError::UnsupportedChunkLoad, req))
    }

    /// Poll the next loaded chunk that is ready to be inserted into the level's chunk storage.
    /// Every requested load chunk `request_chunk_load` method that returned `Ok(())` should
    /// return some some result here, even if it's an error. **This operation must be
    /// non-blocking.**
    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>> {
        None
    }

    /// Request saving of the chunk at the given position. **This operation must be non-blocking.**
    #[allow(unused_variables)]
    fn request_chunk_save(&mut self, req: ChunkSaveRequest) -> Result<(), LevelSourceError> {
        Err(LevelSourceError::UnsupportedChunkSave)
    }

}


/// This structure is constructed by levels and passed to `LevelSource` when requesting for
/// chunk loading, the chunk must be constructed from the given data.
#[derive(Clone, Debug)]
pub struct ChunkLoadRequest {
    pub env: Arc<LevelEnv>,
    pub height: ChunkHeight,
    pub cx: i32,
    pub cz: i32,
}

impl ChunkLoadRequest {

    /// Build a chunk from this chunk info.
    pub fn build_chunk(&self) -> Chunk {
        Chunk::new(Arc::clone(&self.env), self.height, self.cx, self.cz)
    }

    pub fn build_proto_chunk(&self) -> ProtoChunk {
        ProtoChunk {
            inner: Box::new(self.build_chunk()),
            proto_entities: Vec::new(),
            dirty: false
        }
    }

}

#[derive(Clone)]
pub struct ChunkSaveRequest {
    pub cx: i32,
    pub cz: i32,
    pub chunk: Arc<RwLock<Chunk>>
}


/// A temporary chunk structure used to add entity builders that will be added to the level's ECS
/// later in sync when the source actually returns it.
pub struct ProtoChunk {
    /// A boxed chunk, the inner chunk is intentionally boxed because proto chunks are made
    /// to be moved and sent between threads.
    pub(super) inner: Box<Chunk>,
    /// Proto entities that will be built and added to the
    pub(super) proto_entities: Vec<(EntityBuilder, Option<Vec<usize>>)>,
    /// This boolean indicates when the proto chunk must be saved after added to the level.
    pub dirty: bool
}

impl ProtoChunk {

    /// Add an entity builder to this proto chunk, this builder will be added to the level when
    /// building the actual `Chunk`. **You must** ensure that this entity contains a `BaseEntity`
    /// component with an `entity_type` supported by the level's environment.
    ///
    /// This method also return the index of this entity within the proto chunk, this can be
    /// used to add passengers to this entity or make this entity ride another one.
    pub fn add_proto_entity(&mut self, entity_builder: EntityBuilder) -> usize {
        let idx = self.proto_entities.len();
        self.proto_entities.push((entity_builder, None));
        idx
    }

    pub fn add_proto_entity_passengers(&mut self, host_index: usize, passenger_index: usize) {
        self.proto_entities[host_index].1.get_or_insert_with(|| Vec::new()).push(passenger_index);
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

impl Debug for ProtoChunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProtoChunk")
            .field("dirty", &self.dirty)
            .finish_non_exhaustive()
    }
}


/// Dummy chunk loader which do nothing.
pub struct NullLevelSource;
impl LevelSource for NullLevelSource {}


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

    fn request_chunk_load(&mut self, req: ChunkLoadRequest) -> Result<(), (LevelSourceError, ChunkLoadRequest)> {
        match self.loader.request_chunk_load(req) {
            Err((LevelSourceError::UnsupportedChunkPosition, info)) => {
                // If the loader does not support this chunk, directly request the generator.
                self.generator.request_chunk_load(info)
            }
            Err(e) => Err(e),
            _ => Ok(())
        }
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>> {

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
        let mut res = self.generator.poll_chunk();
        if let Some(Ok(ref mut proto_chunk)) = res {
            // Because this proto chunk was just generated, mark it as dirty in
            // order to save it once added to the level.
            proto_chunk.dirty = true;
        }
        res

    }

    fn request_chunk_save(&mut self, req: ChunkSaveRequest) -> Result<(), LevelSourceError> {
        self.loader.request_chunk_save(req)
    }

}


/// A trait to implement for level generators, this trait provides a synchronous method to
/// generate a specific chunk. This trait is not valid for methods expecting a `LevelSource`,
/// to do this you need to wrap it into `LevelGeneratorSource`, this structure will clone your
/// generator in any given workers count and run them asynchronously.
pub trait LevelGenerator {
    fn generate(&mut self, info: ChunkLoadRequest) -> Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>;
}


/// A trait used by `LevelGeneratorSource` in order to build a specific `LevelGenerator` in each
/// worker thread, this allows the generator to be thread unsafe. For this specific use, this
/// implies that this builder should be send and sync for.
pub trait LevelGeneratorBuilder {
    type Generator: LevelGenerator;
    fn build(&mut self) -> Self::Generator;
}


/// A wrapper for `LevelGenerator` that implements `LevelSource` to provide asynchronous level
/// generation. This wrapper dispatches incoming chunk request into the given number of worker
/// threads.
pub struct LevelGeneratorSource {
    request_sender: Sender<ChunkLoadRequest>,
    result_receiver: Receiver<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>>,
}

impl LevelGeneratorSource {

    pub fn new<B>(generator_builder: B, workers_count: usize) -> Self
    where
        B: LevelGeneratorBuilder + Send + Sync + 'static
    {

        let (
            request_sender,
            request_receiver
        ) = unbounded();

        let (
            result_sender,
            result_receiver
        ) = bounded(workers_count * 128);

        let generator_builder = Arc::new(Mutex::new(generator_builder));

        for i in 0..workers_count {

            let request_receiver = request_receiver.clone();
            let result_sender = result_sender.clone();
            let generator_builder = Arc::clone(&generator_builder);

            std::thread::Builder::new()
                .name(format!("Level generator worker #{}", i))
                .spawn(move || {
                    let worker = {
                        LevelGeneratorSourceWorker {
                            generator: generator_builder.lock().unwrap().build(),
                            request_receiver,
                            result_sender,
                            total_count: 0,
                            total_duration: Duration::default(),
                        }
                    };
                    worker.run()
                })
                .unwrap();

        }

        Self {
            request_sender,
            result_receiver
        }

    }

}

impl LevelSource for LevelGeneratorSource {

    fn request_chunk_load(&mut self, req: ChunkLoadRequest) -> Result<(), (LevelSourceError, ChunkLoadRequest)> {
        // SAFETY: Unwrap should be safe because the channel is unbounded.
        self.request_sender.send(req).unwrap();
        Ok(())
    }

    fn poll_chunk(&mut self) -> Option<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>> {
        self.result_receiver.try_recv().ok()
    }

}

/// Internal thread structure used by `LevelGeneratorSource`.
struct LevelGeneratorSourceWorker<G> {
    generator: G,
    request_receiver: Receiver<ChunkLoadRequest>,
    result_sender: Sender<Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)>>,
    total_count: u32,
    total_duration: Duration
}

impl<G> LevelGeneratorSourceWorker<G>
where
    G: LevelGenerator
{

    fn run(mut self) {
        loop {
            // TODO: println!("[{}] Waiting...", std::thread::current().name().unwrap());
            match self.request_receiver.recv() {
                Ok(chunk_info) => {
                    let begin = Instant::now();
                    let res = self.generator.generate(chunk_info);
                    self.total_duration += begin.elapsed();
                    self.total_count += 1;
                    // TODO: println!("[{}] Average time: {:?}", std::thread::current().name().unwrap(), self.total_duration / self.total_count);
                    if let Err(_) = self.result_sender.send(res) {
                        break
                    }
                },
                Err(_) => break
            }
        }
    }

}


/// A primitive super-flat generator that only generate the terrain from given layers,
/// no structure is generated.
#[derive(Debug, Clone)]
pub struct SuperFlatGenerator {
    layers: Vec<(&'static BlockState, i32, u32)>
}

impl SuperFlatGenerator {

    pub fn new() -> Self {
        Self {
            layers: Vec::new()
        }
    }

    pub fn add_layer(&mut self, state: &'static BlockState, y: i32, height: u32) {
        self.layers.push((state, y, height));
    }

}

impl LevelGenerator for SuperFlatGenerator {

    fn generate(&mut self, info: ChunkLoadRequest) -> Result<ProtoChunk, (LevelSourceError, ChunkLoadRequest)> {
        let mut chunk = info.build_proto_chunk();
        for &(state, y, height) in &self.layers {
            for y in y..(y + height as i32) {
                // TODO: This algorithm is not optimized, we can optimize it if we add
                //  a "fill_blocks" method in "Chunk".
                for x in 0..16 {
                    for z in 0..16 {
                        let _ = chunk.set_block(x, y, z, state);
                    }
                }
            }
        }
        Ok(chunk)
    }

}
