pub mod person_renderer;

use crate::{
    interface::{
        asset_manager::AssetManager, camera::Camera, gpu::gpu_context::GPUContext,
        renderer::population_renderer::person_renderer::PersonRenderer,
    },
    simulation::supervisor::viewer::view::PopulationView,
};

pub struct PopulationRenderer {
    pub person_renderer: PersonRenderer,
}

impl PopulationRenderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let person_renderer = PersonRenderer::new(gpu_context, camera);

        Self { person_renderer }
    }

    pub fn apply_population_view(
        gpu_context: &GPUContext,
        asset_manager: &AssetManager,
        population_view: &PopulationView,
        population_renderer: &mut Self,
    ) {
        PersonRenderer::apply_population_view(
            gpu_context,
            asset_manager,
            population_view,
            &mut population_renderer.person_renderer,
        );
    }

    pub fn render(
        gpu_context: &GPUContext,
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        asset_manager: &AssetManager,
        population_renderer: &Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        PersonRenderer::render(
            gpu_context,
            surface_texture_view,
            depth_texture_view,
            camera_uniform_bind_group,
            asset_manager,
            &population_renderer.person_renderer,
            encoder,
        );
    }
}
