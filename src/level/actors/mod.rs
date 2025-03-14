mod camera;
mod lights;
mod player;

pub use {
  camera::PrimaryCamera,
  player::{CameraLook, Player, PlayerCamera},
};

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins((player::plugin, camera::plugin, lights::plugin));
}
