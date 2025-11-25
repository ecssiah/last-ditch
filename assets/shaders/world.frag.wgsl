@group(1) @binding(0) var tile_array: texture_2d_array<f32>;
@group(1) @binding(1) var tile_sampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) layer: u32,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn main(input: FragmentInput) -> FragmentOutput {
    var output: FragmentOutput;
    
    let color = textureSample(tile_array, tile_sampler, input.uv, input.layer);

    output.color = color;
    
    return output;
}
