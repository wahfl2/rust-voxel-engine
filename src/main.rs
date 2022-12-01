use main_loop::MainLoop;
use nalgebra::Point3;

pub mod main_loop;
pub mod render;
pub mod event;

fn main() {
    let main_loop = MainLoop::new();
    pollster::block_on(main_loop.run());
}
