
mod stack;
pub use stack::*;

pub mod behaviour;


pub struct Item {
    name: &'static str,
    stack_size: u16
}

impl Item {

    pub const fn new(name: &'static str, stack_size: u16) -> Self {
        Self { name, stack_size }
    }

    #[inline]
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn get_stack_size(&self) -> u16 {
        self.stack_size
    }

}


#[macro_export]
macro_rules! items {
    ($global_vis:vis $static_id:ident $namespace:literal [
        $($item_id:ident $item_name:literal $($stack_size:literal)?),*
        $(,)?
    ]) => {

        $($global_vis static $item_id: $crate::item::Item = $crate::item::Item::new(
            concat!($namespace, ':', $item_name),
            $crate::inner_items_stack_size!($($stack_size)?)
        );)*

        $global_vis static $static_id: [&'static $crate::item::Item; $crate::count!($($item_id)*)] = [
            $(&$item_id),*
        ];

    };
}

#[macro_export]
macro_rules! inner_items_stack_size {
    () => { 64 };
    ($stack_size:literal) => { $stack_size };
}
