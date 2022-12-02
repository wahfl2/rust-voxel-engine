use std::collections::HashMap;

use rustc_hash::FxHashMap;
use winit::event::{VirtualKeyCode, WindowEvent};

use crate::event::event_bus::EventBus;

#[derive(Clone, Copy, Debug)]
pub enum Movement {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct InputHandler {
    pub movement_input: FxHashMap<VirtualKeyCode, Movement>,
}

impl InputHandler {
    pub fn process_events(&self, bus: &mut EventBus, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    if let Some(movement) = self.movement_input.get(&key) {
                        println!("pushed");
                        bus.push_event(movement.clone());
                    }
                }
            },
            _ => ()
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        let mut hash = FxHashMap::default();
        hash.insert(VirtualKeyCode::W, Movement::Forward);
        hash.insert(VirtualKeyCode::S, Movement::Backward);
        hash.insert(VirtualKeyCode::A, Movement::Left);
        hash.insert(VirtualKeyCode::D, Movement::Right);

        Self { movement_input: hash }
    }
}