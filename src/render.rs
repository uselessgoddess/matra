use bevy::render::view::RenderLayers;

pub struct Layers;

impl Layers {
  const WORLD: RenderLayers = RenderLayers::layer(0);
  const PORTAL: RenderLayers = RenderLayers::layer(1);

  pub fn all() -> RenderLayers {
    Self::WORLD | Self::PORTAL
  }
}
