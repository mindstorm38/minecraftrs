use std::cell::{Ref, RefMut, RefCell};
use std::any::Any;
use std::rc::Rc;

use mc_core::world::level::Level;

use crate::util::{Components, ComponentError, SystemExecutor, EventTracker, tick_loop};


/// Type alias for a `SystemExecutor` that take a `World` context.
pub type WorldSystemExecutor = SystemExecutor<World>;

/// The world executor, containing a world with its executor.
pub struct WorldContext {
    /// The executor of the internal world.
    pub executor: WorldSystemExecutor,
    /// The internal world run by the executor.
    pub world: World
}

impl WorldContext {

    pub fn new() -> Self {
        Self {
            executor: SystemExecutor::new(),
            world: World::new()
        }
    }

    /// Call the given function with a mutable reference to the world
    /// and the executor as parameters.
    pub fn register<F>(&mut self, func: F)
    where
        F: FnOnce(&mut World, &mut WorldSystemExecutor)
    {
        (func)(&mut self.world, &mut self.executor);
    }

    pub fn run_simple(&mut self) {
        let executor = &mut self.executor;
        let world = &mut self.world;
        world.running = true;
        tick_loop(move |_info| {
            executor.tick(world);
            world.event_tracker.clear_events();
            world.running
        }, 20.0);
    }

}

/// A runtime Minecraft world, containing components, event tracker and levels.
pub struct World {
    /// A boolean set to true while running (likely from an system executor loop),
    /// it can be set to false to stop the loop.
    pub running: bool,
    /// Internal components registered before starting and used after by systems.
    pub components: Components,
    /// Internal event tracker for the world.
    pub event_tracker: EventTracker,
    /// World's levels.
    pub levels: Vec<Rc<RefCell<Level>>>
}

impl World {

    pub fn new() -> Self {
        Self {
            running: false,
            components: Components::new(),
            event_tracker: EventTracker::new(),
            levels: Vec::new()
        }
    }

    // Levels

    pub fn add_level(&mut self, level: Level) {
        self.levels.push(Rc::new(RefCell::new(level)));
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
    pub fn get_component<T: Any>(&self) -> Result<Ref<'_, T>, ComponentError> {
        self.components.get::<T>()
    }

    #[inline]
    pub fn get_component_mut<T: Any>(&self) -> Result<RefMut<'_, T>, ComponentError> {
        self.components.get_mut::<T>()
    }

}
