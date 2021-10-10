use std::sync::Arc;
use std::rc::Rc;

use crate::event::ChunkLoadedEvent;
use crate::world::World;


pub fn system_load_chunks(world: &mut World) {

    let event_tracker = &mut world.event_tracker;

    for level in &world.levels {
        level.borrow_mut().load_chunks_with_callback(|(cx, cz, res)| {
            match res {
                Ok(chunk) => {
                    event_tracker.push_event(ChunkLoadedEvent {
                        level: Rc::clone(&level),
                        chunk: Arc::clone(chunk),
                        cx,
                        cz
                    })
                },
                Err(_err) => {}
            }
        });
    }

}
