pub mod debug_renderer;
pub mod population_renderer;
pub mod render_catalog;
pub mod render_context;
pub mod structure_renderer;
pub mod texture;
pub mod world_renderer;

use crate::interface::{
    camera::Camera,
    gpu::gpu_context::GPUContext,
    renderer::{
        debug_renderer::DebugRenderer, population_renderer::PopulationRenderer,
        render_context::RenderContext, structure_renderer::StructureRenderer,
        world_renderer::WorldRenderer,
    },
};

pub struct Renderer {
    pub population_renderer: PopulationRenderer,
    pub structure_renderer: StructureRenderer,
    pub world_renderer: WorldRenderer,
    pub debug_renderer: DebugRenderer,
}

impl Renderer {
    pub fn new(gpu_context: &GPUContext, render_context: &RenderContext, camera: &Camera) -> Self {
        let world_renderer = WorldRenderer::new(gpu_context, render_context, camera);
        let structure_renderer = StructureRenderer::new(gpu_context, render_context, camera);
        let population_renderer = PopulationRenderer::new(gpu_context, render_context, camera);
        let debug_renderer = DebugRenderer::new(gpu_context, camera);

        Self {
            world_renderer,
            structure_renderer,
            population_renderer,
            debug_renderer,
        }
    }
}
