use std::any::{Any, TypeId};

use rustc_hash::FxHashMap;

use super::event::Event;

pub struct EventBus {
    events: FxHashMap<TypeId, Vec<Event<Box<dyn Any>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { events: FxHashMap::default() }
    }

    pub fn push_event<T: Any>(&mut self, event_data: T) {
        let event_vec_op = self.events.get_mut(&event_data.type_id());
        if let Some(event_vec) = event_vec_op {
            event_vec.push(Event::new(Box::new(event_data)));
        } else {
            self.events.insert(
                event_data.type_id(), 
                vec![Event::new(Box::new(event_data))]
            );
        }
    }

    pub fn get_events_data<T: Any>(&self) -> Vec<&T> {
        let events_vec_op = self.events.get(&TypeId::of::<T>());
        if let Some(events_vec) = events_vec_op {
            events_vec.into_iter()
                .map(|t| {
                    t.data.downcast_ref::<T>().unwrap()
                }).collect()
        } else {
            Vec::new()
        }
    }

    pub fn clear_events_data<T: Any>(&mut self) {
        self.events.remove(&TypeId::of::<T>());
    }

    pub fn clear_all(&mut self) {
        self.events.clear();
    }
}