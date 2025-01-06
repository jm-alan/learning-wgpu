use wgpu::{
  Color, CommandEncoder, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
  RenderPassDescriptor, StoreOp, TextureView, TextureViewDescriptor,
};

use crate::app::App;

impl App<'_> {
  pub(crate) fn on_redraw(&self) {
    let Some(ref surface) = self.surface else {
      unreachable!();
    };
    let Some((ref device, ref queue)) = self.device else {
      unreachable!();
    };
    let Ok(context) = surface.get_current_texture() else {
      panic!("Failed to get surface texture on redraw request.");
    };

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
    let view = context
      .texture
      .create_view(&TextureViewDescriptor::default());

    self.perform_render_pass(&mut encoder, &view);
    queue.submit(Some(encoder.finish()));
    context.present();
  }

  fn perform_render_pass(&self, encoder: &mut CommandEncoder, view: &TextureView) {
    let Some(ref pipeline) = self.pipeline else {
      panic!("Attempted to conduct render pass with no pipeline.");
    };

    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
      label: None,
      color_attachments: &[Some(RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: Operations {
          load: LoadOp::Clear(Color::BLACK),
          store: StoreOp::Store,
        },
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
    });

    render_pass.set_pipeline(pipeline);
    render_pass.draw(0..3, 0..1);
  }
}
