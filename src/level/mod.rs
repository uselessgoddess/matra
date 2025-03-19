pub mod actors;
mod particles;
mod portals;
mod utils;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins((actors::plugin, utils::plugin, particles::plugin));
}
