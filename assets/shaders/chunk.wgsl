struct FogUniformData {
    color: vec3<f32>,
    start: f32,
    end: f32,
};

@group(0) @binding(0) 
var<uniform> fog_uniform_data: FogUniformData;

struct CameraUniformData {
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
};

@group(1) @binding(0) 
var<uniform> camera_uniform_data: CameraUniformData;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) light: f32,
};

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) light: f32,
    @location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.Position = camera_uniform_data.view_projection_matrix * vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    output.light = input.light;
    output.world_position = input.position;

    return output;
}

@group(2) @binding(0) var atlas: texture_2d<f32>;
@group(2) @binding(1) var atlas_sampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) light: f32,
    @location(2) world_position: vec3<f32>,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(input: FragmentInput) -> FragmentOutput {
    var output: FragmentOutput;
    
    let sampled_color = textureSample(atlas, atlas_sampler, input.uv).rgb;
    let lit_color = sampled_color * input.light;

    let distance = length(input.world_position - camera_uniform_data.camera_position);
    let fog_factor = select(
        1.0,
        clamp((fog_uniform_data.end - distance) / max(0.001, fog_uniform_data.end - fog_uniform_data.start), 0.0, 1.0),
        distance > fog_uniform_data.start
    );
    let fog_color = srgb_to_linear(fog_uniform_data.color);

    let final_color = mix(lit_color, fog_color, 1.0 - fog_factor);
    
    let gamma_corrected_color = linear_to_srgb(final_color);

    output.color = vec4<f32>(gamma_corrected_color, 1.0);
    
    return output;
}

fn linear_to_srgb(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / 2.2));
}

fn srgb_to_linear(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(2.2));
}
