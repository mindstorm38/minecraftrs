use std::any::{TypeId, Any};
use std::ops::{Deref, DerefMut};


struct Event {
    event: Box<dyn Any>,
    tid: TypeId,
    invalid: bool
}


pub struct EventTracker {
    events: Vec<Event>
}

impl EventTracker {

    pub fn new() -> Self {
        Self {
            events: Vec::new()
        }
    }

    pub fn push_event<T: Any>(&mut self, event: T) {
        self.events.push(Event {
            event: Box::new(event),
            tid: TypeId::of::<T>(),
            invalid: false
        })
    }

    /// Return true if the tracker has any pending event of the given type `T`.
    pub fn has_event<T: Any>(&self) -> bool {
        self.events.iter().any(|event| !event.invalid && event.tid == TypeId::of::<T>())
    }

    /// Simple iteration over all events of the same type.
    pub fn poll_events<'a, T: Any>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.events.iter()
            .filter(|event| !event.invalid && event.tid == TypeId::of::<T>())
            .map(|event| {
                // SAFETY: We can unwrap because the TypeId is checked in the filter.
                event.event.downcast_ref().unwrap()
            })
    }

    /// Simple iteration over all events handles of the same type, this allows
    /// mutating events and cancelling them.
    pub fn poll_events_handles<'a, T: Any>(&'a mut self) -> impl Iterator<Item = EventHandle<'a, T>> + 'a {
        self.events.iter_mut()
            .filter(|event| !event.invalid && event.tid == TypeId::of::<T>())
            .map(|event| {
                // SAFETY: We can unwrap because the TypeId is checked in the filter.
                EventHandle {
                    event: event.event.downcast_mut().unwrap(),
                    invalid: &mut event.invalid
                }
            })
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

}


pub struct EventHandle<'a, T> {
    event: &'a mut T,
    invalid: &'a mut bool
}

impl<T> EventHandle<'_, T> {

    /// Cancel this event for subsequent iterations.
    pub fn cancel(&mut self) {
        *self.invalid = true;
    }

}

impl<T> Deref for EventHandle<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

impl<T> DerefMut for EventHandle<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.event
    }
}
