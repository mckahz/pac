use winit::{dpi::PhysicalSize, event_loop::EventLoop};

pub struct Window {
    pub size: PhysicalSize<u32>,
    pub winit: winit::window::Window,
}

impl Window {
    pub fn new(width: u32, height: u32) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();
        let winit = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        let size = PhysicalSize { width, height };
        (
            Self {
                size,
                winit,
            },
            event_loop,
        )
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }
}
