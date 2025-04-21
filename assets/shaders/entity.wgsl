struct GPUFog {
    color: vec3<f32>,
    start: f32,
    end: f32,
};
@group(0) @binding(0) var<uniform> gpu_fog: GPUFog;

struct GPUCamera {
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding: f32,
};

@group(1) @binding(0)
var<uniform> gpu_camera: GPUCamera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(4) instance_pos: vec3<f32>,
    @location(5) instance_height: f32,
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_position = input.position + input.instance_pos;
    out.position = gpu_camera.view_projection_matrix * vec4<f32>(world_position, 1.0);
    out.normal = input.normal;
    out.world_position = world_position;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = vec3<f32>((input.normal * 0.5) + vec3<f32>(0.5));
    
    let distance = length(input.world_position - gpu_camera.camera_position);
    let fog_factor = clamp((gpu_fog.end - distance) / (gpu_fog.end - gpu_fog.start), 0.0, 1.0);
    
    let final_color = mix(gpu_fog.color, base_color, fog_factor);

    return vec4<f32>(final_color, 1.0);
}
