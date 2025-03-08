use bevy::{
  prelude::*,
  render::{
    extract_component::ExtractComponent,
    render_asset::RenderAssetUsages,
    render_resource::{
      Extent3d, TextureDimension, TextureFormat, TextureUsages,
    },
  },
};

#[derive(Component, ExtractComponent, Clone)]
pub struct PostFxSettings {
  dither: Handle<Image>,
}

impl PostFxSettings {
  pub fn new(level: u32, asset_server: &AssetServer) -> Self {
    let power = level + 1;
    let map_size: u32 = 1 << power;
    let mut buffer = Vec::<u8>::new();

    for row in 0..map_size {
      for col in 0..map_size {
        let a = row ^ col;
        // Interleave bits of `a` with bits of y coordinate in reverse order
        let mut result: u64 = 0;
        let mut bit = 0;
        let mut mask = power as i32 - 1;
        loop {
          if bit >= 2 * power {
            break;
          }
          result |= (((col >> mask) & 1) << bit) as u64;
          bit += 1;
          result |= (((a >> mask) & 1) << bit) as u64;
          bit += 1;
          mask -= 1;
        }
        let value = ((result as f32 / map_size.pow(2) as f32) * 255.0) as u8;
        buffer.push(value);
      }
    }

    let mut image = Image::new(
      Extent3d { width: map_size, height: map_size, depth_or_array_layers: 1 },
      TextureDimension::D2,
      buffer,
      TextureFormat::R8Unorm,
      RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage = TextureUsages::COPY_DST
      | TextureUsages::STORAGE_BINDING
      | TextureUsages::TEXTURE_BINDING;

    Self { dither: asset_server.add(image) }
  }

  pub fn handle(&self) -> Handle<Image> {
    self.dither.clone()
  }
}
