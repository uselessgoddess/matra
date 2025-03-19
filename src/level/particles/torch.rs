use {crate::prelude::*, hanabi::prelude::*};

pub fn create(radius: f32) -> EffectAsset {
  let mut color_gradient = Gradient::new();
  color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
  color_gradient.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
  color_gradient.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
  color_gradient.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

  let mut size_gradient = Gradient::new();
  size_gradient.add_key(0.3, Vec3::new(0.2, 0.02, 1.0));
  size_gradient.add_key(1.0, Vec3::splat(0.0));

  let writer = ExprWriter::new();

  let init_pos = SetPositionSphereModifier {
    center: writer.lit(Vec3::ZERO).expr(),
    radius: writer.lit(Vec3::splat(radius)).expr(),
    dimension: ShapeDimension::Volume,
  };

  let age = writer.lit(0.).expr();
  let init_age = SetAttributeModifier::new(Attribute::AGE, age);

  // Give a bit of variation by randomizing the lifetime per particle
  let lifetime = writer.lit(0.0).uniform(writer.lit(0.6)).expr();
  let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

  // Add drag to make particles slow down a bit after the initial acceleration
  let drag = writer.lit(2.).expr();
  let update_drag = LinearDragModifier::new(drag);

  let mut module = writer.finish();

  let update_accel = AccelModifier::constant(&mut module, Vec3::Y * 10.0);

  EffectAsset::new(16384 * 32, SpawnerSettings::rate(50000.0.into()), module)
    .with_name("torch")
    .init(init_pos)
    .init(init_age)
    .init(init_lifetime)
    .update(update_drag)
    .update(update_accel)
    .render(ColorOverLifetimeModifier { gradient: color_gradient })
    .render(SizeOverLifetimeModifier {
      gradient: size_gradient,
      screen_space_size: false,
    })
    .render(OrientModifier::new(OrientMode::AlongVelocity))
}
