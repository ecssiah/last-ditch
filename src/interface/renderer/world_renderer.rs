pub mod block_renderer;
pub mod sector_renderer;

use crate::{
    interface::{
        asset_manager::AssetManager,
        camera::Camera,
        gpu::gpu_context::GPUContext,
        renderer::world_renderer::{
            block_renderer::BlockRenderer, sector_renderer::SectorRenderer,
        },
    },
    simulation::supervisor::viewer::view::WorldView,
};

pub struct WorldRenderer {
    pub block_renderer: BlockRenderer,
    pub sector_renderer: SectorRenderer,
}

impl WorldRenderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let block_renderer = BlockRenderer::new(gpu_context, camera);
        let sector_renderer = SectorRenderer::new(gpu_context, camera);

        Self {
            block_renderer,
            sector_renderer,
        }
    }

    pub fn apply_world_view(
        gpu_context: &GPUContext,
        camera: &Camera,
        asset_manager: &AssetManager,
        world_view: &WorldView,
        world_renderer: &mut Self,
    ) {
        BlockRenderer::apply_world_view(
            gpu_context,
            camera,
            asset_manager,
            world_view,
            &mut world_renderer.block_renderer,
        );

        SectorRenderer::apply_world_view(
            gpu_context,
            camera,
            asset_manager,
            world_view,
            &mut world_renderer.sector_renderer,
        );
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        asset_manager: &AssetManager,
        world_renderer: &Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        BlockRenderer::render(
            surface_texture_view,
            depth_texture_view,
            camera_uniform_bind_group,
            asset_manager,
            &world_renderer.block_renderer,
            encoder,
        );

        SectorRenderer::render(
            surface_texture_view,
            depth_texture_view,
            camera_uniform_bind_group,
            asset_manager,
            &world_renderer.sector_renderer,
            encoder,
        );

    }
}
