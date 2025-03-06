@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) instance_position: vec3f
};

const CUBE_VERTICES: array<vec3<f32>, 8> = array(
    vec3<f32>(-0.5, -0.5, -0.5),
    vec3<f32>( 0.5, -0.5, -0.5),
    vec3<f32>( 0.5,  0.5, -0.5),
    vec3<f32>(-0.5,  0.5, -0.5),
    vec3<f32>(-0.5, -0.5,  0.5),
    vec3<f32>( 0.5, -0.5,  0.5),
    vec3<f32>( 0.5,  0.5,  0.5),
    vec3<f32>(-0.5,  0.5,  0.5),
);

const CUBE_INDICES: array<u32, 36> = array(
    0, 1, 2, 2, 3, 0, // Front
    1, 5, 6, 6, 2, 1, // Right
    5, 4, 7, 7, 6, 5, // Back
    4, 0, 3, 3, 7, 4, // Left
    3, 2, 6, 6, 7, 3, // Top
    4, 5, 1, 1, 0, 4  // Bottom
);

@vertex
fn vs_main(    
    @builtin(vertex_index) vertex_index: u32,
    @location(0) instance_position: vec3<f32>,
    @location(1) _padding: f32,
) -> VertexOutput {
    var output: VertexOutput;

    let cube_vertex = CUBE_VERTICES[CUBE_INDICES[vertex_index]];
    output.Position = view_proj * vec4(instance_position + cube_vertex, 1.0);
    output.instance_position = instance_position;

    return output;
}

@fragment
fn fs_main(@location(0) instance_position: vec3f) -> @location(0) vec4<f32> {
    let max_range = 10.0;

    return vec4(
        instance_position.x / max_range, 
        instance_position.y / max_range, 
        instance_position.z / max_range, 
        1.0,
    );
}

fn random(seed: f32) -> f32 {
    return fract(sin(seed) * 43758.5453123);
}