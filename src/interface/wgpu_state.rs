//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

use std::sync::Arc;

pub struct WGPUState<'window> {
    pub window: Arc<winit::window::Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_texture_view_descriptor: wgpu::TextureViewDescriptor<'window>,
}
