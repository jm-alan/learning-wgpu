use super::App;
use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::ActiveEventLoop,
  window::{Window, WindowId},
};

impl ApplicationHandler for App {
  fn resumed(&mut self, eloop: &ActiveEventLoop) {
    println!("Resumed!");

    if self.active && self.ready {
      return;
    }
    self.active = true;

    if self.ready {
      return;
    }

    match eloop.create_window(Window::default_attributes()) {
      Ok(window) => self.window = Some(window),
      Err(err) => {
        panic!("The operating system refused window creation; {err}");
      },
    }

    self.ready = true;
    while let Some((wid, event)) = self.unready_events.pop_front() {
      self.window_event(eloop, wid, event);
    }
  }

  fn window_event(&mut self, eloop: &ActiveEventLoop, wid: WindowId, event: WindowEvent) {
    if !self.ready {
      self.unready_events.push_back((wid, event));
      return;
    }

    match event {
      WindowEvent::CloseRequested | WindowEvent::Destroyed => eloop.exit(),
      _ => {},
    }
  }
}
