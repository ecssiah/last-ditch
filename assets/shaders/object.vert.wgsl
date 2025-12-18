struct CameraUniformData {
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
};

@group(0) @binding(0) 
var<uniform> camera_uniform_data: CameraUniformData;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) instance_world_position: vec3<f32>,
    @location(4) instance_rotation_xy: f32,
};

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn main(input: VertexInput) -> VertexOutput {
    let model_matrix = get_model_matrix(
        input.instance_world_position, 
        input.instance_rotation_xy
    );

    let mvp_matrix = camera_uniform_data.view_projection_matrix * model_matrix;

    var output: VertexOutput;

    output.Position = mvp_matrix * vec4<f32>(input.position, 1.0);
    output.uv = input.uv;

    return output;
}

fn get_model_matrix(world_position: vec3<f32>, rotation_xy: f32) -> mat4x4<f32> {
    let cos_xy = cos(rotation_xy);
    let sin_xy = sin(rotation_xy);

    let rotation_matrix = mat4x4<f32>(
        vec4<f32>(cos_xy, sin_xy, 0.0, 0.0),
        vec4<f32>(-sin_xy, cos_xy, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );

    let translation_matrix = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(world_position, 1.0),
    );

    let scale_x = 1.0;
    let scale_y = 1.0;
    let scale_z = 1.0;

    let scale_matrix = mat4x4<f32>(
        vec4<f32>(scale_x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, scale_y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, scale_z, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    return translation_matrix * rotation_matrix * scale_matrix;
}
