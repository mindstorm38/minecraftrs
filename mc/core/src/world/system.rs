use std::cell::{RefCell, Ref, RefMut, BorrowError};
use std::any::{TypeId, Any, type_name};
use std::collections::HashMap;
use thiserror::Error;

use super::World;


struct System {

}


pub struct SystemExecutor {
    systems: Vec<System>
}

impl SystemExecutor {

    pub fn tick(&mut self, world: &mut World) {
        for system in self.systems {

        }
    }

    pub fn add_system<S>(&mut self, system: S)
    where
        S: FnMut(&mut World) {

    }

}


#[derive(Debug)]
pub struct SystemComponents(HashMap<TypeId, RefCell<Box<dyn Any>>>);

impl SystemComponents {

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<T: Any>(&mut self, component: T) -> Option<T> {
        let cell = RefCell::new(Box::new(component));
        self.0.insert(TypeId::of::<T>(), cell).map(|bx| {
            // SAFETY: Unwrap is safe because the map key check the type.
            *bx.into_inner().downcast().unwrap()
        })
    }

    pub fn remove<T: Any>(&mut self) -> Option<T> {
        self.0.remove(&TypeId::of::<T>()).map(|bx| {
            // SAFETY: Unwrap is safe because the map key check the type.
            *bx.into_inner().downcast().unwrap()
        })
    }

    #[inline]
    fn get_cell<T: Any>(&self) -> Result<&RefCell<Box<dyn Any>>, SystemComponentError> {
        self.0.get(&TypeId::of::<T>()).ok_or_else(|| {
            SystemComponentError::Unknown(type_name::<T>())
        })
    }

    pub fn get<T: Any>(&self) -> Result<Ref<T>, SystemComponentError> {
        match self.get_cell::<T>()?.try_borrow() {
            Ok(rf) => {
                Ok(Ref::map(rf, |rf| rf.downcast_ref::<T>().unwrap()))
            },
            Err(_) => Err(SystemComponentError::Borrowed(type_name::<T>()))
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Result<RefMut<T>, SystemComponentError> {
        match self.get_cell::<T>()?.try_borrow_mut() {
            Ok(rf) => {
                Ok(RefMut::map(rf, |rf| rf.downcast_mut::<T>().unwrap()))
            },
            Err(_) => Err(SystemComponentError::Borrowed(type_name::<T>()))
        }
    }

}


#[derive(Error)]
pub enum SystemComponentError {
    #[error("Unknown component of type '{0}'.")]
    Unknown(&'static str),
    #[error("The component of type '{0}' is already borrowed.")]
    Borrowed(&'static str),
}
