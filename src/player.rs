use bevy::{
  input::mouse::MouseMotion,
  window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
  level::actors::{CameraLook, Player},
  prelude::*,
  utils::single,
};

#[derive(Resource, Deref, DerefMut)]
struct MouseLocked(bool);

pub fn plugin(app: &mut App) {
  app
    .insert_resource(MouseLocked(true))
    .add_systems(Update, (mouse_lock, toggle_lock))
    .add_systems(Update, (movement, rotation, dolly));
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
    desired_velocity: direction.normalize_or_zero() * 5.0,
    float_height: 1.5,
    ..default()
  });
}

fn rotation(
  locked: Res<MouseLocked>,
  mut motion: EventReader<MouseMotion>,
  mut player: Query<&mut Transform, With<Player>>,
  mut camera: Query<&mut Rig, (With<Camera3d>, Without<Player>)>,
) {
  const SENS: f32 = 0.0010_f32;

  if !locked.0 {
    return;
  }

  single!(mut player, mut camera);

  for ev in motion.read() {
    let mut camera = camera.driver_mut::<YawPitch>();

    // player.rotate_y(-ev.delta.x * SENS);
    player.rotate_y(-ev.delta.x * SENS / 2.0);
    camera.rotate_yaw_pitch(
      0.0, // -ev.delta.x * SENS,
      -ev.delta.y * SENS.to_degrees(),
    );
  }
}

fn dolly(
  mut target: Query<&Transform, With<CameraLook>>,
  mut rig: Query<&mut Rig>,
) {
  single!(target);

  for mut rig in rig.iter_mut() {
    rig.driver_mut::<LookAt>().target = target.translation;
  }
}
