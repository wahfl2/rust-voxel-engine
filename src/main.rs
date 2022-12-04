use main_loop::MainLoop;
pub mod main_loop;
pub mod render;
pub mod event;
pub mod input;

fn main() {
    let main_loop = MainLoop::new();
    pollster::block_on(main_loop.run());
}
