@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) ao: u32,
};

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) ao: f32,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.Position = view_proj * vec4<f32>(input.position, 1.0);
    output.color = input.color;

    var ao_strength: f32 = 1.0;

    if input.ao == 2 {
        ao_strength = 0.05;
    } else if input.ao == 1 {
        ao_strength = 0.5;
    }

    output.ao = ao_strength;

    return output;
}

struct FragmentInput {
    @location(0) color: vec4<f32>,
    @location(1) ao: f32,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(input: FragmentInput) -> FragmentOutput {
    let shaded = input.color.rgb * input.ao;

    return FragmentOutput(vec4<f32>(shaded, input.color.a));
}
