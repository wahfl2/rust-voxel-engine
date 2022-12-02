use std::any::Any;

pub struct Event<T> {
    pub data: T,
}

impl<T> Event<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}