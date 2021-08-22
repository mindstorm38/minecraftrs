//! A tiny specific Entity Component System (ECS) for Minecraft entity ecosystem.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::any::{TypeId, Any};
use thiserror::Error;

mod storage;
pub use storage::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct EntityId(usize);


pub struct Ecs {
    entity_count: usize,
    components: HashMap<TypeId, Box<dyn Any>>
}

impl Ecs {

    pub fn new() -> Self {
        Self {
            entity_count: 0,
            components: HashMap::new()
        }
    }

    pub fn register_component<C: Component>(&mut self) {
        match self.components.entry(TypeId::of::<C>()) {
            Entry::Vacant(v) => {
                v.insert(Box::new(C::Storage::new()));
            },
            _ => {}
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity_id = self.entity_count;
        self.entity_count += 1;
        EntityBuilder {
            storage: self,
            eid: EntityId(entity_id)
        }
    }

    pub fn get_component_storage_ref<C: Component>(&self) -> Result<&C::Storage, EntityError> {
        match self.components.get(&TypeId::of::<C>()) {
            Some(any_storage) => Ok(any_storage.downcast_ref().unwrap()),
            None => Err(EntityError::UnknownComponent)
        }
    }

    pub fn get_component_storage_mut<C: Component>(&mut self) -> Result<&mut C::Storage, EntityError> {
        match self.components.get_mut(&TypeId::of::<C>()) {
            Some(any_storage) => Ok(any_storage.downcast_mut().unwrap()),
            None => Err(EntityError::UnknownComponent)
        }
    }

    pub fn set_component<C: Component>(&mut self, eid: EntityId, comp: C) -> Result<(), EntityError> {
        let storage: &mut C::Storage = self.get_component_storage_mut::<C>()?;
        storage.set_component(eid, comp);
        Ok(())
    }

    pub fn remove_component<C: Component>(&mut self, eid: EntityId) -> Result<C, EntityError> {
        let storage: &mut C::Storage = self.get_component_storage_mut::<C>()?;
        storage.remove_component(eid).ok_or(EntityError::NoComponent)
    }

    pub fn get_component_ref<C: Component>(&self, eid: EntityId) -> Result<&C, EntityError> {
        let storage: &C::Storage = self.get_component_storage_ref::<C>()?;
        storage.get_component_ref(eid).ok_or(EntityError::NoComponent)
    }

    pub fn get_component_mut<C: Component>(&mut self, eid: EntityId) -> Result<&mut C, EntityError> {
        let storage: &mut C::Storage = self.get_component_storage_mut::<C>()?;
        storage.get_component_mut(eid).ok_or(EntityError::NoComponent)
    }

    pub fn iter_component_ref<'a, C: Component>(&'a self) -> Result<impl Iterator<Item = &'a C>, EntityError>
        where
            &'a C::Storage: IntoIterator<Item = &'a C>
    {
        let storage: &C::Storage = self.get_component_storage_ref::<C>()?;
        Ok(storage.into_iter())
    }

    pub fn iter_component_mut<'a, C: Component>(&'a mut self) -> Result<impl Iterator<Item = &'a mut C>, EntityError>
        where
            &'a mut C::Storage: IntoIterator<Item = &'a mut C>
    {
        let storage: &mut C::Storage = self.get_component_storage_mut::<C>()?;
        Ok(storage.into_iter())
    }

}


pub struct EntityBuilder<'a> {
    storage: &'a mut Ecs,
    eid: EntityId
}

impl<'a> EntityBuilder<'a> {

    pub fn with<C: Component>(mut self, comp: C) -> Self {
        self.storage.set_component(self.eid, comp);
        self
    }

    pub fn build(mut self) -> EntityId {
        self.eid
    }

}


#[derive(Debug, Error, Copy, Clone)]
pub enum EntityError {
    #[error("Given component must be registered in the entity storage.")]
    UnknownComponent,
    #[error("Given entity does not have this component.")]
    NoComponent,
}


/// A typed storage used to store `Component`s.
pub trait ComponentStorage<T>: Any {
    fn new() -> Self;
    fn set_component(&mut self, eid: EntityId, comp: T) -> Option<T>;
    fn remove_component(&mut self, eid: EntityId) -> Option<T>;
    fn get_component_ref(&self, eid: EntityId) -> Option<&T>;
    fn get_component_mut(&mut self, eid: EntityId) -> Option<&mut T>;
}


/// A trait to implement for each of your components, the component must also
/// be bound to a static lifetime and be sized at compile time.
pub trait Component: Any + Sized {

    /// The storage type to use for the component, if the component is used by every entity the
    /// best is a `VecStorage`, if it is used by  few entities the best is a `HashMapStorage`.
    type Storage: ComponentStorage<Self>;

}