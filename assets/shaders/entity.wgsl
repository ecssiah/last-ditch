struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(4) instance_pos: vec3<f32>,
    @location(5) instance_height: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_position = input.position + vec3<f32>(input.instance_pos.x, input.instance_pos.y + input.instance_height, input.instance_pos.z);
    out.position = vec4<f32>(world_position, 1.0);

    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.6, 0.54, 0.63, 1.0);
}
