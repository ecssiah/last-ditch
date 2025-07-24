@group(3) @binding(0) var mesh_texture: texture_2d<f32>;
@group(3) @binding(1) var mesh_sampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn main(input: FragmentInput) -> FragmentOutput {
    var output: FragmentOutput;
    
    let sampled_color = textureSample(mesh_texture, mesh_sampler, input.uv).rgb;
    let gamma_corrected_color = linear_to_srgb(sampled_color);

    output.color = vec4<f32>(gamma_corrected_color, 1.0);
    
    return output;
}

fn linear_to_srgb(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / 2.2));
}

fn srgb_to_linear(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(2.2));
}
