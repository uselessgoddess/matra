mod editor;

use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  prelude::*,
};

use crate::prelude::*;

pub fn plugin(app: &mut App) {
  {
    app.add_plugins((
      FrameTimeDiagnosticsPlugin,
      LogDiagnosticsPlugin::filtered(vec![]),
      PhysicsDebugPlugin::default(),
      PortalGizmosPlugin,
      editor::plugin,
    ));
    // .insert_gizmo_group(
    //   PhysicsGizmos { aabb_color: Some(Color::WHITE), ..default() },
    //   GizmoConfig { enabled: false, ..default() },
    // );
  }
}
