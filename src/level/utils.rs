use {
  crate::{level::actors::PrimaryCamera, prelude::*},
  bevy::utils::HashMap,
  hanabi::Gradient,
};

pub fn plugin(app: &mut App) {
  app.register_type::<PortalMarker>().add_systems(Update, spawn);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform)]
pub struct PortalMarker {
  scope: String,
}

pub fn spawn(
  portals: Query<(Entity, &PortalMarker), Added<PortalMarker>>,
  camera: Option<Single<Entity, With<PrimaryCamera>>>,
  mut commands: Commands,
) {
  let Some(camera) = camera else {
    return;
  };

  let mut scopes = HashMap::<_, Vec<_>>::new();

  for (entity, portal) in portals.iter() {
    scopes
      .entry(&portal.scope)
      .and_modify(|scope| scope.push(entity))
      .or_insert(vec![entity]);
  }

  for (scope, portals) in scopes {
    match portals[..] {
      [a, b] => {
        let mut spawn = |a, b| {
          commands.entity(a).remove::<PortalMarker>().insert(
            Portal::new(*camera, b)
              .with_cull_mode(None)
              .with_flip_near_plane_normal(true),
          );
        };
        spawn(a, b);
        spawn(b, a);
      }
      [single] => warn!("found portal without target: {single:?}"),
      _ => warn!("more than one pair of portal from scope: `{scope}`"),
    }
  }
}
