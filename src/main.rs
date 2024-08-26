use render::State;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let builder = winit::window::WindowBuilder::new();
    let window = builder.with_title("Jarvis").build(&event_loop).unwrap();

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

pub fn main() {
    pollster::block_on(run());
}