use super::*;

#[derive(ShaderType)]
pub struct Params {
  ray_origin: Vec3,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct CloudsLabel;

define_config!(PayloadConfig = "shaders/pfx/clouds.wgsl");
pub struct Payload {}

impl super::Payload for Payload {
  type Query = PostFxSettings;
  type Config = PayloadConfig;

  fn layout() -> Vec<BindGroupLayoutEntryBuilder> {
    vec![uniform_buffer_sized(false, Some(Params::min_size()))]
  }

  fn store(_render_device: &RenderDevice) -> Self {
    Self {}
  }

  fn bind(
    &self,
    world: &World,
    _settings: &Self::Query,
  ) -> Option<Vec<OwnedBindingResource>> {
    let render_device = world.get_resource::<RenderDevice>().unwrap();

    let mut buffer = encase::UniformBuffer::new(Vec::new());
    buffer.write(&Params { ray_origin: Vec3::new(1.0, 0.0, 0.5) }).unwrap();

    let resource = OwnedBindingResource::Buffer(
      render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        contents: buffer.as_ref(),
      }),
    );

    Some(vec![resource])
  }
}
