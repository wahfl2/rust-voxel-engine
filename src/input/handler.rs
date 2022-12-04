use rustc_hash::FxHashMap;
use winit::{event::{VirtualKeyCode, WindowEvent, ElementState}, event_loop::EventLoopProxy};

use crate::event::events::{Events, ButtonEventState, ButtonInputEvent};

#[derive(Clone, Copy, Debug)]
pub enum Movement {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ButtonState {
    Pressed,
    Released,
}

pub struct InputHandler {
    pub key_states: FxHashMap<VirtualKeyCode, ButtonState>,
    pub movement_input: FxHashMap<VirtualKeyCode, Movement>,
}

impl InputHandler {
    pub fn process_event(&mut self, proxy: &mut EventLoopProxy<Events>, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    let insert_state = match input.state {
                        ElementState::Pressed => ButtonState::Pressed,
                        ElementState::Released => ButtonState::Released,
                    };

                    let mut prev_state = ButtonState::Released;
                    if let Some(state) = self.key_states.get_mut(&key) {
                        prev_state = *state;
                        *state = insert_state
                    } else {
                        self.key_states.insert(key, insert_state);
                    }

                    if prev_state != insert_state {
                        proxy.send_event(Events::ButtonInput(ButtonInputEvent {
                            key,
                            state: match insert_state {
                                ButtonState::Pressed => ButtonEventState::JustPressed,
                                ButtonState::Released => ButtonEventState::JustReleased,
                            }
                        })).unwrap();
                    }
                }
            },
            _ => ()
        }
    }

    pub fn pressed(&self, key: &VirtualKeyCode) -> bool {
        if let Some(state) = self.key_states.get(key) {
            match state {
                ButtonState::Pressed => true,
                ButtonState::Released => false,
            }
        } else {
            false
        }
    }

    pub fn process_input(&self, proxy: &mut EventLoopProxy<Events>) {
        for (key, dir) in self.movement_input.iter() {
            if self.pressed(key) {
                proxy.send_event(Events::Movement(*dir)).unwrap();
            }
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

        Self { 
            key_states: FxHashMap::default(),
            movement_input: hash 
        }
    }
}