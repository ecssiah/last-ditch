struct Fog {
    color: vec3<f32>,
    start: f32,
    end: f32,
};
@group(0) @binding(0) var<uniform> fog: Fog;

struct ViewProjection {
    matrix: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> view_projection: ViewProjection;

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
    out.position = view_projection.matrix * vec4<f32>(world_position, 1.0);
    out.normal = input.normal;
    out.world_position = world_position;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = vec3<f32>((input.normal * 0.5) + vec3<f32>(0.5));
    let distance = length(input.world_position);
    let fog_factor = clamp((fog.end - distance) / (fog.end - fog.start), 0.0, 1.0);
    let final_color = mix(fog.color, base_color, fog_factor);

    return vec4<f32>(final_color, 1.0);
}
