use crate::util::{Components, ComponentError, SystemExecutor, tick_loop};

use std::cell::{Ref, RefMut, RefCell};
use std::any::Any;
use std::rc::Rc;


pub type WorldSystemExecutor = SystemExecutor<World>;


pub struct World {
    /// Internal components registered before starting and used after by systems.
    components: Components,
    /// External executor.
    executor: Rc<RefCell<WorldSystemExecutor>>,
}

impl World {

    pub fn new() -> Self {
        Self {
            components: Components::new(),
            executor: Rc::new(RefCell::new(SystemExecutor::new()))
        }
    }

    #[inline]
    pub fn clone_executor(&self) -> Rc<RefCell<WorldSystemExecutor>> {
        Rc::clone(&self.executor)
    }

    /// If you want to run multiple worlds on the same executor, you should set
    #[inline]
    pub fn set_executor(&mut self, executor: Rc<RefCell<WorldSystemExecutor>>) {
        self.executor = executor;
    }

    pub fn with_executor<F>(&mut self, func: F)
    where
        F: FnOnce(&mut World, &mut WorldSystemExecutor)
    {
        let executor = self.clone_executor();
        func(self, &mut executor.borrow_mut());
    }

    #[inline]
    pub fn insert_component<T: Any>(&mut self, component: T) -> Option<T> {
        self.components.insert(component)
    }

    #[inline]
    pub fn remove_component<T: Any>(&mut self) -> Option<T> {
        self.components.remove::<T>()
    }

    #[inline]
    pub fn get_component<T: Any>(&self) -> Result<Ref<T>, ComponentError> {
        self.components.get::<T>()
    }

    #[inline]
    pub fn get_component_mut<T: Any>(&self) -> Result<RefMut<T>, ComponentError> {
        self.components.get_mut::<T>()
    }

    pub fn simple_run(&mut self) {
        let executor = self.clone_executor();
        tick_loop(move |info| {
            executor.borrow_mut().tick(self);
            false
        }, 20.0);
    }

}
