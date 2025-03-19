use {
  crate::{
    config::GameConfig,
    core::noise::perlin_1d,
    level::actors::{CameraLook, Player, PlayerCamera},
    prelude::*,
    utils::single,
  },
  bevy::{
    input::mouse::MouseMotion,
    window::{CursorGrabMode, PrimaryWindow},
  },
};

#[derive(Resource, Deref, DerefMut)]
struct MouseLocked(bool);

pub fn plugin(app: &mut App) {
  app
    .insert_resource(MouseLocked(true))
    .add_systems(Update, (mouse_lock, toggle_lock))
    .add_systems(Update, (rotation, target, dolly))
    .add_systems(Update, movement);
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
    desired_velocity: direction.normalize_or_zero() * 6.5,
    float_height: 1.5,
    ..default()
  });
}

fn rotation(
  locked: Res<MouseLocked>,
  mut motion: EventReader<MouseMotion>,
  mut player: Query<&mut Transform, With<Player>>,
  mut camera: Query<&mut PlayerCamera>,
) {
  use std::f32::consts::PI;

  const SENS: f32 = 0.0010_f32;

  if !locked.0 {
    return;
  }

  single!(mut player, mut camera);

  for ev in motion.read() {
    player.rotate_y(-ev.delta.x * SENS / 2.0);

    // camera.yaw = camera.yaw - ev.delta.x * SENS;
    camera.pit =
      (camera.pit - ev.delta.y * SENS).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
  }
}

fn target(
  mut target: Query<&mut Transform, With<CameraLook>>,
  camera: Query<&PlayerCamera>,
) {
  single!(mut target, camera);

  let look = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pit, 0.0)
    .mul_vec3(Vec3::Z);

  target.translation = look * -5.0;
}

fn dolly(
  player: Query<&TnuaController, With<Player>>,
  target: Query<&Transform, With<CameraLook>>,
  mut rig: Query<&mut Rig>,
  config: Res<GameConfig>,
  time: Res<Time>,
) {
  single!(player, target);

  let mut offset = Vec2::ZERO;

  // breathing
  offset.y += perlin_1d(time.elapsed_secs() / 5.0) / 5.0;

  let perlin = |x| perlin_1d(x + time.elapsed_secs() * 1.5);
  // steps
  if let Some(basis) = player.dynamic_basis()
    && basis.effective_velocity().length() >= 0.1
  {
    offset +=
      Vec2::new(perlin(1.3), perlin(3.7)) * config.camera.dolly.steps / 100.0;
  }

  for mut rig in rig.iter_mut() {
    rig.driver_mut::<LookAt>().target = target.translation + offset.extend(0.0);
  }
}
