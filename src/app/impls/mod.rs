use std::sync::Arc;

use pollster::block_on;
use wgpu::{
  include_wgsl, DeviceDescriptor, Features, FragmentState, Limits, MemoryHints, MultisampleState,
  PipelineLayoutDescriptor, PowerPreference, PrimitiveState, RenderPipelineDescriptor,
  RequestAdapterOptions, VertexState,
};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window};

use super::App;

mod handler;
mod redraw;

impl App<'_> {
  pub fn new() -> Self {
    Self::default()
  }

  fn init_window(&mut self, eloop: &ActiveEventLoop) {
    match eloop.create_window(Window::default_attributes()) {
      Ok(window) => self.window = Some(Arc::new(window)),
      Err(err) => {
        panic!("The operating system refused window creation; {err}");
      },
    };
  }

  fn init(&mut self, eloop: &ActiveEventLoop) {
    self.init_window(eloop);
    self.init_surface();
    self.init_adapter();
    self.init_device();
    self.init_config();
    self.init_pipeline();
    self.configure_surface();
    self.ready = true;
  }

  fn init_surface(&mut self) {
    let Some(window) = self.window.clone() else {
      panic!("Attempted to initialize surface with no window");
    };

    match self.instance.create_surface(window) {
      Ok(surface) => self.surface = Some(surface),
      Err(err) => panic!("Failed to create surface from window; {err}"),
    };
  }

  fn init_adapter(&mut self) {
    let Some(ref surface) = self.surface else {
      panic!("Attempted to initialize adapter with no surface.");
    };

    let adapter_future = self.instance.request_adapter(&RequestAdapterOptions {
      power_preference: PowerPreference::HighPerformance,
      force_fallback_adapter: false,
      compatible_surface: Some(surface),
    });

    self.adapter = block_on(adapter_future);
  }

  fn init_device(&mut self) {
    let Some(ref adapter) = self.adapter else {
      panic!("Init device called with no adapter");
    };

    let device_future = adapter.request_device(
      &DeviceDescriptor {
        label: None,
        required_features: Features::empty(),
        required_limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
        memory_hints: MemoryHints::MemoryUsage,
      },
      None,
    );

    self.device = block_on(device_future).ok()
  }

  fn init_config(&mut self) {
    let PhysicalSize { width, height } = self.window.clone().unwrap().inner_size();
    let Some(ref surface) = self.surface else {
      unreachable!();
    };
    let Some(ref adapter) = self.adapter else {
      unreachable!()
    };
    self.surface_config = surface.get_default_config(adapter, width, height);
  }

  fn init_pipeline(&mut self) {
    let Some((ref device, _)) = self.device else {
      panic!("Attempted to retrieve device when one did not exist.");
    };

    let compiled_shader =
      device.create_shader_module(include_wgsl!("../../../shaders/triangle.wgsl"));
    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor::default());

    let Some(ref adapter) = self.adapter else {
      panic!("Attempted to retrieve adapter when one did not exist.");
    };
    let Some(ref surface) = self.surface else {
      panic!("Attempted to retrieve surface when one did not exist.");
    };

    let surface_capabilities = surface.get_capabilities(adapter);
    let preferred_format = surface_capabilities.formats[0];

    let vertex = VertexState {
      module: &compiled_shader,
      entry_point: Some("vert"),
      buffers: &[],
      compilation_options: Default::default(),
    };

    let fragment = FragmentState {
      module: &compiled_shader,
      entry_point: Some("frag"),
      targets: &[Some(preferred_format.into())],
      compilation_options: Default::default(),
    };

    self.pipeline = Some(device.create_render_pipeline(&RenderPipelineDescriptor {
      label: None,
      layout: Some(&layout),
      vertex,
      fragment: Some(fragment),
      primitive: PrimitiveState::default(),
      multisample: MultisampleState::default(),
      depth_stencil: None,
      multiview: None,
      cache: None,
    }));
  }

  fn configure_surface(&self) {
    let Some(ref surface) = self.surface else {
      panic!("Attempted to configure a nonexistent surface");
    };
    let Some((ref device, _)) = self.device else {
      panic!("Attempted to configure a surface with no available device");
    };
    let Some(ref config) = self.surface_config else {
      panic!("Attempted to apply a nonexistent config to a surface");
    };

    surface.configure(device, config);
  }
}
