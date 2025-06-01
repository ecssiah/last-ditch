struct CameraUniformData {
    view_projection_matrix: mat4x4<f32>,
    camera_position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera_uniform_data: CameraUniformData;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(5) instance_position: vec3<f32>,
    @location(6) instance_height: f32,
    @location(7) instance_color: vec4<f32>,
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) instance_color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let scale = input.instance_height / 1.0;

    let scaled_position = vec3<f32>(
        scale * input.position.x, 
        scale * input.position.y, 
        scale * input.position.z
    );

    let world_position = scaled_position + input.instance_position + vec3<f32>(0.0, 0.5, 0.0);
    
    out.position = camera_uniform_data.view_projection_matrix * vec4<f32>(world_position, 1.0);
    out.normal = input.normal;
    out.world_position = world_position;
    out.instance_color = input.instance_color;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let vertical_brightness = clamp(input.normal.y * 0.5 + 0.5, 0.0, 1.5);

    let direction_factor = abs(input.normal.z) - abs(input.normal.x);

    let direction_adjustment = clamp(direction_factor * 0.1, -0.05, 0.05);
    let brightness = vertical_brightness + direction_adjustment;

    let base_color = input.instance_color.rgb * brightness;

    let gamma_corrected_color = linear_to_srgb(base_color);

    return vec4<f32>(gamma_corrected_color, 1.0);
}

fn linear_to_srgb(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / 2.2));
}

fn srgb_to_linear(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(2.2));
}
