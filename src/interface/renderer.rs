pub mod debug_renderer;
pub mod population_renderer;
pub mod render_catalog;
pub mod render_context;
pub mod texture;
pub mod world_renderer;

use crate::interface::{
    camera::Camera,
    gpu::gpu_context::GPUContext,
    renderer::{
        debug_renderer::DebugRenderer, population_renderer::PopulationRenderer,
        texture::texture_manager::TextureManager, world_renderer::WorldRenderer,
    },
};

pub struct Renderer {
    pub population_renderer: PopulationRenderer,
    pub world_renderer: WorldRenderer,
    pub debug_renderer: DebugRenderer,
}

impl Renderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let world_renderer = WorldRenderer::new(gpu_context, camera);
        let population_renderer = PopulationRenderer::new(gpu_context, camera);
        let debug_renderer = DebugRenderer::new(gpu_context, camera);

        Self {
            world_renderer,
            population_renderer,
            debug_renderer,
        }
    }

    pub fn setup_bind_groups(
        gpu_context: &GPUContext,
        texture_manager: &TextureManager,
        renderer: &mut Self,
    ) {
        let world_renderer_bind_group = WorldRenderer::setup_bind_group(
            gpu_context,
            texture_manager,
            &renderer.world_renderer.bind_group_layout,
        );

        renderer.world_renderer.bind_group = Some(world_renderer_bind_group);

        let population_renderer_bind_group = PopulationRenderer::setup_bind_group(
            gpu_context,
            texture_manager,
            &renderer.population_renderer.bind_group_layout,
        );

        renderer.population_renderer.bind_group = Some(population_renderer_bind_group);
    }
}
