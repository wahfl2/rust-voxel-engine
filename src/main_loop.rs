use std::{collections::VecDeque, time::Instant};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

use crate::render::render_state::RenderState;

pub struct MainLoop {
    pub window: Window,
    event_loop: EventLoop<()>,
    prev_frame_start: Instant,
    frame_times: Vec<f32>,
    pub fps: f32,
}

impl MainLoop {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        MainLoop { 
            window: WindowBuilder::new().build(&event_loop).unwrap(),
            event_loop,
            prev_frame_start: Instant::now(),
            frame_times: Vec::with_capacity(30),
            fps: 0.0,
        }
    }

    pub async fn run(mut self) {
        env_logger::init();
        self.prev_frame_start = Instant::now();

        let mut state = RenderState::new(&self.window).await;
    
        self.event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => if !state.input(event) { // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }

                    _ => {}
                }
            },

            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            },

            Event::MainEventsCleared => {
                self.frame_times.push(Instant::now().duration_since(self.prev_frame_start).as_secs_f32());
                if self.frame_times.len() >= 30 {
                    let mut sum = 0.0;
                    for frame_time in self.frame_times.iter() {
                        sum += frame_time;
                    }
                    self.fps = 1.0 / (sum / (self.frame_times.len() as f32));
                    self.frame_times.clear();
                    println!("Avg. fps: {:.2}", self.fps);
                }
                self.prev_frame_start = Instant::now();

                self.window.request_redraw();
            }
            _ => {}
        });
    }
}