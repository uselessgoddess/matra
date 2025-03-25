mod node;
mod pass;
mod pipeline;
mod settings;

use bevy::{
  core_pipeline::core_3d::graph::{Core3d, Node3d},
  prelude::*,
  render::{
    RenderApp,
    extract_component::ExtractComponentPlugin,
    render_graph::{RenderGraphApp, RenderLabel, ViewNodeRunner},
  },
};

pub use {
  node::PostFxNode,
  pipeline::{Config, Payload, PostFxPipeline},
  settings::PostFxSettings,
};

use pass::{clouds, dither};

pub struct PostFxPlugin;

impl Plugin for PostFxPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((ExtractComponentPlugin::<PostFxSettings>::default(),));

    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };

    render_app
      .add_render_graph_node::<ViewNodeRunner<PostFxNode<clouds::Payload>>>(
        Core3d,
        clouds::CloudsLabel,
      )
      .add_render_graph_node::<ViewNodeRunner<PostFxNode<dither::Payload>>>(
        Core3d,
        dither::DitherLabel,
      )
      .add_render_graph_edges(
        Core3d,
        (
          Node3d::EndMainPass,
          clouds::CloudsLabel,
          dither::DitherLabel,
          Node3d::Tonemapping,
        ),
      );
  }

  fn finish(&self, app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };

    render_app.init_resource::<PostFxPipeline<clouds::Payload>>();
    render_app.init_resource::<PostFxPipeline<dither::Payload>>();
  }
}
