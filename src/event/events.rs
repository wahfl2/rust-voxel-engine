use winit::event::VirtualKeyCode;

use crate::input::handler::Movement;

#[derive(Debug, Clone)]
pub enum Events {
    Movement(Movement),
    ButtonInput(ButtonInputEvent),
}

#[derive(Debug, Clone)]
pub struct ButtonInputEvent {
    pub key: VirtualKeyCode,
    pub state: ButtonEventState,
}

#[derive(Debug, Clone)]
pub enum ButtonEventState {
    JustPressed,
    JustReleased,
}