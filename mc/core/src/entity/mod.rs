//! A tiny specific Entity Component System (ECS) for Minecraft entity ecosystem.

use std::collections::HashMap;
use std::any::TypeId;

use once_cell::sync::Lazy;
use hecs::{EntityBuilder, EntityRef};
use nbt::CompoundTag;
use uuid::Uuid;


/// A type for static definition of entity types.
pub struct EntityType {
    pub name: &'static str,
    pub builder: fn(&mut EntityBuilder),
    pub components: &'static [&'static EntityComponent]
}

pub struct EntityComponent {
    /// A function called to encode an entity's component into a NBT compound tag.
    /// You should get the component from the given `EntityRef`.
    pub encode: fn(src: &EntityRef, dst: &mut CompoundTag),
    /// A function called to decode an entity's component from a NBT compound tag.
    /// If the tag does not provide required values, put default ones, and then add
    /// the component to the given `EntityBuilder`, if  the missing tags are critical,
    /// return an `Err` with a description of the error.
    pub decode: fn(src: &CompoundTag, dst: &mut EntityBuilder, palette: &GlobalEntities) -> Result<(), String>
}


pub struct GlobalEntities {
    name_to_entity_type: HashMap<&'static str, &'static EntityType>
}

impl GlobalEntities {

    pub fn new() -> Self {
        Self {
            name_to_entity_type: HashMap::new()
        }
    }

    /// A simple constructor to directly call `register_all` with given entity types slice.
    pub fn with_all(slice: &[&'static EntityType]) -> Self {
        let mut entities = Self::new();
        entities.register_all(slice);
        entities
    }

    /// Register a single entity type to this palette.
    pub fn register(&mut self, entity_type: &'static EntityType) {
        self.name_to_entity_type.insert(entity_type.name, entity_type);
    }

    /// An optimized way to call `register` multiple times for each given entity type.
    pub fn register_all(&mut self, slice: &[&'static EntityType]) {
        for &entity_type in slice {
            self.register(entity_type);
        }
    }

    /// Get an entity type from its name.
    pub fn get_entity_type(&self, name: &str) -> Option<&'static EntityType> {
        self.name_to_entity_type.get(name).cloned()
    }

    pub fn entity_types_count(&self) -> usize {
        self.name_to_entity_type.len()
    }

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
