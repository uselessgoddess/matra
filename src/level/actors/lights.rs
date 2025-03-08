use crate::{GameState, prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

pub fn spawn(mut camera: Query<&mut SpotLight>) {
  for mut spot in camera.iter_mut() {
    spot.range = 1024.0;
  }
}
