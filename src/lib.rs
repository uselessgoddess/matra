#![feature(let_chains)]

mod core;
mod debug;
pub mod level;
pub mod pfx;
pub mod player;
mod render;

use {
  crate::{core::CorePlugin, pfx::PostFxPlugin},
  bevy::{core_pipeline::auto_exposure::AutoExposurePlugin, prelude::*},
  blenvy::BlenvyPlugin,
};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  #[default]
  Loading,
  Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(CorePlugin)
      .add_plugins(BlenvyPlugin::default())
      .init_state::<GameState>()
      .add_plugins((AutoExposurePlugin, PostFxPlugin))
      .add_plugins((level::plugin, player::plugin));
  }
}

pub mod prelude {
  pub use super::*;

  pub use {
    avian3d::prelude::*,
    bevy_asset_loader::prelude::*,
    bevy_atmosphere::prelude::*,
    bevy_easy_portals::{gizmos::PortalGizmosPlugin, *},
    bevy_spectator::*,
    bevy_tnua::prelude::*,
    bevy_tnua_avian3d::*,
    blenvy::*,
    hanabi::prelude::*,
    iyes_progress::prelude::*,
  };
}
