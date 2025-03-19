mod torch;

use {
  super::utils::register, crate::prelude::*,
  bevy::reflect::GetTypeRegistration, std::any::type_name,
};

trait Spawn {
  fn spawn(
    &self,
    entity: Entity,
    commands: &mut Commands,
    effects: &mut Assets<EffectAsset>,
  );
}

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Default, Component)]
pub struct Torch {
  pub radius: f32,
}

impl Default for Torch {
  fn default() -> Self {
    Self { radius: 0.25 }
  }
}

impl Spawn for Torch {
  fn spawn(
    &self,
    entity: Entity,
    commands: &mut Commands,
    effects: &mut Assets<EffectAsset>,
  ) {
    let torch = ParticleEffect::new(effects.add(torch::create(self.radius)));

    commands.entity(entity).insert(torch).insert(GravityScale(0.0));
  }
}

pub fn plugin(app: &mut App) {
  register::<Torch>(app);
}

fn register<T: Spawn + Component + GetTypeRegistration>(app: &mut App) {
  app
    .register_type::<T>()
    .add_systems(Update, spawn::<T>.run_if(in_state(GameState::Playing)));
}

pub fn spawn<T: Spawn + Component>(
  mut commands: Commands,
  added: Query<(Entity, &T)>,
  mut effects: ResMut<Assets<EffectAsset>>,
) {
  for (entity, component) in added.iter() {
    component.spawn(entity, &mut commands, &mut effects);
    commands.entity(entity).insert(Name::new(type_name::<T>())).remove::<T>();
  }
}
