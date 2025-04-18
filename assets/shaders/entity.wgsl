struct ViewProjection {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> view_projection: ViewProjection;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(4) instance_pos: vec3<f32>,
    @location(5) instance_height: f32,
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) vertex_index: u32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_position = input.position + input.instance_pos;
    out.position = view_projection.view_proj * vec4<f32>(world_position, 1.0);
    out.vertex_index = input.vertex_index;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let index_f = f32(input.vertex_index % 4) / 4.0;

    let color = vec4<f32>(index_f, index_f, 0.5, 1.0);

    return color;
}
