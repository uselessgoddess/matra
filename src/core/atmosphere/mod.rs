mod clouds;

use {crate::prelude::*, bevy::render::render_resource::ShaderType};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(AtmospherePlugin)
    .add_plugins(clouds::CloudsPlugin)
    .insert_resource(AmbientLight::NONE);
}

pub use clouds::NishitaPlus;
