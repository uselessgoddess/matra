use {
  crate::prelude::*,
  bevy::render::{RenderApp, render_resource::ShaderType},
  bevy_atmosphere::pipeline::AtmosphereImageBindGroupLayout,
};

pub struct NishitaPlugin;

impl Plugin for NishitaPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(AtmosphereModel::new(NishitaPlus::default()))
      .add_systems(Update, nishita_time);
  }

  fn finish(&self, app: &mut App) {
    let render_app = app.sub_app_mut(RenderApp);

    render_app.init_resource::<AtmosphereImageBindGroupLayout>();

    app.add_atmosphere_model::<NishitaPlus>();
  }
}

fn nishita_time(mut atmosphere: AtmosphereMut<NishitaPlus>, time: Res<Time>) {
  atmosphere.time += time.delta_secs();
}

#[derive(Atmospheric, ShaderType, Reflect, Debug, Clone)]
#[uniform(0, NishitaPlus)]
#[internal("core/atmosphere/shaders/nishita.wgsl")]
pub struct NishitaPlus {
  pub ray_origin: Vec3,
  pub sun_position: Vec3,
  pub sun_intensity: f32,
  pub planet_radius: f32,
  pub atmosphere_radius: f32,
  pub rayleigh_coefficient: Vec3,
  pub rayleigh_scale_height: f32,
  pub mie_coefficient: f32,
  pub mie_scale_height: f32,
  pub mie_direction: f32,
  // plus
  pub time: f32,
}

impl From<&NishitaPlus> for NishitaPlus {
  fn from(nishita: &NishitaPlus) -> Self {
    nishita.clone()
  }
}

impl Default for NishitaPlus {
  fn default() -> Self {
    Self {
      ray_origin: Vec3::new(0.0, 6372e3, 0.0),
      sun_position: Vec3::new(1.0, 1.0, 1.0),
      sun_intensity: 22.0,
      planet_radius: 6371e3,
      atmosphere_radius: 6471e3,
      rayleigh_coefficient: Vec3::new(5.5e-6, 13.0e-6, 22.4e-6),
      rayleigh_scale_height: 8e3,
      mie_coefficient: 21e-6,
      mie_scale_height: 1.2e3,
      mie_direction: 0.758,
      //
      time: 0.0,
    }
  }
}
