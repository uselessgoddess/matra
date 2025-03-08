use bevy::prelude::*;

use {avian3d::prelude::*, bevy_tnua::prelude::*, bevy_tnua_avian3d::*};

use {
  crate::GameState,
  serde::{Deserialize, Serialize},
};

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
#[require(super::PrimaryCamera)]
pub struct PlayerCamera {
  pub axis: f32,
}

pub fn plugin(app: &mut App) {
  app
    .register_type::<Player>()
    .register_type::<PlayerCamera>()
    .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
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
        parent
          .spawn((Camera3d::default(), Transform {
            translation: Vec3::new(0.0, 1.95, 0.0),
            ..default()
          }))
          .insert(PlayerCamera::default());
      });
  }
}
