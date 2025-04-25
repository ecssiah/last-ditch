pub mod load_data;
pub mod mode;
pub mod simulate_data;

pub use mode::Mode;

use egui::{FontId, FullOutput, Ui, ViewportId};
use glam::Vec2;
use std::sync::Arc;

use crate::{
    interface::hud::{load_data::LoadData, simulate_data::SimulateData},
    simulation::{self, observation::view::admin_view},
};

pub struct HUD {
    context: egui::Context,
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
    mode: Mode,
}

impl HUD {
    pub fn new(
        device: &wgpu::Device,
        window: Arc<winit::window::Window>,
        surface_format: wgpu::TextureFormat,
    ) -> HUD {
        let context = egui::Context::default();

        let state =
            egui_winit::State::new(context.clone(), ViewportId::ROOT, &window, None, None, None);

        let renderer = egui_wgpu::Renderer::new(device, surface_format, None, 1, false);

        let mode = Mode::Load(LoadData {
            message: "No Message".to_string(),
        });

        let hud = HUD {
            context,
            state,
            renderer,
            mode,
        };

        hud
    }

    pub fn update(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        window: &winit::window::Window,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_view: &wgpu::TextureView,
    ) {
        let raw_input = self.state.take_egui_input(window);

        let full_output: FullOutput = self.context.run(raw_input, |context| match &self.mode {
            Mode::Load(data) => self.draw_load(context, data),
            Mode::Simulate(data) => self.draw_simulate(context, data),
        });

        let paint_jobs = self
            .context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: window.inner_size().into(),
            pixels_per_point: window.scale_factor() as f32,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        self.renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &screen_descriptor);

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("EGUI Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        self.renderer.render(
            &mut render_pass.forget_lifetime(),
            &paint_jobs,
            &screen_descriptor,
        )
    }

    pub fn prepare_load(&mut self, admin_view: &simulation::observation::view::AdminView) {
        if let Mode::Load(data) = &mut self.mode {
            data.message = admin_view.message.clone();
        }
    }

    fn draw_load(&self, context: &egui::Context, data: &LoadData) {
        egui::Area::new(egui::Id::new(0)).show(context, |ui| {
            Self::draw_hud_text(ui, Vec2::new(10.0, 10.0), data.message.to_owned());
        });
    }

    fn draw_simulate(&self, _context: &egui::Context, data: &SimulateData) {}

    fn draw_hud_text(ui: &mut Ui, position: Vec2, text: String) {
        ui.painter().text(
            egui::pos2(position.x - 1.0, position.y + 1.0),
            egui::Align2::LEFT_TOP,
            text.clone(),
            FontId::proportional(22.0),
            egui::Color32::BLACK,
        );

        ui.painter().text(
            egui::pos2(position.x, position.y),
            egui::Align2::LEFT_TOP,
            text,
            FontId::proportional(22.0),
            egui::Color32::WHITE,
        );
    }
}
