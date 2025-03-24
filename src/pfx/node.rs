use {
  bevy::{
    ecs::query::QueryItem,
    prelude::*,
    render::{
      render_graph::{NodeRunError, RenderGraphContext, ViewNode},
      render_resource::*,
      renderer::RenderContext,
      view::ViewTarget,
    },
  },
  std::marker::PhantomData,
};

use super::Payload;

pub struct PostFxNode<P: Payload> {
  _marker: PhantomData<fn() -> P>,
}

impl<P: Payload> Default for PostFxNode<P> {
  fn default() -> Self {
    Self { _marker: PhantomData }
  }
}

impl<P: Payload<Query: Component>> ViewNode for PostFxNode<P> {
  type ViewQuery = (&'static ViewTarget, &'static P::Query);

  fn run(
    &self,
    _graph: &mut RenderGraphContext,
    render_context: &mut RenderContext,
    (view_target, query): QueryItem<Self::ViewQuery>,
    world: &World,
  ) -> Result<(), NodeRunError> {
    fn sequential_layout(entries: Vec<BindingResource>) -> Vec<BindGroupEntry> {
      entries
        .into_iter()
        .enumerate()
        .map(|(binding, resource)| BindGroupEntry {
          binding: binding as u32,
          resource,
        })
        .collect()
    }

    let process_pipeline = world.resource::<super::PostFxPipeline<P>>();

    let pipeline_cache = world.resource::<PipelineCache>();

    let Some(pipeline) =
      pipeline_cache.get_render_pipeline(process_pipeline.pipeline_id)
    else {
      return Ok(());
    };

    let post_process = view_target.post_process_write();

    let mut entries = vec![
      post_process.source.into_binding(),
      process_pipeline.screen_sampler.into_binding(),
    ];
    if let Some(binding) = process_pipeline.payload.bind(world, query) {
      entries.extend(binding);
    } else {
      return Ok(());
    }

    let bind_group = render_context.render_device().create_bind_group(
      "pfx-bind-group",
      &process_pipeline.layout,
      &sequential_layout(entries),
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
