use std::any::{TypeId, Any};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::collections::hash_map::Entry;


struct Event {
    event: Box<dyn Any>,
    // tid: TypeId,
    invalid: bool
}


pub struct EventTracker {
    /// Currently used events.
    events: HashMap<TypeId, Vec<Event>>,
    /// All empty events vectors that can be used in the `events` map.
    empty_vec_pool: Vec<Vec<Event>>,
    /// Only used when returning empty iterators.
    empty_events: Vec<Event>
}

impl EventTracker {

    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            empty_vec_pool: Vec::new(),
            empty_events: Vec::new()
        }
    }

    pub fn push_event<T: Any>(&mut self, event: T) {
        match self.events.entry(TypeId::of::<T>()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                v.insert(self.empty_vec_pool.pop().unwrap_or_else(|| Vec::new()))
            }
        }.push(Event {
            event: Box::new(event),
            invalid: false
        })
    }

    /// Return true if the tracker has any pending event of the given type `T`.
    pub fn has_event<T: Any>(&self) -> bool {
        return self.events.contains_key(&TypeId::of::<T>())
        // self.events.iter().any(|event| !event.invalid && event.tid == TypeId::of::<T>())
    }

    /// Simple iteration over all events of the same type.
    pub fn poll_events<'a, T: Any>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        match self.events.get(&TypeId::of::<T>()) {
            Some(events) => events.iter(),
            None => self.empty_events.iter()
        }.filter(|event| !event.invalid)
            .map(|event| {
                // SAFETY: We can unwrap because the TypeId is checked in the filter.
                event.event.downcast_ref().unwrap()
            })
    }

    /// Simple iteration over all events handles of the same type, this allows
    /// mutating events and cancelling them.
    pub fn poll_events_handles<'a, T: Any>(&'a mut self) -> impl Iterator<Item = EventHandle<'a, T>> + 'a {
        match self.events.get_mut(&TypeId::of::<T>()) {
            Some(events) => events.iter_mut(),
            None => self.empty_events.iter_mut()
        }.map(|event| {
            // SAFETY: We can unwrap because the TypeId is checked in the filter.
            EventHandle {
                event: event.event.downcast_mut().unwrap(),
                invalid: &mut event.invalid
            }
        })
    }

    pub fn clear_events(&mut self) {
        self.empty_vec_pool.extend(self.events.drain().map(|(_, mut vec)| {
            vec.clear();
            vec
        }));
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
