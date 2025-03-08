use {
  crate::{GameState, pfx::PostFxSettings, prelude::*, render},
  bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceReflections, VolumetricFog},
  },
};

#[derive(Component, Default)]
pub struct PrimaryCamera;

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

pub fn spawn(
  camera: Query<Entity, Added<PrimaryCamera>>,
  mut commands: Commands,
  assets: Res<AssetServer>,
) {
  for entity in camera.iter() {
    commands
      .entity(entity)
      .insert(Projection::Perspective(PerspectiveProjection {
        fov: 75.0 / 180.0 * std::f32::consts::PI,
        near: 1e-8,
        ..default()
      }))
      .insert(render::Layers::all())
      .insert((
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        VolumetricFog { ..default() },
        ScreenSpaceReflections { ..default() },
        ScreenSpaceAmbientOcclusion { ..default() },
        AtmosphereCamera::default(),
      ))
      .insert(PostFxSettings::new(8, &assets));
  }
}
