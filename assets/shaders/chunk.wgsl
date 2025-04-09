@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) light: f32,
};

struct VertexOutput {
    @builtin(position) Position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) light: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.Position = view_proj * vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    output.light = input.light;

    return output;
}

@group(1) @binding(0) var atlas: texture_2d<f32>;
@group(1) @binding(1) var atlas_sampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) light: f32,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(input: FragmentInput) -> FragmentOutput {
    var output: FragmentOutput;
    output.color = textureSample(atlas, atlas_sampler, input.uv);

    return output;
}
