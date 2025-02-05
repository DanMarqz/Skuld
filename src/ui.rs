use winit::{
    // application::Application,
    event::{Event, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId, WindowAttributes},
};

struct App {
    window: Window,
}

impl App {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Failed to create window");

        Self { window }
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            std::process::exit(0);
        }
    }
}

pub fn run_window() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app)
}
