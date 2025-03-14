use crate::{GameState, prelude::*};

#[derive(Debug, Component)]
pub struct Player;

impl Player {
  pub const HEIGHT: f32 = 1.95;
}

#[derive(Debug, Component, Default)]
#[require(Camera3d, super::PrimaryCamera)]
pub struct PlayerCamera {
  pub yaw: f32,
  pub pit: f32,
}

#[derive(Debug, Component)]
pub struct CameraLook;

impl CameraLook {
  pub const ARM: f32 = -5.0;
}

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn.run_if(in_state(GameState::Playing)));

  #[cfg(debug_assertions)]
  app.add_systems(Update, camera_gizmos);
}

pub fn spawn(player: Query<Entity, Added<Player>>, mut commands: Commands) {
  for entity in player.iter() {
    commands
      .entity(entity)
      .insert(RigidBody::Dynamic)
      .insert((
        Collider::capsule(0.5, 1.0),
        LockedAxes::ROTATION_LOCKED,
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(0.0, 0.99)),
      ))
      .with_children(|parent| {
        parent.spawn((
          Transform::from_xyz(0.0, Player::HEIGHT, 0.0),
          PlayerCamera::default(),
        ));
        parent
          .spawn((Transform::from_xyz(0.0, 1.95, CameraLook::ARM), CameraLook));
      });
  }
}

fn camera_gizmos(
  query: Query<&Transform, With<CameraLook>>,
  mut gizmos: Gizmos,
) {
  if !debug::gizmos() {
    return;
  }

  for transform in query.iter() {
    gizmos.cross(transform.to_isometry(), 1.0, Color::WHITE);
  }
}
