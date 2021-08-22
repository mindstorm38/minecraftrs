use std::collections::hash_map::{Values, ValuesMut};
use std::collections::HashMap;

use super::{ComponentStorage, EntityId};


/// A vector component storage, should be used for components that are widely used by entities.
pub struct VecStorage<T: 'static>(Vec<Option<T>>);

impl<T: 'static> ComponentStorage<T> for VecStorage<T> {

    fn new() -> Self {
        Self(Vec::new())
    }

    fn set_component(&mut self, eid: EntityId, comp: T) -> Option<T> {
        if eid.0 >= self.0.len() {
            self.0.resize_with(eid.0 + 1, || None);
        }
        self.0[eid.0].replace(comp)
    }

    fn remove_component(&mut self, eid: EntityId) -> Option<T> {
        self.0.get_mut(eid.0)?.take()
    }

    fn get_component_ref(&self, eid: EntityId) -> Option<&T> {
        self.0.get(eid.0)?.as_ref()
    }

    fn get_component_mut(&mut self, eid: EntityId) -> Option<&mut T> {
        self.0.get_mut(eid.0)?.as_mut()
    }

}

pub struct VecStorageIter<'a, T: 'static>(std::slice::Iter<'a, Option<T>>);
pub struct VecStorageIterMut<'a, T: 'static>(std::slice::IterMut<'a, Option<T>>);

impl<'a, T: 'static> Iterator for VecStorageIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.0.next()?.as_ref() {
                return Some(item);
            }
        }
    }
}

impl<'a, T: 'static> Iterator for VecStorageIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.0.next()?.as_mut() {
                return Some(item);
            }
        }
    }
}

impl<'a, T: 'static> IntoIterator for &'a VecStorage<T> {
    type Item = &'a T;
    type IntoIter = VecStorageIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        VecStorageIter(self.0.iter())
    }
}

impl<'a, T: 'static> IntoIterator for &'a mut VecStorage<T> {
    type Item = &'a mut T;
    type IntoIter = VecStorageIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        VecStorageIterMut(self.0.iter_mut())
    }
}

/// A hash map component storage, should be used for components that are used by few entities.
pub struct HashMapStorage<T>(HashMap<EntityId, T>);

impl<T: 'static> ComponentStorage<T> for HashMapStorage<T> {

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn set_component(&mut self, eid: EntityId, comp: T) -> Option<T> {
        self.0.insert(eid, comp)
    }

    fn remove_component(&mut self, eid: EntityId) -> Option<T> {
        self.0.remove(&eid)
    }

    fn get_component_ref(&self, eid: EntityId) -> Option<&T> {
        self.0.get(&eid)
    }

    fn get_component_mut(&mut self, eid: EntityId) -> Option<&mut T> {
        self.0.get_mut(&eid)
    }

}

impl<'a, T: 'static> IntoIterator for &'a HashMapStorage<T> {
    type Item = &'a T;
    type IntoIter = Values<'a, EntityId, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.values()
    }
}

impl<'a, T: 'static> IntoIterator for &'a mut HashMapStorage<T> {
    type Item = &'a mut T;
    type IntoIter = ValuesMut<'a, EntityId, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.values_mut()
    }
}
