//! Information displayed over World rendering

pub mod mode;

pub use mode::Mode;

use crate::{
    interface::{
        gpu_context::GPUContext,
        hud::mode::{LoadData, MenuData, ShutdownData, SimulateData},
    },
    simulation::{self, observation::view::View, state::receiver::action::Action},
};
use egui::{FontId, FullOutput, Id, Ui};
use glam::Vec2;
use std::sync::Arc;
use winit::event::{DeviceEvent, WindowEvent};

pub struct HUD {
    pub mode: Mode,
    pub action_vec: Vec<Action>,
}

impl HUD {
    pub fn new() -> Self {
        let action_vec = Vec::new();
        let mode = Mode::Menu(mode::MenuData {
            message: "NO MESSAGE SET".to_string(),
        });

        Self { action_vec, mode }
    }

    pub fn get_action_vec(action_vec: &mut Vec<Action>) -> Vec<Action> {
        std::mem::take(action_vec)
    }

    pub fn get_full_output(
        window_arc: Arc<winit::window::Window>,
        mode: &Mode,
        egui_context: &egui::Context,
        egui_winit_state: &mut egui_winit::State,
        action_vec: &mut Vec<Action>,
    ) -> FullOutput {
        let raw_input = egui_winit_state.take_egui_input(&window_arc);

        let mut action_vec_ = std::mem::take(action_vec);

        let full_output: FullOutput = egui_context.run(raw_input, |context| match &mode {
            Mode::Menu(menu_data) => Self::draw_menu(context, menu_data, &mut action_vec_),
            Mode::Load(load_data) => Self::draw_load(context, load_data, &mut action_vec_),
            Mode::Simulate(simulate_data) => {
                Self::draw_simulate(context, simulate_data, &mut action_vec_)
            }
            Mode::Shutdown(shutdown_data) => {
                Self::draw_shutdown(context, shutdown_data, &mut action_vec_)
            }
        });

        *action_vec = action_vec_;

        full_output
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        window_arc: Arc<winit::window::Window>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        egui_context: &egui::Context,
        hud: &mut HUD,
        egui_winit_state: &mut egui_winit::State,
        egui_renderer: &mut egui_wgpu::Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let full_output = Self::get_full_output(
            Arc::clone(&window_arc),
            &hud.mode,
            egui_context,
            egui_winit_state,
            &mut hud.action_vec,
        );

        let paint_jobs = egui_context.tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: window_arc.inner_size().into(),
            pixels_per_point: window_arc.scale_factor() as f32,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            egui_renderer.update_texture(device, queue, *id, image_delta);
        }

        egui_renderer.update_buffers(device, queue, encoder, &paint_jobs, &screen_descriptor);

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("EGUI Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_texture_view,
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

        egui_renderer.render(
            &mut render_pass.forget_lifetime(),
            &paint_jobs,
            &screen_descriptor,
        )
    }

    pub fn apply_menu_view(view: &View, mode: &mut Mode) {
        let menu_data = mode::MenuData {
            message: view.admin_view.message.clone(),
        };

        *mode = Mode::Menu(menu_data);
    }

    pub fn apply_load_view(view: &View, mode: &mut Mode) {
        let load_data = mode::LoadData {
            message: view.admin_view.message.clone(),
        };

        *mode = Mode::Load(load_data);
    }

    pub fn apply_simulate_view(view: &View, mode: &mut Mode) {
        let judge_view = &view.population_view.judge_view;

        let position_string = format!(
            "Cell: ({:.0}, {:.0}, {:.0})\n",
            judge_view.position.x, judge_view.position.y, judge_view.position.z,
        );

        let world_position_string = format!(
            "World: ({:.2}, {:.2}, {:.2})\n",
            judge_view.world_position.x, judge_view.world_position.y, judge_view.world_position.z,
        );

        let sector_string = format!(
            "Sector: ({:.0}, {:.0}, {:.0}) ID {:?}\n",
            judge_view.sector_coordinates.x,
            judge_view.sector_coordinates.y,
            judge_view.sector_coordinates.z,
            usize::from(judge_view.sector_id),
        );

        let mut message = String::new();
        message.push_str(&position_string);
        message.push_str(&world_position_string);
        message.push_str(&sector_string);

        let simulate_data = mode::SimulateData { message };

        *mode = Mode::Simulate(simulate_data);
    }

    pub fn apply_shutdown_view(view: &View, mode: &mut Mode) {
        let shutdown_data = mode::ShutdownData {
            message: view.admin_view.message.clone(),
        };

        *mode = Mode::Shutdown(shutdown_data);
    }

    fn draw_menu(context: &egui::Context, _menu_data: &MenuData, action_vec: &mut Vec<Action>) {
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
            let action = Action::Admin(admin_action);

            action_vec.push(action);
        }

        if exit_clicked {
            let admin_action = simulation::state::receiver::action::AdminAction::Quit;
            let action = Action::Admin(admin_action);

            action_vec.push(action);
        }
    }

    fn draw_load(context: &egui::Context, load_data: &LoadData, _action_vec: &mut Vec<Action>) {
        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
            .show(context, |ui| {
                Self::draw_hud_text(ui, Vec2::new(6.0, 6.0), &load_data.message);
            });
    }

    fn draw_simulate(
        context: &egui::Context,
        simulate_data: &SimulateData,
        _action_vec: &mut Vec<Action>,
    ) {
        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
            .show(context, |ui| {
                Self::draw_hud_text(ui, Vec2::new(6.0, 6.0), &simulate_data.message);
            });
    }

    fn draw_shutdown(
        context: &egui::Context,
        shutdown_data: &ShutdownData,
        _action_vec: &mut Vec<Action>,
    ) {
        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
            .show(context, |ui| {
                Self::draw_hud_text(ui, Vec2::new(6.0, 6.0), &shutdown_data.message);
            });
    }

    fn draw_hud_text(ui: &mut Ui, position: Vec2, text: &str) {
        let font_size = 16.0;

        ui.painter().text(
            egui::pos2(position.x - 1.0, position.y + 1.0),
            egui::Align2::LEFT_TOP,
            text,
            FontId::proportional(font_size),
            egui::Color32::BLACK,
        );

        ui.painter().text(
            egui::pos2(position.x, position.y),
            egui::Align2::LEFT_TOP,
            text,
            FontId::proportional(font_size),
            egui::Color32::WHITE,
        );
    }

    pub fn handle_device_event(
        event: &DeviceEvent,
        mode: &Mode,
        gpu_context: &mut GPUContext,
    ) -> bool {
        match mode {
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
        event: &WindowEvent,
        mode: &Mode,
        gpu_context: &mut GPUContext,
    ) -> bool {
        match mode {
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
