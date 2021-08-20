use std::cell::{RefCell, Ref, RefMut, BorrowError};
use std::any::{TypeId, Any, type_name};
use std::collections::HashMap;
use thiserror::Error;


/// This structure is just a detailed wrapper of a function, with a name used
/// for debugging purposes. A system is just a function that is called on each
/// tick by a `SystemExecutor`.
struct System<Ctx> {
    /// For debug purpose.
    name: &'static str,
    /// Function used to execute the system.
    func: Box<dyn FnMut(&mut Ctx)>,
}


/// This structure is used to run a given context with some registered systems.
/// Each system is a function that will be called on each tick with a mutable
/// reference to the context.
pub struct SystemExecutor<Ctx> {
    systems: Vec<System<Ctx>>
}

impl<Ctx> SystemExecutor<Ctx> {

    pub fn new() -> Self {
        Self {
            systems: Vec::new()
        }
    }

    pub fn tick(&mut self, ctx: &mut Ctx) {
        for system in &mut self.systems {
            (system.func)(ctx);
        }
    }

    pub fn add_named_system<SysFunc>(&mut self, name: &'static str, func: SysFunc)
    where
        SysFunc: FnMut(&mut Ctx) + 'static
    {
        self.systems.push(System {
            func: Box::new(func),
            name
        });
    }

    #[inline]
    pub fn add_system<SysFunc>(&mut self, func: SysFunc)
    where
        SysFunc: FnMut(&mut Ctx) + 'static
    {
        self.add_named_system(type_name::<SysFunc>(), func);
    }

    pub fn iter_system_names<'a>(&'a self) -> impl Iterator<Item = &'static str> + 'a {
        self.systems.iter().map(|sys| sys.name)
    }

}
