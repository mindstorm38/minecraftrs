//! A tiny specific Entity Component System (ECS) for Minecraft entity ecosystem.

use once_cell::sync::Lazy;
use hecs::EntityBuilder;
use uuid::Uuid;


/// A type for static definition of entity types.
pub struct EntityType {
    pub name: &'static str,
    pub builder: fn(&mut EntityBuilder)
}


#[macro_export]
macro_rules! entities {
    ($global_vis:vis $static_id:ident $namespace:literal [
        $($entity_id:ident $entity_name:literal [$($comp_construct:ty),*]),*
        $(,)?
    ]) => {

        $($global_vis static $entity_id: $crate::entity::EntityType = $crate::entity::EntityType {
            name: concat!($namespace, ':', $entity_name),
            builder: |builder| {
                $(builder.add(<$comp_construct>::default());)*
            }
        };)*

        $global_vis static $static_id: [&'static $crate::entity::EntityType; $crate::count!($($entity_id)*)] = [
            $(&$entity_id),*
        ];

    };
}
