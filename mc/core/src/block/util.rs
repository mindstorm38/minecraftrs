use std::sync::RwLock;
use std::collections::{HashMap, HashSet};

use once_cell::sync::OnceCell;

use crate::block::{Block, BlockKey};


/// A map structure that maps blocks (defined statically) to a value.
#[repr(transparent)]
pub struct BlockMap<T>(pub HashMap<BlockKey, T>);

impl<T> BlockMap<T> {

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, block: &'static Block, value: T) -> Option<T> {
        self.0.insert(block.get_key(), value)
    }

    pub fn remove(&mut self, block: &'static Block) -> Option<T> {
        self.0.remove(&block.get_key())
    }

    pub fn get(&self, block: &'static Block) -> Option<&T> {
        self.0.get(&block.get_key())
    }

}

/// A set structure that store blocks.
#[repr(transparent)]
pub struct BlockSet(pub HashSet<BlockKey>);

impl BlockSet {

    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn insert(&mut self, block: &'static Block) -> bool {
        self.0.insert(block.get_key())
    }

    pub fn extend<I: IntoIterator<Item = &'static Block>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().map(|b| b.get_key()));
    }

    pub fn remove(&mut self, block: &'static Block) -> bool {
        self.0.remove(&block.get_key())
    }

    pub fn contains(&self, block: &'static Block) -> bool {
        self.0.contains(&block.get_key())
    }

}



pub struct BlockStaticMap<T> {
    inner: OnceCell<RwLock<BlockMap<T>>>
}

impl<T> BlockStaticMap<T> {

    pub const fn new() -> Self {
        Self { inner: OnceCell::new() }
    }

    pub fn inner(&self) -> &RwLock<BlockMap<T>> {
        self.inner.get_or_init(|| RwLock::new(BlockMap::new()))
    }

    pub fn insert(&self, block: &'static Block, value: T) -> Option<T> {
        self.inner().write().unwrap().insert(block, value)
    }

    pub fn insert_with(&self, func: impl FnOnce(&mut BlockMap<T>)) {
        let mut map_guard = self.inner().write().unwrap();
        (func)(&mut *map_guard);
    }

    pub fn remove(&self, block: &'static Block) -> Option<T> {
        self.inner().write().unwrap().remove(block)
    }

    pub fn get(&self, block: &'static Block) -> Option<T>
    where
        T: Copy
    {
        self.inner().read().unwrap().get(block).copied()
    }

}


pub struct BlockStaticSet {
    inner: OnceCell<RwLock<BlockSet>>
}

impl BlockStaticSet {

    pub const fn new() -> Self {
        Self { inner: OnceCell::new() }
    }

    pub fn inner(&self) -> &RwLock<BlockSet> {
        self.inner.get_or_init(|| RwLock::new(BlockSet::new()))
    }

    pub fn insert(&self, block: &'static Block) -> bool {
        self.inner().write().unwrap().insert(block)
    }

    pub fn insert_with(&self, func: impl FnOnce(&mut BlockSet)) {
        let mut set_guard = self.inner().write().unwrap();
        (func)(&mut *set_guard);
    }

    pub fn extend<I: IntoIterator<Item = &'static Block>>(&self, iter: I) {
        self.inner().write().unwrap().extend(iter);
    }

    pub fn remove(&self, block: &'static Block) -> bool {
        self.inner().write().unwrap().remove(block)
    }

    pub fn contains(&self, block: &'static Block) -> bool {
        self.inner().read().unwrap().contains(block)
    }

}
