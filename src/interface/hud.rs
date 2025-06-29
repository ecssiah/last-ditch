//! Information displayed over World rendering

pub mod mode;

pub use mode::Mode;
use winit::event::{DeviceEvent, WindowEvent};

use crate::{
    interface::{
        gpu_context::GPUContext,
        hud::mode::{LoadData, MenuData, ShutdownData, SimulateData},
    },
    simulation::{self},
};
use egui::{FontId, FullOutput, Id, Ui};
use glam::Vec2;

pub struct HUD {
    mode: Mode,
    action_vec: Vec<simulation::state::receiver::action::Action>,
}

impl HUD {
    pub fn new() -> Self {
        let action_vec = Vec::new();
        let mode = Mode::Menu(mode::MenuData {
            message: "NO MESSAGE SET".to_string(),
        });

        Self { action_vec, mode }
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
                    Mode::Menu(menu_data) => self.draw_menu(context, &menu_data, &mut action_vec),
                    Mode::Load(load_data) => self.draw_load(context, &load_data, &mut action_vec),
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
            gpu_context.egui_renderer.update_texture(
                &gpu_context.device,
                &gpu_context.queue,
                *id,
                image_delta,
            );
        }

        gpu_context.egui_renderer.update_buffers(
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

        gpu_context.egui_renderer.render(
            &mut render_pass.forget_lifetime(),
            &paint_jobs,
            &screen_descriptor,
        )
    }

    pub fn get_actions(&mut self) -> Vec<simulation::state::receiver::action::Action> {
        std::mem::take(&mut self.action_vec)
    }

    pub fn prepare_menu(&mut self, view: &simulation::observation::view::View) {
        let menu_data = mode::MenuData {
            message: view.admin_view.message.clone(),
        };

        self.mode = Mode::Menu(menu_data);
    }

    pub fn prepare_load(&mut self, view: &simulation::observation::view::View) {
        let load_data = mode::LoadData {
            message: view.admin_view.message.clone(),
        };

        self.mode = Mode::Load(load_data);
    }

    pub fn prepare_simulate(&mut self, view: &simulation::observation::view::View) {
        let simulate_data = mode::SimulateData {
            message: view.admin_view.message.clone(),
        };

        self.mode = Mode::Simulate(simulate_data);
    }

    pub fn prepare_shutdown(&mut self, view: &simulation::observation::view::View) {
        let shutdown_data = mode::ShutdownData {
            message: view.admin_view.message.clone(),
        };

        self.mode = Mode::Shutdown(shutdown_data);
    }

    fn draw_menu(
        &self,
        context: &egui::Context,
        _menu_data: &MenuData,
        action_vec: &mut Vec<simulation::state::receiver::action::Action>,
    ) {
        let mut start_clicked = false;
        let mut exit_clicked = false;

        egui::CentralPanel::default().show(context, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.4);

                start_clicked = ui
                    .add_sized([200.0, 60.0], egui::Button::new("Start"))
                    .clicked();

                exit_clicked = ui
                    .add_sized([200.0, 60.0], egui::Button::new("Exit"))
                    .clicked();
            });
        });

        if start_clicked {
            let admin_action = simulation::state::receiver::action::AdminAction::Start;
            let action = simulation::state::receiver::action::Action::Admin(admin_action);

            action_vec.push(action);
        }

        if exit_clicked {
            let admin_action = simulation::state::receiver::action::AdminAction::Shutdown;
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
        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
            .show(context, |ui| {
                Self::draw_hud_text(ui, Vec2::new(6.0, 6.0), &load_data.message);
            });
    }

    fn draw_simulate(
        &self,
        context: &egui::Context,
        simulate_data: &SimulateData,
        _action_vec: &mut Vec<simulation::state::receiver::action::Action>,
    ) {
        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
            .show(context, |ui| {
                Self::draw_hud_text(ui, Vec2::new(6.0, 6.0), &simulate_data.message);
            });
    }

    fn draw_shutdown(
        &self,
        context: &egui::Context,
        shutdown_data: &ShutdownData,
        _action_vec: &mut Vec<simulation::state::receiver::action::Action>,
    ) {
        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
            .show(context, |ui| {
                Self::draw_hud_text(ui, Vec2::new(6.0, 6.0), &shutdown_data.message);
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

    pub fn handle_device_event(
        &mut self,
        event: &DeviceEvent,
        gpu_context: &mut GPUContext,
    ) -> bool {
        match &self.mode {
            Mode::Menu(_) => {
                if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
                    gpu_context.egui_winit_state.on_mouse_motion((*dx, *dy))
                };

                true
            }
            Mode::Load(_) => {
                if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
                    gpu_context.egui_winit_state.on_mouse_motion((*dx, *dy))
                };

                true
            }
            Mode::Simulate(_) => false,
            Mode::Shutdown(_) => false,
        }
    }

    pub fn handle_window_event(
        &mut self,
        event: &WindowEvent,
        gpu_context: &mut GPUContext,
    ) -> bool {
        match &self.mode {
            Mode::Menu(_) => {
                let _event_response = gpu_context
                    .egui_winit_state
                    .on_window_event(&gpu_context.window_arc, event);

                true
            }
            Mode::Load(_) => {
                let _event_response = gpu_context
                    .egui_winit_state
                    .on_window_event(&gpu_context.window_arc, event);

                true
            }
            Mode::Simulate(_) => false,
            Mode::Shutdown(_) => false,
        }
    }
}
