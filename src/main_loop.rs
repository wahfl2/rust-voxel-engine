use std::time::Instant;

use nalgebra::{Vector3, Point3};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{WindowBuilder, Window},
};

use crate::{render::{render_state::RenderState, camera::CameraController}, input::handler::{InputHandler, Movement}, event::events::Events, game::static_data::StaticBlockData};

pub struct MainLoop {
    pub window: Window,
    event_loop: EventLoop<Events>,
    prev_frame_start: Instant,
    frame_times: Vec<f32>,
    pub fps: f32,
}

impl MainLoop {
    pub fn new() -> Self {
        let event_loop = EventLoopBuilder::<Events>::with_user_event().build();

        MainLoop { 
            window: WindowBuilder::new()
                .with_maximized(true)
                .build(&event_loop).unwrap(),
            event_loop,
            prev_frame_start: Instant::now(),
            frame_times: Vec::with_capacity(30),
            fps: 0.0,
        }
    }

    pub async fn run(mut self) {
        env_logger::init();
        self.prev_frame_start = Instant::now();

        // let static_data = StaticBlockData::

        let mut render_state = RenderState::new(&self.window).await;
        let mut input_handler = InputHandler::default();
        let mut proxy = self.event_loop.create_proxy();
        let mut camera_controller = CameraController::new(Point3::new(0.0, 0.0, -5.0), 1.0);
    
        self.event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => if !render_state.input(event) {
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
                        render_state.resize(*physical_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(**new_inner_size);
                    }

                    _ => {
                        input_handler.process_event(&mut proxy, event);
                    }
                }
            },

            Event::DeviceEvent { event, .. } => {
                camera_controller.process_device_event(event);
            }

            Event::UserEvent(event) => {
                match event {
                    Events::Movement(dir) => {
                        let move_speed = 0.1;
                        let mut sum = Vector3::zeros();
                        let look_vec = render_state.camera.transform.rotation.inverse() * Vector3::z();
                        let right_vec = look_vec.cross(&render_state.camera.up);

                        match dir {
                            Movement::Forward => {
                                sum += look_vec;
                            },
                            Movement::Backward => {
                                sum -= look_vec;
                            },
                            Movement::Left => {
                                sum -= right_vec;
                            },
                            Movement::Right => {
                                sum += right_vec;
                            },
                        }

                        camera_controller.position += sum * move_speed;
                    },
                    Events::ButtonInput(_input) => {
                        
                    }
                }
            }

            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                render_state.camera.transform = camera_controller.get_transform();
                input_handler.process_input(&mut proxy);
                render_state.update();

                match render_state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size),
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