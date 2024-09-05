
use state::State;
use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}};

mod state;
mod vertex_generator;
mod buffers;
mod setup;
mod index_generator;
mod colors;

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(400,1080))
        .with_title("Jarvis")
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window).await;

    event_loop
        .run(move |event, target| {
            match event {
                Event::AboutToWait => {
                    state.window().request_redraw();
                },
                Event::WindowEvent { window_id: _, event } 
                    => match event {
                        WindowEvent::Resized(new_size) => {
                            state.resize(new_size);
                        }
                        WindowEvent::RedrawRequested => {
                            state.render();
                        },
                        WindowEvent::KeyboardInput {
                            event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::KeyR),
                                ..
                            },
                        ..
                        } => {state.start_record();},
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => target.exit(),
                        _ => {}
                },
                _ => {}
            }
        })
        .unwrap();
}