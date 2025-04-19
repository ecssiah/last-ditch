struct ViewProjection {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
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
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_position = input.position + input.instance_pos;
    out.position = view_projection.view_proj * vec4<f32>(world_position, 1.0);
    out.normal = input.normal;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color = vec4<f32>((input.normal * 0.5) + vec3<f32>(0.5), 1.0);
    return color;
}
