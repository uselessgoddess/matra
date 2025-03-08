use bevy::{
  core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  prelude::{FromWorld, *},
  render::{
    render_resource::{
      binding_types::{sampler, texture_2d},
      *,
    },
    renderer::RenderDevice,
  },
};

#[derive(Resource)]
pub struct PostFxPipeline {
  pub layout: BindGroupLayout,
  pub pipeline_id: CachedRenderPipelineId,
  //
  pub screen_sampler: Sampler,
  pub dither_sampler: Sampler,
}

const SHADER_ASSET_PATH: &str = "shaders/pfx.wgsl";

impl FromWorld for PostFxPipeline {
  fn from_world(world: &mut World) -> Self {
    let render_device = world.resource::<RenderDevice>();

    let layout = render_device.create_bind_group_layout(
      "pfx-bind-group-layout",
      &BindGroupLayoutEntries::sequential(
        ShaderStages::FRAGMENT,
        (
          texture_2d(TextureSampleType::Float { filterable: true }),
          sampler(SamplerBindingType::Filtering),
          texture_2d(TextureSampleType::Float { filterable: true }),
          sampler(SamplerBindingType::Filtering),
        ),
      ),
    );

    let screen_sampler =
      render_device.create_sampler(&SamplerDescriptor::default());
    let dither_sampler =
      render_device.create_sampler(&SamplerDescriptor::default());

    let shader = world.load_asset(SHADER_ASSET_PATH);

    let pipeline_id = world
      .resource_mut::<PipelineCache>()
      .queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("pfx-pipeline".into()),
        layout: vec![layout.clone()],
        vertex: fullscreen_shader_vertex_state(),
        fragment: Some(FragmentState {
          shader,
          shader_defs: vec![],
          entry_point: "fragment".into(),
          targets: vec![Some(ColorTargetState {
            format: TextureFormat::Rgba8UnormSrgb,
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

    Self { layout, pipeline_id, screen_sampler, dither_sampler }
  }
}
