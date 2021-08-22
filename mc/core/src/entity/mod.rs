//! A tiny specific Entity Component System (ECS) for Minecraft entity ecosystem.

use uuid::Uuid;
use crate::ecs::{Ecs, EntityBuilder, Component, VecStorage};


/// A type for static definition of entity types.
pub struct EntityType {
    pub name: &'static str,
    pub imp: &'static (dyn EntityImpl + Sync)
}

impl EntityType {

    pub fn register(&self, ecs: &mut Ecs) {
        self.imp.register(ecs);
    }

    pub fn build<'a>(&'static self, ecs: &'a mut Ecs) -> EntityBuilder<'a> {
        self.imp.build(ecs.create_entity().with(Entity::new(self)))
    }

}

pub trait EntityImpl {
    fn register(&self, ecs: &mut Ecs);
    fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>;
}

/// A special entity impl that does nothing to the entity builder.
pub struct NoEntityImpl;
impl EntityImpl for NoEntityImpl{
    fn register(&self, _: &mut Ecs) { }
    fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a> { builder }
}


#[macro_export]
macro_rules! entities {
    ($global_vis:vis $static_id:ident $namespace:literal [
        $($entity_id:ident $entity_name:literal $imp_id:ident),*
        $(,)?
    ]) => {

        $($global_vis static $entity_id: $crate::entity::EntityType = $crate::entity::EntityType {
            name: concat!($namespace, ':', $entity_name),
            imp: &$imp_id
        };)*

        $global_vis static $static_id: [&'static $crate::entity::EntityType; $crate::count!($($entity_id)*)] = [
            $(&$entity_id),*
        ];

    };
}



// Entity //

pub struct Entity {
    etype: &'static EntityType,
    uuid: Uuid,
    x: f64,
    y: f64,
    z: f64,
    custom_name: Option<String>,
    custom_name_visible: bool,
    tags: Option<Vec<String>>
}

impl Component for Entity {
    type Storage = VecStorage<Self>;
}

impl Entity {
    pub fn new(etype: &'static EntityType) -> Self {
        Self {
            etype,
            uuid: Uuid::new_v4(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            custom_name: None,
            custom_name_visible: false,
            tags: None
        }
    }
}

// Motion Entity //

pub struct MotionEntity {
    dx: f64,
    dy: f64,
    dz: f64
}

impl Component for MotionEntity {
    type Storage = VecStorage<Self>;
}
