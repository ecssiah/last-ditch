const CUBE_VERTICES: array<vec3<f32>, 8> = array(
    vec3<f32>(-0.5, -0.5, -0.5),
    vec3<f32>( 0.5, -0.5, -0.5),
    vec3<f32>(-0.5,  0.5, -0.5),
    vec3<f32>( 0.5,  0.5, -0.5),
    vec3<f32>(-0.5, -0.5,  0.5),
    vec3<f32>( 0.5, -0.5,  0.5),
    vec3<f32>(-0.5,  0.5,  0.5),
    vec3<f32>( 0.5,  0.5,  0.5),
);

const CUBE_INDICES: array<u32, 36> = array(
    1, 0, 2, 1, 2, 3,
    4, 5, 7, 4, 7, 6,
    5, 1, 3, 5, 3, 7, 
    0, 4, 6, 0, 6, 2, 
    6, 7, 3, 6, 3, 2,
    0, 1, 5, 0, 5, 4,  
);

@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) instance_position: vec3<f32>,
    @location(1) instance_color: vec4<f32>,
    @location(2) ao_1: vec4<f32>,
    @location(3) ao_2: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) instance_color: vec4<f32>,
    @location(1) vertex_ao: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let cube_index = CUBE_INDICES[input.vertex_index];
    let cube_vertex = CUBE_VERTICES[cube_index];

    output.Position = view_proj * vec4(input.instance_position.xyz + cube_vertex, 1.0);
    output.instance_color = input.instance_color;
    output.vertex_ao = select_ao(cube_index, input.ao_1, input.ao_2);

    return output;
}

struct FragmentInput {
    @location(0) instance_color: vec4<f32>,
    @location(1) vertex_ao: f32,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>
}

@fragment
fn fs_main(input: FragmentInput) -> FragmentOutput {
    var fragment_output: FragmentOutput;

    let occluded_color = input.instance_color.rgb * input.vertex_ao;

    fragment_output.color = vec4<f32>(occluded_color, input.instance_color.a);

    return fragment_output;
}

fn select_ao(index: u32, ao_1: vec4<f32>, ao_2: vec4<f32>) -> f32 {
    if index < 4 {
        return ao_1[index];
    } else {
        return ao_2[index - 4];
    }
}