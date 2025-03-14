use {
  crate::prelude::*,
  serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Default)]
//
#[derive(Asset, Reflect, Serialize, Deserialize, Resource)]
#[reflect(Resource, Debug, Default)]
pub struct GameConfig {
  pub camera: Camera,
}

#[derive(Debug, Clone, Default)]
//
#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct Camera {
  pub dolly: Dolly,
}

#[derive(Debug, Clone)]
//
#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct Dolly {
  pub steps: Vec2,
}

impl Default for Dolly {
  fn default() -> Self {
    Self { steps: Vec2::new(28.0, 14.0) }
  }
}
