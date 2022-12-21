use winit::{
    event::Event,
    event_loop::{EventLoop, ControlFlow},
};

use crate::events::EventHandler;
use crate::window::Window;

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        return Application {};
    }

    pub fn run(self) {
        let event_loop: EventLoop<()> = EventLoop::new();
        let _window: Window = Window::new(&event_loop);

        self.run_eventloop(event_loop);
    }

    fn run_eventloop(self, event_loop: EventLoop<()>) {
        event_loop.run(move |event: Event<()>, _, control_flow: &mut ControlFlow| {
            *control_flow = ControlFlow::Wait;

            EventHandler::handle(event, control_flow)
        });
    }
}
