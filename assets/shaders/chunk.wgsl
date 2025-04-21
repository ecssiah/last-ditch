struct GPUFog {
    color: vec3<f32>,
    start: f32,
    end: f32,
};

@group(0) @binding(0) 
var<uniform> gpu_fog: GPUFog;

struct GPUCamera {
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding: f32, // keep 16-byte alignment
};

@group(1) @binding(0) 
var<uniform> gpu_camera: GPUCamera;

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
    output.Position = gpu_camera.view_projection_matrix * vec4<f32>(input.position, 1.0);
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

    let distance = length(input.world_position - gpu_camera.camera_position);    
    let fog_factor = clamp((gpu_fog.end - distance) / (gpu_fog.end - gpu_fog.start), 0.0, 1.0);
    let final_color = mix(gpu_fog.color, lit_color, fog_factor);

    output.color = vec4<f32>(final_color, 1.0);
    return output;
}
