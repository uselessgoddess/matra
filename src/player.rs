use bevy::{
  input::mouse::MouseMotion,
  window::{CursorGrabMode, PrimaryWindow},
};

use {
  crate::{
    level::actors::{Player, PlayerCamera},
    prelude::*,
  },
  std::f32::consts::PI,
};

#[derive(Resource, Deref, DerefMut)]
struct MouseLocked(bool);

pub fn plugin(app: &mut App) {
  app
    .insert_resource(MouseLocked(true))
    .add_systems(Update, (mouse_lock, toggle_lock))
    .add_systems(Update, (movement, rotation));
}

fn toggle_lock(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut locked: ResMut<MouseLocked>,
) {
  if keyboard.just_pressed(KeyCode::Escape) {
    locked.0 = !locked.0;
  }
}

fn mouse_lock(
  locked: Res<MouseLocked>,
  mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
  if locked.is_changed() {
    let mut window = window.single_mut();
    (window.cursor_options.grab_mode, window.cursor_options.visible) =
      if locked.0 {
        (CursorGrabMode::Locked, false)
      } else {
        (CursorGrabMode::None, true)
      };
  }
}

fn movement(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut query: Query<(&mut TnuaController, &Transform), With<Player>>,
) {
  let Ok((mut controller, transform)) = query.get_single_mut() else {
    return;
  };

  let mut direction = Vec3::ZERO;
  if keyboard.pressed(KeyCode::KeyW) {
    direction -= Vec3::Z;
  }
  if keyboard.pressed(KeyCode::KeyS) {
    direction += Vec3::Z;
  }
  if keyboard.pressed(KeyCode::KeyA) {
    direction -= Vec3::X;
  }
  if keyboard.pressed(KeyCode::KeyD) {
    direction += Vec3::X;
  }

  // transform direction to correspond to camera rotation
  direction = (transform.rotation * direction) * Vec3::new(1.0, 0.0, 1.0);

  // set controller basis
  controller.basis(TnuaBuiltinWalk {
    desired_velocity: direction.normalize_or_zero() * 10.0,
    float_height: 1.5,
    ..default()
  });
}

fn rotation(
  locked: Res<MouseLocked>,
  mut motion: EventReader<MouseMotion>,
  mut player_transform: Query<&mut Transform, With<Player>>,
  mut camera_transform: Query<
    (&mut Transform, &mut PlayerCamera),
    (With<Camera3d>, Without<Player>),
  >,
) {
  const SENS: f32 = 0.0020;

  if !locked.0 {
    return;
  }

  let Ok(mut player_transform) = player_transform.get_single_mut() else {
    return;
  };
  let Ok((mut camera_transform, mut camera)) =
    camera_transform.get_single_mut()
  else {
    return;
  };

  for ev in motion.read() {
    player_transform.rotate_y(-ev.delta.x * SENS);

    camera.axis = (camera.axis - ev.delta.y * SENS).clamp(-PI / 2.0, PI / 2.0);
    camera_transform.rotation = Quat::from_rotation_x(camera.axis);
  }
}
