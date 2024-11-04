use app::{App, State};
use world::*;

mod app;
mod graphics;
mod world;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::builder().build()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut App::new(AppState::default()))?;
    Ok(())
}

#[derive(Default)]
pub struct AppState {}

impl State for AppState {
    fn initialize(&mut self, world: &mut world::World) {
        spawn_entities(world, ACTIVE_CAMERA | CAMERA | LOCAL_TRANSFORM | PLAYER, 1)[0];
    }

    fn receive_event(&mut self, _world: &mut world::World, _event: &winit::event::WindowEvent) {}

    fn update(&mut self, _world: &mut world::World) {}
}
