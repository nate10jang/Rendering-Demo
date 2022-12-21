use winit::{
    window::{WindowBuilder, Window as WinitWindow},
    event_loop::EventLoop
};

pub struct Window {
    _surface: WinitWindow,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Window {
        let surface: WinitWindow = WindowBuilder::new()
            .with_title("Graphics Engine")
            .build(event_loop)
            .unwrap();

        return Window {
            _surface: surface,
        };
    }
}