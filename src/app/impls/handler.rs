use winit::{
  application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent,
  event_loop::ActiveEventLoop, window::WindowId,
};

use crate::app::App;

impl ApplicationHandler for App<'_> {
  fn resumed(&mut self, eloop: &ActiveEventLoop) {
    println!("Resumed!");

    if self.active && self.ready {
      return;
    }
    self.active = true;

    if self.ready {
      return;
    }

    self.init(eloop);

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
      WindowEvent::CursorMoved { .. }
      | WindowEvent::TouchpadPressure { .. }
      | WindowEvent::MouseWheel { .. } => {},
      WindowEvent::RedrawRequested => {
        self.on_redraw();
      },
      WindowEvent::Resized(PhysicalSize { width, height }) => {
        let Some(ref window) = self.window else {
          unreachable!();
        };
        let Some(ref mut config) = self.surface_config else {
          unreachable!();
        };

        config.width = width;
        config.height = height;

        self.configure_surface();
        window.request_redraw();
      },
      _ => println!("WEVENT: {event:?}"),
    }
  }
}
