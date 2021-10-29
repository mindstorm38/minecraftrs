use std::any::{Any, TypeId, type_name};
use std::cell::{RefCell, Ref, RefMut};
use std::collections::HashMap;
use thiserror::Error;


/// A generic hash map. This is used by the world to keep track of each component and
/// can be used later by systems registered in the world's system executor.
#[derive(Debug)]
pub struct Components(HashMap<TypeId, RefCell<Box<dyn Any>>>);

impl Components {

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
    fn get_cell<T: Any>(&self) -> Result<&RefCell<Box<dyn Any>>, ComponentError> {
        self.0.get(&TypeId::of::<T>()).ok_or_else(|| {
            ComponentError::Unknown(type_name::<T>())
        })
    }

    pub fn get<T: Any>(&self) -> Result<Ref<'_, T>, ComponentError> {
        match self.get_cell::<T>()?.try_borrow() {
            Ok(rf) => {
                Ok(Ref::map(rf, |rf| rf.downcast_ref::<T>().unwrap()))
            },
            Err(_) => Err(ComponentError::Borrowed(type_name::<T>()))
        }
    }

    pub fn get_mut<T: Any>(&self) -> Result<RefMut<'_, T>, ComponentError> {
        match self.get_cell::<T>()?.try_borrow_mut() {
            Ok(rf) => {
                Ok(RefMut::map(rf, |rf| rf.downcast_mut::<T>().unwrap()))
            },
            Err(_) => Err(ComponentError::Borrowed(type_name::<T>()))
        }
    }

}


#[derive(Debug, Error)]
pub enum ComponentError {
    #[error("Unknown component of type '{0}'.")]
    Unknown(&'static str),
    #[error("The component of type '{0}' is already borrowed.")]
    Borrowed(&'static str),
}
