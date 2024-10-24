pub use winit::application::ApplicationHandler;
pub use winit::event::WindowEvent;
pub use winit::event_loop::{ActiveEventLoop, ControlFlow};
pub use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    pub window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window {:?} has received the signal to quit.", window_id);
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            _=> (),
        }
    }
}