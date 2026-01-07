@group(1) @binding(0) 
var texture_atlas: texture_2d_array<f32>;

@group(1) @binding(1) 
var texture_atlas_sampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) layer_index: u32,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn main(input: FragmentInput) -> FragmentOutput {
    var output: FragmentOutput;
    
    let color = textureSample(
        texture_atlas, 
        texture_atlas_sampler, 
        input.uv, 
        input.layer_index
    );

    output.color = color;
    
    return output;
}
