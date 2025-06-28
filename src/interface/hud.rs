//! Information displayed over World rendering

pub mod mode;

pub use mode::Mode;

use crate::{
    interface::{
        gpu_context::GPUContext,
        hud::mode::{LoadData, MenuData, ShutdownData, SimulateData},
    },
    simulation::{self},
};
use egui::{FontId, FullOutput, Ui};
use glam::Vec2;

pub struct HUD {
    mode: Mode,
    action_vec: Vec<simulation::state::receiver::action::Action>,
    renderer: egui_wgpu::Renderer,
}

impl HUD {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let action_vec = Vec::new();

        let renderer = egui_wgpu::Renderer::new(device, surface_format, None, 1, false);

        let mode = Mode::Menu(mode::MenuData {
            message: "NO MESSAGE SET".to_string(),
        });

        Self {
            action_vec,
            renderer,
            mode,
        }
    }

    pub fn update(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        gpu_context: &mut GPUContext,
        texture_view: &wgpu::TextureView,
    ) {
        let raw_input = gpu_context
            .egui_winit_state
            .take_egui_input(&gpu_context.window_arc);

        let mut action_vec = std::mem::take(&mut self.action_vec);

        let full_output: FullOutput =
            gpu_context
                .egui_context
                .run(raw_input, |context| match &self.mode {
                    Mode::Load(load_data) => self.draw_load(context, &load_data, &mut action_vec),
                    Mode::Menu(menu_data) => self.draw_menu(context, &menu_data, &mut action_vec),
                    Mode::Simulate(simulate_data) => {
                        self.draw_simulate(context, &simulate_data, &mut action_vec)
                    }
                    Mode::Shutdown(shutdown_data) => {
                        self.draw_shutdown(context, &shutdown_data, &mut action_vec)
                    }
                });

        self.action_vec = action_vec;

        let paint_jobs = gpu_context
            .egui_context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: gpu_context.window_arc.inner_size().into(),
            pixels_per_point: gpu_context.window_arc.scale_factor() as f32,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(&gpu_context.device, &gpu_context.queue, *id, image_delta);
        }

        self.renderer.update_buffers(
            &gpu_context.device,
            &gpu_context.queue,
            encoder,
            &paint_jobs,
            &screen_descriptor,
        );

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

    pub fn get_actions(&mut self) -> Vec<simulation::state::receiver::action::Action> {
        std::mem::take(&mut self.action_vec)
    }

    pub fn prepare_menu(&mut self, view: &simulation::observation::view::View) {
        if let Mode::Menu(data) = &mut self.mode {
            data.message = view.admin_view.message.clone();
        }
    }

    pub fn prepare_load(&mut self, view: &simulation::observation::view::View) {
        if let Mode::Load(data) = &mut self.mode {
            data.message = view.admin_view.message.clone();
        }
    }

    pub fn prepare_simulate(&mut self, view: &simulation::observation::view::View) {
        if let Mode::Simulate(data) = &mut self.mode {
            data.message = view.admin_view.message.clone();
        }
    }

    pub fn prepare_shutdown(&mut self, view: &simulation::observation::view::View) {
        if let Mode::Shutdown(data) = &mut self.mode {
            data.message = view.admin_view.message.clone();
        }
    }

    fn draw_menu(
        &self,
        context: &egui::Context,
        _menu_data: &MenuData,
        action_vec: &mut Vec<simulation::state::receiver::action::Action>,
    ) {
        let mut start_clicked = false;

        egui::CentralPanel::default().show(context, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.4);
                start_clicked = ui.button("Start").clicked();
                // ui.label(&menu_data.message);
            });
        });

        println!("{:?}", start_clicked);

        if start_clicked {
            println!("CLICKED!");

            let admin_action = simulation::state::receiver::action::AdminAction::Start;
            let action = simulation::state::receiver::action::Action::Admin(admin_action);

            action_vec.push(action);
        }
    }

    fn draw_load(
        &self,
        context: &egui::Context,
        load_data: &LoadData,
        _action_vec: &mut Vec<simulation::state::receiver::action::Action>,
    ) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            Self::draw_hud_text(ui, Vec2::new(10.0, 10.0), &load_data.message);
        });
    }

    fn draw_simulate(
        &self,
        context: &egui::Context,
        simulate_data: &SimulateData,
        _action_vec: &mut Vec<simulation::state::receiver::action::Action>,
    ) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            Self::draw_hud_text(ui, Vec2::new(10.0, 10.0), &simulate_data.message);
        });
    }

    fn draw_shutdown(
        &self,
        context: &egui::Context,
        shutdown_data: &ShutdownData,
        _action_vec: &mut Vec<simulation::state::receiver::action::Action>,
    ) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            Self::draw_hud_text(ui, Vec2::new(10.0, 10.0), &shutdown_data.message);
        });
    }

    fn draw_hud_text(ui: &mut Ui, position: Vec2, text: &str) {
        ui.painter().text(
            egui::pos2(position.x - 1.0, position.y + 1.0),
            egui::Align2::LEFT_TOP,
            text,
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
