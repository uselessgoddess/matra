mod node;
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
  node::PostFxNode, pipeline::PostFxPipeline, settings::PostFxSettings,
};

pub struct PostFxPlugin;

impl Plugin for PostFxPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((ExtractComponentPlugin::<PostFxSettings>::default(),));

    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };

    render_app
      .add_render_graph_node::<ViewNodeRunner<PostFxNode>>(Core3d, PostFxLabel)
      .add_render_graph_edges(
        Core3d,
        (Node3d::EndMainPass, PostFxLabel, Node3d::Tonemapping),
      );
  }

  fn finish(&self, app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };

    render_app.init_resource::<PostFxPipeline>();
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostFxLabel;
