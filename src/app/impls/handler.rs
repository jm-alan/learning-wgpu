use winit::{
  application::ApplicationHandler,
  event::{DeviceEvent, DeviceId, WindowEvent},
  event_loop::ActiveEventLoop,
  window::WindowId,
};

use crate::app::App;

impl ApplicationHandler for App<'_> {
  fn resumed(&mut self, eloop: &ActiveEventLoop) {
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

  fn window_event(&mut self, eloop: &ActiveEventLoop, w_id: WindowId, event: WindowEvent) {
    if !self.ready {
      self.unready_events.push_back((w_id, event));
      return;
    }

    let mut was_resized = false;

    match event {
      WindowEvent::CloseRequested | WindowEvent::Destroyed => eloop.exit(),
      WindowEvent::CursorMoved { .. }
      | WindowEvent::TouchpadPressure { .. }
      | WindowEvent::MouseWheel { .. } => {},
      WindowEvent::RedrawRequested => {
        if let Some(_) = self.resize_standby {
          return;
        }
        self.on_redraw();
      },
      re @ WindowEvent::Resized(_) => {
        was_resized = true;
        self.resize_standby = Some(re);
      },
      _ => {
        println!("[_]: {event:?}");
      },
    }

    if was_resized {
      return;
    }

    if let Some(WindowEvent::Resized(ps)) = self.resize_standby {
      self.resize_standby = None;
      self.on_resize(ps);
    };
  }

  fn device_event(&mut self, eloop: &ActiveEventLoop, d_id: DeviceId, event: DeviceEvent) {
    match event {
      DeviceEvent::MouseMotion { .. } => {},
      _ => println!("_X_: {event:?}"),
    }
  }
}
