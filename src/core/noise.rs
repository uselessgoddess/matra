use noise::{NoiseFn, Perlin};

pub fn perlin_1d(x: f32) -> f32 {
  Perlin::default().get([x as f64]) as f32
}
