use bevy::prelude::*;

mod assets;

pub fn plugin(app: &mut App) {
  app.add_plugins(assets::plugin);
}
