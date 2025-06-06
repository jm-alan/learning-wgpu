mod app;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
  let Ok(eloop) = EventLoop::new() else {
    panic!("Failed to create event loop");
  };

  let Ok(_) = eloop.run_app(&mut App::new()) else {
    panic!("App shut down unexpectedly");
  };
}
