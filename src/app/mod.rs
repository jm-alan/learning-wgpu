use std::{collections::VecDeque, sync::Arc};
use wgpu::{Adapter, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::{
  event::WindowEvent,
  window::{Window, WindowId},
};

mod impls;

#[derive(Default)]
pub struct App<'surface_lifetime> {
  active: bool,
  ready: bool,
  unready_events: VecDeque<(WindowId, WindowEvent)>,
  instance: Instance,
  window: Option<Arc<Window>>,
  surface: Option<Surface<'surface_lifetime>>,
  pub(crate) surface_config: Option<SurfaceConfiguration>,
  adapter: Option<Adapter>,
  device: Option<(Device, Queue)>,
  pipeline: Option<RenderPipeline>,
  resize_standby: Option<WindowEvent>,
}
