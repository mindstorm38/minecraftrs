use std::cell::{Ref, RefMut, RefCell};
use std::any::Any;
use std::rc::Rc;

use mc_core::world::level::Level;

use crate::util::{Components, ComponentError, SystemExecutor, EventTracker, tick_loop};


/// Type alias for a `SystemExecutor` that take a `World` context.
pub type WorldSystemExecutor = SystemExecutor<World>;


pub struct World {
    /// Internal components registered before starting and used after by systems.
    components: Components,
    /// External executor.
    pub executor: Rc<RefCell<WorldSystemExecutor>>,
    /// Internal event tracker for the world.
    pub event_tracker: EventTracker,
    /// World's levels.
    pub levels: Vec<Rc<RefCell<Level>>>
}

impl World {

    pub fn new() -> Self {
        Self {
            components: Components::new(),
            executor: Rc::new(RefCell::new(SystemExecutor::new())),
            event_tracker: EventTracker::new(),
            levels: Vec::new()
        }
    }

    // Executor

    #[inline]
    pub fn clone_executor(&self) -> Rc<RefCell<WorldSystemExecutor>> {
        Rc::clone(&self.executor)
    }

    pub fn with_executor<F>(&mut self, func: F)
    where
        F: FnOnce(&mut World, &mut WorldSystemExecutor)
    {
        let executor = self.clone_executor();
        func(self, &mut executor.borrow_mut());
    }

    // Components

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

    // Running

    /// Run the world, this is a "simple" run because you have not to customize the
    /// tick loop. The implementation clone the executor in order to avoid borrowing
    /// the world and then run the loop at 20 TPS. The loop than call the executor
    /// tick method and then clear events of the event tracker.
    ///
    /// This method is not made for you if you need to run multiple worlds with the
    /// same executor.
    pub fn simple_run(&mut self) {
        let executor = self.clone_executor();
        tick_loop(move |_info| {
            executor.borrow_mut().tick(self);
            self.event_tracker.clear_events();
            false
        }, 20.0);
    }

}
