mod nishita;

use {crate::prelude::*, bevy::render::render_resource::ShaderType};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(AtmospherePlugin)
    .add_plugins(nishita::NishitaPlugin)
    .insert_resource(AmbientLight::NONE);
}

pub use nishita::NishitaPlus;
