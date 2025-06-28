//! WebGPU Interface

use std::sync::Arc;

pub struct GPUContext<'window> {
    pub window_arc: Arc<winit::window::Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub texture_view_descriptor: wgpu::TextureViewDescriptor<'window>,
    pub egui_context: egui::Context,
    pub egui_winit_state: egui_winit::State,
}
