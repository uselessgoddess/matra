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

  fn bind(
    &self,
    world: &World,
    settings: &Self::Query,
  ) -> Option<Vec<OwnedBindingResource>> {
    let Some(dither) =
      world.resource::<RenderAssets<GpuImage>>().get(settings.handle().id())
    else {
      warn!("Failed to get threshold map, skipping...");
      return None;
    };

    Some(vec![
      OwnedBindingResource::TextureView(dither.texture_view.clone()),
      OwnedBindingResource::Sampler(self.dither_sampler.clone()),
    ])
  }
}
