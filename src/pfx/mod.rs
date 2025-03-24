mod node;
mod pipeline;
mod settings;

use bevy::{
  core_pipeline::core_3d::graph::{Core3d, Node3d},
  prelude::*,
  render::{
    RenderApp,
    extract_component::ExtractComponentPlugin,
    render_asset::RenderAssets,
    render_graph::{RenderGraphApp, RenderLabel, ViewNodeRunner},
    render_resource::{
      binding_types::{sampler, texture_2d},
      *,
    },
    renderer::RenderDevice,
    texture::GpuImage,
  },
};

pub use {
  node::PostFxNode,
  pipeline::{Config, Payload, PostFxPipeline},
  settings::PostFxSettings,
};

use pipeline::define_config;

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

mod dither {
  use super::*;

  #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
  pub struct DitherLabel;

  define_config!(PayloadConfig = "shaders/pfx/pfx.wgsl");
  pub struct Payload {
    pub dither_sampler: Sampler,
  }

  impl super::Payload for Payload {
    type Query = PostFxSettings;
    type Config = PayloadConfig;

    fn layout() -> Vec<BindGroupLayoutEntryBuilder> {
      vec![
        texture_2d(TextureSampleType::Float { filterable: true }),
        sampler(SamplerBindingType::Filtering),
      ]
    }

    fn store(render_device: &RenderDevice) -> Self {
      let dither_sampler =
        render_device.create_sampler(&SamplerDescriptor::default());
      Self { dither_sampler }
    }

    fn bind<'a, 'q>(
      &'a self,
      world: &'a World,
      settings: &'q Self::Query,
    ) -> Option<Vec<BindingResource<'a>>> {
      let Some(dither) =
        world.resource::<RenderAssets<GpuImage>>().get(settings.handle().id())
      else {
        warn!("Failed to get threshold map, skipping...");
        return None;
      };

      Some(vec![
        dither.texture_view.into_binding(),
        self.dither_sampler.into_binding(),
      ])
    }
  }
}

mod clouds {
  use super::*;

  #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
  pub struct CloudsLabel;

  define_config!(PayloadConfig = "shaders/pfx/clouds.wgsl");
  pub struct Payload {}

  impl super::Payload for Payload {
    type Query = PostFxSettings;
    type Config = PayloadConfig;

    fn layout() -> Vec<BindGroupLayoutEntryBuilder> {
      vec![]
    }

    fn store(_render_device: &RenderDevice) -> Self {
      Self {}
    }

    fn bind<'a, 'q>(
      &'a self,
      _world: &'a World,
      _settings: &'q Self::Query,
    ) -> Option<Vec<BindingResource<'a>>> {
      Some(vec![])
    }
  }
}
