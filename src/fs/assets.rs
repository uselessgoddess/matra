use {
  crate::{config::GameConfig, prelude::*},
  common_assets::toml::TomlAssetPlugin,
};

pub fn plugin(app: &mut App) {
  app.add_plugins(config).add_loading_state(
    LoadingState::new(GameState::Loading)
      .continue_to_state(GameState::Playing)
      .with_dynamic_assets_file::<StandardDynamicAssetCollection>("assets.ron")
      .load_collection::<ConfigAssets>(),
  );
}

fn config(app: &mut App) {
  app
    .register_type::<GameConfig>()
    .init_resource::<GameConfig>()
    .add_plugins(TomlAssetPlugin::<GameConfig>::new(&["toml"]))
    .add_systems(Update, autoload_config);
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct ConfigAssets {
  #[asset(key = "config")]
  pub _config: Handle<GameConfig>,
}

fn autoload_config(
  mut commands: Commands,
  config: Res<Assets<GameConfig>>,
  mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
) {
  for event in config_asset_events.read() {
    if let AssetEvent::LoadedWithDependencies { id }
    | AssetEvent::Modified { id } = event
    {
      commands.insert_resource(config.get(*id).unwrap().clone());
    }
  }
}
