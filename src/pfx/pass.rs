use bevy::{
  prelude::*,
  render::{
    render_asset::RenderAssets,
    render_graph::RenderLabel,
    render_resource::{
      binding_types::{sampler, texture_2d, uniform_buffer_sized},
      *,
    },
    renderer::RenderDevice,
    texture::GpuImage,
  },
};

use super::{Config, Payload, PostFxSettings, pipeline::define_config};

pub mod clouds;
pub mod dither;
