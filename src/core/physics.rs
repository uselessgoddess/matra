use {crate::prelude::*, bevy::prelude::*};

pub fn plugin(app: &mut App) {
  app.add_plugins((PortalPlugins, PhysicsPlugins::default())).add_plugins((
    TnuaControllerPlugin::new(FixedUpdate),
    TnuaAvian3dPlugin::new(FixedUpdate),
  ));
}
