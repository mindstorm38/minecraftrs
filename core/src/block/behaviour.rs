use std::sync::RwLock;
use std::collections::HashMap;

use once_cell::sync::OnceCell;
use crate::block::{Block, BlockKey};


pub struct BlockBehaviourRegister<T> {
    inner: OnceCell<RwLock<HashMap<BlockKey, T>>>
}

impl<T> BlockBehaviourRegister<T> {

    pub const fn new() -> Self {
        Self { inner: OnceCell::new() }
    }

    fn inner(&self) -> &RwLock<HashMap<BlockKey, T>> {
        self.inner.get_or_init(|| RwLock::new(HashMap::new()))
    }

    pub fn register(&self, block: &'static Block, behaviour: T) {
        self.inner().write().unwrap().insert(block.get_key(), behaviour);
    }

}
