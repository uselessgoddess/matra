use bevy::{
  ecs::query::QueryItem,
  prelude::*,
  render::{
    render_asset::RenderAssets,
    render_graph::{NodeRunError, RenderGraphContext, ViewNode},
    render_resource::*,
    renderer::RenderContext,
    texture::GpuImage,
    view::ViewTarget,
  },
};

#[derive(Default)]
pub struct PostFxNode;

impl ViewNode for PostFxNode {
  type ViewQuery = (&'static ViewTarget, &'static super::PostFxSettings);

  fn run(
    &self,
    _graph: &mut RenderGraphContext,
    render_context: &mut RenderContext,
    (view_target, dither_settings): QueryItem<Self::ViewQuery>,
    world: &World,
  ) -> Result<(), NodeRunError> {
    let process_pipeline = world.resource::<super::PostFxPipeline>();

    let pipeline_cache = world.resource::<PipelineCache>();

    let Some(pipeline) =
      pipeline_cache.get_render_pipeline(process_pipeline.pipeline_id)
    else {
      return Ok(());
    };

    let post_process = view_target.post_process_write();

    let Some(dither) = world
      .resource::<RenderAssets<GpuImage>>()
      .get(dither_settings.handle().id())
    else {
      warn!("Failed to get threshold map, skipping...");
      return Ok(());
    };

    let bind_group = render_context.render_device().create_bind_group(
      "pfx-bind-group",
      &process_pipeline.layout,
      &BindGroupEntries::sequential((
        post_process.source,
        &process_pipeline.screen_sampler,
        &dither.texture_view,
        &process_pipeline.dither_sampler,
      )),
    );

    let mut render_pass =
      render_context.begin_tracked_render_pass(RenderPassDescriptor {
        label: Some("pfx-pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
          view: post_process.destination,
          ops: Operations::default(),
          resolve_target: None,
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
      });

    render_pass.set_render_pipeline(pipeline);
    render_pass.set_bind_group(0, &bind_group, &[]);
    render_pass.draw(0..3, 0..1);

    Ok(())
  }
}
