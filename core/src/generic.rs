use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::hash_map::Entry;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::any::{TypeId, Any};


/// Type alias for the internal map.
type Map = HashMap<TypeId, Box<(dyn Any + Send + Sync)>>;

/// A standard generic map.
#[derive(Debug)]
pub struct GenericMap(Map);

/// An RwLocked Generic map, allow interior mutability with Send/Sync.
#[derive(Debug)]
pub struct RwGenericMap(RwLock<GenericMap>);


impl GenericMap {

    pub fn new() -> GenericMap {
        GenericMap(HashMap::new())
    }

    pub fn add<E: Any + Sync + Send>(&mut self, obj: E) {
        match self.0.entry(obj.type_id()) {
            Entry::Occupied(_) => println!("This type of extension already set for this block."),
            Entry::Vacant(v) => { v.insert(Box::new(obj)); }
        }
    }

    pub fn get<E: Any + Sync + Send>(&self) -> Option<&E> {
        let v = self.0.get(&TypeId::of::<E>())?;
        let v = &**v; // This ref points the box content
        v.downcast_ref()
    }

    pub fn get_mut<E: Any + Sync + Send>(&mut self) -> Option<&mut E> {
        let v = self.0.get_mut(&TypeId::of::<E>())?;
        let v = &mut **v; // This ref points the box content
        v.downcast_mut()
    }

}


impl RwGenericMap {

    pub fn new() -> RwGenericMap {
        RwGenericMap(RwLock::new(GenericMap::new()))
    }

    pub fn add<E: Any + Sync + Send>(&self, obj: E) {
        self.0.write().unwrap().add(obj);
    }

    pub fn get<E: Any + Sync + Send>(&self) -> Option<GuardedRef<E>> {
        let guard = self.0.read().unwrap();
        // SAFETY: This reference point to the box content and is valid until 'guard' is
        // dropped, otherwise its content may have been dropped by another RwLockWriteGuard.
        let v = guard.get::<E>()? as *const E;
        Some(GuardedRef(guard, v))
    }

    pub fn get_mut<E: Any + Sync + Send>(&self) -> Option<GuardedMut<E>> {
        let mut guard = self.0.write().unwrap();
        // SAFETY: This reference point to the box content and is valid until 'guard' is
        // dropped, otherwise its content may have been dropped by another RwLockWriteGuard.
        let v = guard.get_mut::<E>()? as *mut E;
        Some(GuardedMut(guard, v))
    }

}


#[allow(dead_code)]
pub struct GuardedRef<'a, V>(RwLockReadGuard<'a, GenericMap>, *const V);

impl<V> Deref for GuardedRef<'_, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.1 }
    }
}


#[allow(dead_code)]
pub struct GuardedMut<'a, V>(RwLockWriteGuard<'a, GenericMap>, *mut V);

impl<V> Deref for GuardedMut<'_, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.1 }
    }
}

impl<V> DerefMut for GuardedMut<'_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.1 }
    }
}
