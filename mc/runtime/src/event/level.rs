use std::sync::{RwLock, Arc};
use std::cell::RefCell;
use std::rc::Rc;

use mc_core::world::level::Level;
use mc_core::world::chunk::Chunk;


pub struct ChunkLoadedEvent {
    pub level: Rc<RefCell<Level>>,
    pub chunk: Arc<RwLock<Chunk>>,
    pub cx: i32,
    pub cz: i32
}
