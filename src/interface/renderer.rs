pub mod debug_renderer;
pub mod population_renderer;
pub mod world_renderer;

use crate::interface::{
    asset_manager::AssetManager,
    camera::Camera,
    gpu::gpu_context::GPUContext,
    interface_mode::InterfaceMode,
    renderer::{
        debug_renderer::DebugRenderer,
        population_renderer::{person_renderer::PersonRenderer, PopulationRenderer},
        world_renderer::{sector_renderer::SectorRenderer, WorldRenderer},
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
        asset_manager: &AssetManager,
        renderer: &mut Self,
    ) {
        let sector_renderer_bind_group = SectorRenderer::setup_bind_group(
            gpu_context,
            asset_manager,
            &renderer.world_renderer.sector_renderer.bind_group_layout,
        );

        renderer.world_renderer.sector_renderer.bind_group = Some(sector_renderer_bind_group);

        let person_renderer_bind_group = PersonRenderer::setup_bind_group(
            gpu_context,
            asset_manager,
            &renderer
                .population_renderer
                .person_renderer
                .bind_group_layout,
        );

        renderer.population_renderer.person_renderer.bind_group = Some(person_renderer_bind_group);
    }

    pub fn render(
        interface_mode: &InterfaceMode,
        surface_texture_view: &wgpu::TextureView,
        camera: &Camera,
        gpu_context: &mut GPUContext,
        asset_manager: &AssetManager,
        renderer: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        match interface_mode {
            InterfaceMode::Setup => (),
            InterfaceMode::Menu => (),
            InterfaceMode::Run => Self::render_run_mode(
                &surface_texture_view,
                camera,
                gpu_context,
                asset_manager,
                renderer,
                encoder,
            ),
        }
    }

    fn render_setup_mode() {}

    fn render_menu_mode() {}

    fn render_run_mode(
        surface_texture_view: &wgpu::TextureView,
        camera: &Camera,
        gpu_context: &mut GPUContext,
        asset_manager: &AssetManager,
        renderer: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        WorldRenderer::render(
            gpu_context,
            &surface_texture_view,
            &asset_manager.depth_texture_view,
            &camera.uniform_bind_group,
            asset_manager,
            &renderer.world_renderer,
            encoder,
        );

        PopulationRenderer::render(
            gpu_context,
            &surface_texture_view,
            &asset_manager.depth_texture_view,
            &camera.uniform_bind_group,
            asset_manager,
            &renderer.population_renderer,
            encoder,
        );

        DebugRenderer::render(
            &surface_texture_view,
            &asset_manager.depth_texture_view,
            &renderer.debug_renderer,
            encoder,
        );
    }
}
