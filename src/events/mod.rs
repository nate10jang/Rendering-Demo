use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow
};

pub struct EventHandler {}

impl EventHandler {
    pub fn handle(event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::MainEventsCleared {} => {
                // Game loop here
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    }
}
