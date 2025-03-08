use bevy::{
  prelude::*,
  render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, Face, TextureDimension, TextureFormat},
  },
};

pub fn uv(darkness: u8) -> Image {
  const TEXTURE_SIZE: usize = 8;

  let mut palette: [u8; 32] = [
    255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102,
    255, 102, 255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102,
    255, 255,
  ];

  palette = palette.map(|c| c / darkness);

  let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
  for y in 0..TEXTURE_SIZE {
    let offset = TEXTURE_SIZE * y * 4;
    texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
    palette.rotate_right(4);
  }

  Image::new_fill(
    Extent3d {
      width: TEXTURE_SIZE as u32,
      height: TEXTURE_SIZE as u32,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    &texture_data,
    TextureFormat::Rgba8UnormSrgb,
    RenderAssetUsages::default(),
  )
}

pub fn material(
  images: &mut Assets<Image>,
  darkness: u8,
  cull_mode: Option<Face>,
) -> StandardMaterial {
  StandardMaterial {
    base_color_texture: Some(images.add(uv(darkness))),
    cull_mode,
    ..default()
  }
}
