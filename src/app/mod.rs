use std::collections::VecDeque;
use winit::{
  event::WindowEvent,
  window::{Window, WindowId},
};

mod app_handler;

#[derive(Default)]
pub struct App {
  active: bool,
  ready: bool,
  window: Option<Window>,
  unready_events: VecDeque<(WindowId, WindowEvent)>,
}
