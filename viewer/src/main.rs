use std::sync::Arc;

use viewer::State;
use wgpu::SurfaceError;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};

fn main() {
    smol::block_on(run());
}

async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let window = Arc::new(window);

    let mut state = State::new(window).await;

    let _ = event_loop.run(move |event, target| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                target.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => state.input(&event),
            WindowEvent::RedrawRequested => match state.render() {
                Ok(_) => (),
                Err(SurfaceError::Lost) => state.resize(state.size),
                Err(SurfaceError::OutOfMemory) => target.exit(),
                Err(e) => eprintln!("{:?}", e),
            },
            _ => state.update(),
        },
        _ => (),
    });
}
