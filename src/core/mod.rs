use crate::prelude::*;

mod atmosphere;
mod dev;
mod hanabi;
mod physics;
mod system;

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(system::plugin)
      .add_plugins(physics::plugin)
      .add_plugins(atmosphere::plugin)
      .add_plugins(hanabi::plugin);

    if debug::dev() {
      app.add_plugins(dev::plugin);
    }
  }
}
