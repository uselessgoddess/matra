#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var threshold_map_texture: texture_2d<f32>;
@group(0) @binding(3) var threshold_map_sampler: sampler;

fn quantize_color(color: vec3<f32>, levels: f32) -> vec3<f32> {
    let step = 1.0 / levels;
    return round(color / step) * step;
}

fn luma(color: vec3<f32>) -> f32 {
  return dot(color, vec3<f32>(0.299, 0.587, 0.114));
}

fn aces(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return (x * (a * x + b)) / (x * (c * x + d) + e);
}

@fragment
fn fragment(
	in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
    let screen_size = vec2i(textureDimensions(screen_texture));
    let threshold_map_size = vec2i(textureDimensions(threshold_map_texture));
    let pixel_position = vec2i(floor(in.uv * vec2f(screen_size)));
    let map_position = vec2f(pixel_position % threshold_map_size) / vec2f(threshold_map_size);

    let threshold = textureSample(threshold_map_texture, threshold_map_sampler, map_position).r;

    let base_color = textureSample(screen_texture, screen_sampler, in.uv);
    let luma = luma(base_color.xyz);

    if (luma > threshold) {
        return vec4f(base_color.xyz * 1.5, 1.0);
    } else {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }
}