use bevy::{
  pbr::{VolumetricFog, VolumetricLight},
  prelude::*,
};
//
use matra::{level::actors::Player, prelude::*};

#[derive(Resource)]
struct CycleTimer(Timer);

use {
  bevy::pbr::light_consts::lux::AMBIENT_DAYLIGHT, matra::core::NishitaPlus,
};

fn daylight_cycle(
  mut atmosphere: AtmosphereMut<NishitaPlus>,
  mut query: Query<(&mut Transform, &mut DirectionalLight)>,
  mut timer: ResMut<CycleTimer>,
  time: Res<Time>,
) {
  timer.0.tick(time.delta());

  if timer.0.finished() {
    let t = time.elapsed_secs_wrapped() / 2.0;
    atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());
    // atmosphere.mie_coefficient = 5e-5;

    atmosphere.rayleigh_scale_height = 16e3;
    atmosphere.mie_scale_height = 2.4e3;

    if let Some((mut light_trans, mut directional)) = query.iter_mut().next() {
      light_trans.rotation = Quat::from_rotation_x(-t);
      directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
    }
  }
}

fn main() {
  App::new()
    .add_plugins(GamePlugin)
    .insert_resource(AtmosphereModel::new(NishitaPlus::default()))
    .insert_resource(CycleTimer(Timer::new(
      bevy::utils::Duration::from_millis(5),
      TimerMode::Repeating,
    )))
    .add_systems(Startup, setup)
    .add_systems(Update, tweak_scene)
    .add_systems(Update, daylight_cycle)
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Capsule3d::new(0.5, 2.0));
  let material =
    materials.add(StandardMaterial::from_color(Color::srgb(0.0, 1.0, 0.0)));

  commands
    .spawn(Player)
    .insert((Mesh3d(mesh), MeshMaterial3d(material)))
    .insert(Transform::from_xyz(0.0, 10.0, 0.0));

  commands.spawn((
    BlueprintInfo::from_path("levels/World.glb"),
    SpawnBlueprint,
    HideUntilReady,
    GameWorldTag,
  ));
}

fn tweak_scene(
  mut commands: Commands,
  mut lights: Query<(Entity, &mut DirectionalLight), Changed<DirectionalLight>>,
) {
  for (entity, mut light) in lights.iter_mut() {
    light.shadows_enabled = true;
    commands.entity(entity).insert(VolumetricLight);
  }
}

fn tweak_camera(mut camera: Query<(&mut VolumetricFog,)>) {
  for (mut fog,) in camera.iter_mut() {
    fog.ambient_intensity = 0.1;
  }
}
