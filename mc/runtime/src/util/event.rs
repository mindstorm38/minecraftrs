use std::any::{TypeId, Any};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::marker::PhantomData;
use std::fmt::Debug;

use std::slice::{Iter, IterMut};


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
    }

    /// Simple iteration over all events of the same type.
    pub fn poll_events<T: Any>(&self) -> EventIterator<T> {
        EventIterator {
            events_it: self.events.get(&TypeId::of::<T>()).map(|v| v.iter()),
            phantom: PhantomData
        }
    }

    /// Simple iteration over all events handles of the same type, this allows
    /// mutating events and cancelling them.
    pub fn poll_events_handles<'a, T: Any>(&mut self) -> EventHandleIterator<T> {
        EventHandleIterator {
            events_it: self.events.get_mut(&TypeId::of::<T>()).map(|v| v.iter_mut()),
            phantom: PhantomData
        }
    }

    /*/// Simple iteration over all events handles of the same type, this allows
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
    }*/

    pub fn clear_events(&mut self) {
        self.empty_vec_pool.extend(self.events.drain().map(|(_, mut vec)| {
            vec.clear();
            vec
        }));
    }

}


/// Iterator returned from `EventTracker::poll_events`.
pub struct EventIterator<'a, T> {
    events_it: Option<Iter<'a, Event>>,
    phantom: PhantomData<&'a T>
}

impl<'a, T: Any> Iterator for EventIterator<'a, T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.events_it {
            None => None,
            Some(ref mut it) => {
                loop {
                    match it.next() {
                        Some(event) => if !event.invalid {
                            // SAFETY: We can unwrap because TypeId is already checked.
                            break Some(event.event.downcast_ref::<T>().unwrap())
                        },
                        None => break None
                    }
                }
            }
        }
    }

}


/// Iterator returned from `EventTracker::poll_events_handles`.
pub struct EventHandleIterator<'a, T> {
    events_it: Option<IterMut<'a, Event>>,
    phantom: PhantomData<EventHandle<'a, T>>
}

impl<'a, T: Any> Iterator for EventHandleIterator<'a, T> {

    type Item = EventHandle<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.events_it {
            None => None,
            Some(ref mut it) => {
                loop {
                    match it.next() {
                        Some(event) => if !event.invalid {
                            // SAFETY: We can unwrap because TypeId is already checked.
                            break Some(EventHandle {
                                event: event.event.downcast_mut::<T>().unwrap(),
                                invalid: &mut event.invalid
                            })
                        },
                        None => break None
                    }
                }
            }
        }
    }

}


/// An event handle is a wrapper for an event (with deref impl) that allows you to mutate it
/// and also to cancel it for further poll calls.
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
