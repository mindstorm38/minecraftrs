use std::any::{Any, TypeId};

use mc_core::world::level::Level;
use hecs::Entity;

use crate::world::World;


/// An AI component to implement for entities that have an AI.
#[derive(Debug)]
pub struct AiEntity {
    goal_selector_index: usize,
    target_selector_index: usize
}

impl AiEntity {

    pub fn new() -> Self {
        Self {
            goal_selector_index: 0,
            target_selector_index: 0
        }
    }

}

/// This is a world component where all goal selectors are stored, they
/// are later referenced by `AiEntity`.
pub struct AiState {
    goal_selectors: Vec<GoalSelector>
}

impl AiState {

    pub fn new() -> Self {
        Self {
            // Add a default empty selector.
            goal_selectors: vec![GoalSelector::new()]
        }
    }

    #[inline]
    pub fn get_selector_mut(&mut self, index: usize) -> Option<&mut GoalSelector> {
        self.goal_selectors.get_mut(index)
    }

}

pub struct GoalSelector {
    available_goals: Vec<RuntimeGoal>
}

pub struct RuntimeGoal {
    goal: Box<dyn Goal>,
    type_id: TypeId,
    priority: u16,
    running: bool
}

impl GoalSelector {

    pub fn new() -> Self {
        Self {
            available_goals: Vec::new()
        }
    }

    pub fn add_goal<G: Goal>(&mut self, priority: u16, goal: G) {
        self.available_goals.push(RuntimeGoal {
            goal: Box::new(goal),
            type_id: TypeId::of::<G>(),
            priority,
            running: false
        });
    }

    pub fn remove_goal<G: Goal>(&mut self) {
        self.available_goals.retain(|goal| {
            goal.type_id != TypeId::of::<G>()
        });
    }

}

pub trait Goal: Any {

    fn can_use(&self, level: &mut Level, entity: Entity) -> bool;

    fn can_continue_to_use(&self, level: &mut Level, entity: Entity) -> bool {
        self.can_use(level, entity)
    }

    fn start(&self, level: &mut Level, entity: Entity) {}
    fn stop(&self, level: &mut Level, entity: Entity) {}
    fn tick(&self, level: &mut Level, entity: Entity) {}

}


/// Call this function with a mutable reference to a World to register the `AiState`
/// component which is required to run the system `system_entity_ai`.
pub fn register_ai(world: &mut World) {
    world.insert_component(AiState::new());
}

/// A system that runs every entity implementing the component `AiEntity`.
pub fn system_entity_ai(world: &mut World) {

    /*let mut ai_state = world.get_component_mut::<AiState>()
        .expect("You must register an AiEntity component before.");
    FIXME
    for level in &world.levels {

        let mut level = level.borrow_mut();
        let entities = level.get_entities_mut();

        let query = entities.ecs.query_mut::<(&mut VanillaEntity, &mut AiEntity)>();

        for (entity, ai_entity) in query {

            let ai_entity: &mut AiEntity = ai_entity;

            if let Some(goal_selector) = ai_state.get_selector_mut(ai_entity.goal_selector_index) {

            }

        }

    }*/

}
