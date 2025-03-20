use crate::{GameState, core::noise::perlin_1d, prelude::*};

#[derive(Component, Reflect)]
#[reflect(Component)]
// Hack to add component to the Light entry in blender
pub struct NoisedLightHack {
  intensity: f32,
  factor: f32,
  speed: f32,
}

pub fn plugin(app: &mut App) {
  app
    .register_type::<NoisedLightHack>()
    .add_systems(Update, (noised, spot).run_if(in_state(GameState::Playing)));
}

fn noised(
  mut q_child: Query<(&mut PointLight, &Parent)>,
  mut q_parent: Query<&NoisedLightHack>,
  time: Res<Time>,
) {
  for (mut light, parent) in q_child.iter_mut() {
    if let Ok(noised) = q_parent.get(parent.get()) {
      light.intensity = noised.intensity
        + noised.factor * (perlin_1d(time.elapsed_secs() * noised.speed) - 0.5);
    }
  }
}

fn spot(mut spot: Query<&mut SpotLight>, mut point: Query<&mut PointLight>) {
  for mut spot in spot.iter_mut() {
    spot.range = 1024.0;
  }

  for mut point in point.iter_mut() {
    point.range = 1024.0;
  }
}
