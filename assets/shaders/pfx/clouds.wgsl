#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;

@fragment
fn fragment(
	in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
    let screen_size = vec2i(textureDimensions(screen_texture));
    let pixel_position = vec2i(floor(in.uv * vec2f(screen_size)));

    let base_color = textureSample(screen_texture, screen_sampler, in.uv);

    return base_color;
}