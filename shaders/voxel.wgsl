@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) instance_color: vec4<f32>,
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
    // Front
    0, 2, 1, 0, 3, 2,
    // Right
    1, 6, 5, 1, 2, 6,
    // Back
    5, 7, 4, 5, 6, 7, 
    // Left
    4, 3, 0, 4, 7, 3, 
    // Top
    6, 3, 7, 6, 2, 3,
    // Bottom
    6, 3, 7, 6, 2, 3,  
);

const CUBE_NORMALS: array<vec3<f32>, 6> = array(
    // Front (+Z)
    vec3<f32>( 0.0,  0.0,  1.0),
    // Right (+X)
    vec3<f32>( 1.0,  0.0,  0.0), 
    // Back (-Z)
    vec3<f32>( 0.0,  0.0, -1.0), 
    // Left (-X)
    vec3<f32>(-1.0,  0.0,  0.0), 
    // Top (+Y)
    vec3<f32>( 0.0,  1.0,  0.0),
    // Bottom (-Y)
    vec3<f32>( 0.0, -1.0,  0.0),
);

@vertex
fn vs_main(    
    @builtin(vertex_index) vertex_index: u32,
    @location(0) instance_position: vec3<f32>,
    @location(1) instance_color: vec4<f32>,
) -> VertexOutput {
    var output: VertexOutput;

    let cube_vertex = CUBE_VERTICES[CUBE_INDICES[vertex_index]];
    output.Position = view_proj * vec4(instance_position + cube_vertex, 1.0);
    
    output.instance_color = instance_color;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.instance_color;
}