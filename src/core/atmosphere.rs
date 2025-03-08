use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins(AtmospherePlugin).insert_resource(AmbientLight::NONE);
}
