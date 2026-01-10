pub mod block_renderer;
pub mod debug_renderer;
pub mod overlay_renderer;
pub mod person_renderer;
pub mod render_mode;
pub mod sector_renderer;

use crate::interface::{
    asset_manager::AssetManager,
    camera::Camera,
    gpu::gpu_context::GPUContext,
    interface_mode::InterfaceMode,
    renderer::{
        block_renderer::BlockRenderer, debug_renderer::DebugRenderer,
        overlay_renderer::OverlayRenderer, person_renderer::PersonRenderer,
        render_mode::RenderMode, sector_renderer::SectorRenderer,
    },
};

pub struct Renderer {
    pub block_renderer: BlockRenderer,
    pub person_renderer: PersonRenderer,
    pub sector_renderer: SectorRenderer,
    pub overlay_renderer: OverlayRenderer,
    pub debug_renderer: DebugRenderer,
}

impl Renderer {
    pub fn new(
        gpu_context: &GPUContext,
        surface_format: &wgpu::TextureFormat,
        camera: &Camera,
    ) -> Self {
        let overlay_renderer = OverlayRenderer::new(gpu_context, surface_format);
        let sector_renderer = SectorRenderer::new(gpu_context, camera);
        let block_renderer = BlockRenderer::new(gpu_context, camera);
        let person_renderer = PersonRenderer::new(gpu_context, camera);
        let debug_renderer = DebugRenderer::new(gpu_context, camera);

        Self {
            overlay_renderer,
            sector_renderer,
            block_renderer,
            person_renderer,
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
            &renderer.sector_renderer.bind_group_layout,
        );

        renderer.sector_renderer.bind_group = Some(sector_renderer_bind_group);

        let person_renderer_bind_group = PersonRenderer::setup_bind_group(
            gpu_context,
            asset_manager,
            &renderer.person_renderer.bind_group_layout,
        );

        renderer.person_renderer.bind_group = Some(person_renderer_bind_group);
    }

    pub fn render(
        interface_mode: &InterfaceMode,
        surface_texture_view: &wgpu::TextureView,
        camera: &Camera,
        gpu_context: &mut GPUContext,
        asset_manager: &AssetManager,
        renderer: &mut Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        match interface_mode {
            InterfaceMode::Setup => Self::render_setup_mode(
                &surface_texture_view,
                gpu_context,
                asset_manager,
                renderer,
                encoder,
            ),
            InterfaceMode::Menu => Self::render_menu_mode(
                &surface_texture_view,
                gpu_context,
                asset_manager,
                renderer,
                encoder,
            ),
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

    fn render_setup_mode(
        surface_texture_view: &wgpu::TextureView,
        gpu_context: &mut GPUContext,
        asset_manager: &AssetManager,
        renderer: &mut Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        OverlayRenderer::render_setup_mode(
            &RenderMode::Load,
            surface_texture_view,
            gpu_context,
            &mut renderer.overlay_renderer,
            encoder,
        );
    }

    fn render_menu_mode(
        surface_texture_view: &wgpu::TextureView,
        gpu_context: &mut GPUContext,
        asset_manager: &AssetManager,
        renderer: &mut Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        OverlayRenderer::render_menu_mode(
            &RenderMode::Clear,
            surface_texture_view,
            gpu_context,
            &mut renderer.overlay_renderer,
            encoder,
        );
    }

    fn render_run_mode(
        surface_texture_view: &wgpu::TextureView,
        camera: &Camera,
        gpu_context: &mut GPUContext,
        asset_manager: &AssetManager,
        renderer: &mut Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        SectorRenderer::render(
            &RenderMode::Clear,
            surface_texture_view,
            &asset_manager.depth_texture_view,
            &camera.uniform_bind_group,
            asset_manager,
            &renderer.sector_renderer,
            encoder,
        );

        PersonRenderer::render(
            &RenderMode::Load,
            gpu_context,
            surface_texture_view,
            &asset_manager.depth_texture_view,
            &camera.uniform_bind_group,
            asset_manager,
            &renderer.person_renderer,
            encoder,
        );

        OverlayRenderer::render_run_mode(
            &RenderMode::Load,
            &surface_texture_view,
            gpu_context,
            &mut renderer.overlay_renderer,
            encoder,
        );

        DebugRenderer::render(
            &RenderMode::Load,
            &surface_texture_view,
            &asset_manager.depth_texture_view,
            &renderer.debug_renderer,
            encoder,
        );
    }
}
