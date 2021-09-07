//! A tiny specific Entity Component System (ECS) for Minecraft entity ecosystem.

use std::collections::HashMap;
use crate::util::OpaquePtr;

mod codec;
pub use codec::*;


/// A type for static definition of entity types.
pub struct EntityType {
    /// The namespaced name of this entity.
    pub name: &'static str,
    /// Default data components' codecs *(refer to `EntityCodec` doc)* specifications.
    pub codecs: &'static [&'static dyn EntityCodec]
}

/// A trait to implement to component structures that support a codec, this is used by `entities!`
/// macro.
pub trait EntityComponent {
    const CODEC: &'static dyn EntityCodec;
}


/// The global entities palette used in level environment.
pub struct GlobalEntities {
    name_to_entity_type: HashMap<&'static str, &'static EntityType>,
    entity_type_to_codecs: HashMap<OpaquePtr<EntityType>, Vec<&'static dyn EntityCodec>>
}

impl GlobalEntities {

    pub fn new() -> Self {
        Self {
            name_to_entity_type: HashMap::new(),
            entity_type_to_codecs: HashMap::new()
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

        let default_codecs = entity_type.codecs.iter()
            .copied()
            .collect();

        self.name_to_entity_type.insert(entity_type.name, entity_type);
        self.entity_type_to_codecs.insert(OpaquePtr::new(entity_type), default_codecs);

    }

    /// An optimized way to call `register` multiple times for each given entity type.
    pub fn register_all(&mut self, slice: &[&'static EntityType]) {
        self.name_to_entity_type.reserve(slice.len());
        self.entity_type_to_codecs.reserve(slice.len());
        for &entity_type in slice {
            self.register(entity_type);
        }
    }

    /// Get an entity type from its name.
    pub fn get_entity_type(&self, name: &str) -> Option<&'static EntityType> {
        self.name_to_entity_type.get(name).copied()
    }

    pub fn get_codecs(&self, entity_type: &'static EntityType) -> Option<&Vec<&'static dyn EntityCodec>> {
        self.entity_type_to_codecs.get(&OpaquePtr::new(entity_type))
    }

    pub fn get_entity_type_and_codecs(&self, name: &str) -> Option<(&'static EntityType, &Vec<&'static dyn EntityCodec>)> {
        match self.name_to_entity_type.get(name) {
            Some(&typ) => {
                // SAFETY: Unwrap should be safe because every registered
                //  entity type also adds a components vec.
                Some((typ, self.entity_type_to_codecs.get(&OpaquePtr::new(typ)).unwrap()))
            },
            None => None
        }
    }

    pub fn has_entity_type(&self, entity_type: &'static EntityType) -> bool {
        self.entity_type_to_codecs.contains_key(&OpaquePtr::new(entity_type))
    }

    pub fn entity_types_count(&self) -> usize {
        self.name_to_entity_type.len()
    }

}


#[macro_export]
macro_rules! entities {
    ($global_vis:vis $static_id:ident $namespace:literal [
        $($entity_id:ident $entity_name:literal [$($component_struct:ident),*]),*
        $(,)?
    ]) => {

        $($global_vis static $entity_id: $crate::entity::EntityType = $crate::entity::EntityType {
            name: concat!($namespace, ':', $entity_name),
            codecs: &[$(<$component_struct as $crate::entity::EntityComponent>::CODEC),*]
        };)*

        $global_vis static $static_id: [&'static $crate::entity::EntityType; $crate::count!($($entity_id)*)] = [
            $(&$entity_id),*
        ];

    };
}


/*
Check if it's doable
macro_rules! def_entity_component {
    (
        $(#[$met:meta])?
        #[codec($codec_struct_id:ident)]
        struct $struct_id:ident {
            $(
            #[]
            $field_vis:vis $field_id:ident: $field_type:ty
            ),*
            $(,)?
        }
    ) => {

        $(#[$met])?
        struct $struct_id {
            $($field_vis $field_id: $field_type),*
        }

        pub struct $codec_struct_id;
        impl EntityCodec for $codec_struct_id {

        }

    };
}
*/
