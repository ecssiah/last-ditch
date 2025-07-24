struct CameraUniformData {
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
};

struct Transform {
    matrix: mat4x4<f32>,
};

@group(0) @binding(0) 
var<uniform> camera_uniform_data: CameraUniformData;

@group(1) @binding(0)
var<uniform> entity_index: u32;

@group(2) @binding(0)
var<storage, read> transform_array: array<Transform>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let model_matrix = transform_array[entity_index].matrix;

    output.Position = camera_uniform_data.view_projection_matrix * model_matrix * vec4<f32>(input.position, 1.0);
    output.uv = input.uv;

    return output;
}
