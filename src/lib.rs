use state::State;
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}};

mod state;
mod vertex;

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let window = winit::window::WindowBuilder::new()
        .with_fullscreen(None)
        .with_title("Jarvis")
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window).await;

    event_loop
        .run(move |event, target| {
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        state.resize(new_size);
                    }
                    WindowEvent::RedrawRequested => {
                        state.render();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();
}