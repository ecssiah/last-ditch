@group(1) @binding(0) var mesh_texture: texture_2d<f32>;
@group(1) @binding(1) var mesh_sampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn main(input: FragmentInput) -> FragmentOutput {
    var output: FragmentOutput;
    
    let color = textureSample(mesh_texture, mesh_sampler, input.uv);

    output.color = color;
    
    return output;
}
