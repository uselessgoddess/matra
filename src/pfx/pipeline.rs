use bevy::{
  asset::AssetPath,
  core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  prelude::{FromWorld, *},
  render::{
    render_graph::NodeRunError,
    render_resource::{
      binding_types::{sampler, texture_2d},
      *,
    },
    renderer::RenderDevice,
  },
};

macro_rules! define_config {
  ($name:ident = $shader:literal) => {
    pub struct $name;

    impl Config for $name {
      fn shader_asset() -> bevy::asset::AssetPath<'static> {
        $shader.into()
      }
    }
  };
}

pub(crate) use define_config;

pub trait Config {
  fn shader_asset() -> AssetPath<'static>;
}

pub trait Payload: Send + Sync + 'static {
  type Query;
  type Config: Config;

  fn layout() -> Vec<BindGroupLayoutEntryBuilder>; // #1

  fn store(render_device: &RenderDevice) -> Self; // #2

  fn bind(
    &self,
    world: &World,
    query: &Self::Query,
  ) -> Option<Vec<OwnedBindingResource>>; // #3
}

#[derive(Resource)]
pub struct PostFxPipeline<P: Payload> {
  pub main: BindGroupLayout,
  pub user: BindGroupLayout,
  pub pipeline_id: CachedRenderPipelineId,
  //
  pub screen_sampler: Sampler,
  pub payload: P,
}

impl<P: Payload> FromWorld for PostFxPipeline<P> {
  fn from_world(world: &mut World) -> Self {
    fn sequential_layout(
      visibility: ShaderStages,
      entries: Vec<BindGroupLayoutEntryBuilder>,
    ) -> Vec<BindGroupLayoutEntry> {
      entries
        .into_iter()
        .enumerate()
        .map(|(binding, entry)| entry.build(binding as u32, visibility))
        .collect()
    }

    let render_device = world.resource::<RenderDevice>();

    let entries = vec![
      texture_2d(TextureSampleType::Float { filterable: true }),
      sampler(SamplerBindingType::Filtering),
    ];
    let main = render_device.create_bind_group_layout(
      "pfx-main-group-layout",
      &sequential_layout(ShaderStages::FRAGMENT, entries),
    );

    let user = render_device.create_bind_group_layout(
      "pfx-main-group-layout",
      &sequential_layout(ShaderStages::FRAGMENT, P::layout()),
    );

    let screen_sampler =
      render_device.create_sampler(&SamplerDescriptor::default());

    let payload = P::store(&render_device);
    let shader = world.load_asset(P::Config::shader_asset());

    let pipeline_id = world
      .resource_mut::<PipelineCache>()
      .queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("pfx-pipeline".into()),
        layout: vec![main.clone(), user.clone()],
        vertex: fullscreen_shader_vertex_state(),
        fragment: Some(FragmentState {
          shader,
          shader_defs: vec![],
          entry_point: "fragment".into(),
          targets: vec![Some(ColorTargetState {
            format: TextureFormat::Rgba16Float,
            blend: None,
            write_mask: ColorWrites::ALL,
          })],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        push_constant_ranges: vec![],
        zero_initialize_workgroup_memory: false,
      });

    Self { main, user, pipeline_id, screen_sampler, payload }
  }
}
