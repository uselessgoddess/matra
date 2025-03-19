use crate::{GameState, core::noise::perlin_1d, prelude::*};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct NoisedLight {
  factor: f32,
  speed: f32,
}

pub fn plugin(app: &mut App) {
  app
    .register_type::<NoisedLight>()
    .add_systems(Update, (noised, spot).run_if(in_state(GameState::Playing)));
}

fn noised(mut query: Query<(&mut PointLight, &NoisedLight)>, time: Res<Time>) {
  for (mut point, noised) in query.iter_mut() {
    point.intensity +=
      noised.factor * noised.speed * (perlin_1d(time.elapsed_secs()) - 0.5);
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
