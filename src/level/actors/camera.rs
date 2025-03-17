use {
  crate::{
    GameState, level::actors::Player, pfx::PostFxSettings, prelude::*, render,
  },
  bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceReflections, VolumetricFog},
  },
};

#[derive(Component, Default)]
#[require(Camera3d)]
pub struct PrimaryCamera;

pub fn plugin(app: &mut App) {
  app.add_systems(
    Update,
    (spawn, Dolly::<PrimaryCamera>::update_active)
      .run_if(in_state(GameState::Playing)),
  );
}

pub fn spawn(
  camera: Query<Entity, Added<PrimaryCamera>>,
  mut commands: Commands,
  assets: Res<AssetServer>,
) {
  for entity in camera.iter() {
    let mut commands = commands.entity(entity);

    commands
      .insert(Projection::Perspective(PerspectiveProjection {
        fov: 75.0 / 180.0 * std::f32::consts::PI,
        near: 1e-8,
        ..default()
      }))
      // .insert(PostFxSettings::new(8, &assets))
      .insert(render::Layers::all());

    dolly(&mut commands);
    processing(&mut commands);
  }

  fn dolly(commands: &mut EntityCommands) {
    use dolly::prelude::*;

    commands.insert(
      Rig::builder()
        .with(Position::new(Vec3::new(0.0, Player::HEIGHT, 0.0)))
        .with(LookAt::new(Vec3::ZERO))
        .with(Smooth::new_position_rotation(1.0, 1.0))
        .build(),
    );
  }

  fn processing(commands: &mut EntityCommands) {
    commands.insert((
      Tonemapping::TonyMcMapface,
      Bloom::default(),
      VolumetricFog { ..default() },
      ScreenSpaceReflections { ..default() },
      ScreenSpaceAmbientOcclusion { ..default() },
      AtmosphereCamera::default(),
    ));
  }
}
