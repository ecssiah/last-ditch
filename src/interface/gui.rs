//! Information displayed over World rendering

pub mod model;

pub use model::Model;

use crate::{
    interface::gpu::gpu_context::GPUContext,
    simulation::{
        manager::{message::GenerateData, Message},
        viewer::View,
    },
};
use egui::{FontId, FullOutput, Id, Ui};
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
use ultraviolet::Vec2;
use winit::event::{DeviceEvent, WindowEvent};

#[derive(Default)]
pub struct GUI {
    pub menu_active: bool,
    pub model: Model,
    pub message_deque: VecDeque<Message>,
}

impl GUI {
    pub fn new() -> Self {
        let menu_active = true;
        let model = Model::new();
        let message_deque = VecDeque::new();

        Self {
            menu_active,
            model,
            message_deque,
        }
    }

    pub fn get_message_deque(message_deque: &mut VecDeque<Message>) -> VecDeque<Message> {
        std::mem::take(message_deque)
    }

    pub fn get_full_output(
        window_arc: Arc<winit::window::Window>,
        egui_context: &egui::Context,
        egui_winit_state: &mut egui_winit::State,
        gui: &mut GUI,
    ) -> FullOutput {
        let raw_input = egui_winit_state.take_egui_input(&window_arc);

        let mut gui_working = std::mem::take(gui);

        let full_output: FullOutput = egui_context.run(raw_input, |context| {
            Self::show(context, &mut gui_working);
        });

        *gui = gui_working;

        full_output
    }

    pub fn show(context: &egui::Context, gui: &mut GUI) {
        Self::show_menu(context, gui);
        Self::show_hud(context, gui);
    }

    fn show_menu(context: &egui::Context, gui: &mut GUI) {
        if !gui.menu_active {
            return;
        }

        let screen_rect = context.available_rect();
        let width = screen_rect.width() * 0.9;
        let height = screen_rect.height() * 0.9;

        egui::Area::new(egui::Id::new("main_menu_area"))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(context, |ui| {
                egui::Frame::window(&context.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 20, 230))
                    .corner_radius(5.0)
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(width, height));

                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() * 0.3);

                            ui.label("Seed:");

                            ui.add(
                                egui::TextEdit::singleline(&mut gui.model.seed_input_string)
                                    .desired_width(ui.available_width() * 0.1)
                                    .horizontal_align(egui::Align::Center),
                            );

                            ui.add_space(ui.available_height() * 0.1);

                            let generate_clicked = ui
                                .add_sized([200.0, 60.0], egui::Button::new("Generate"))
                                .clicked();

                            if generate_clicked {
                                let generate_data = GenerateData {
                                    seed: Self::parse_seed(&gui.model.seed_input_string),
                                };

                                gui.message_deque
                                    .push_back(Message::Generate(generate_data));
                            }

                            ui.add_space(ui.available_height() * 0.1);

                            let quit_clicked = ui
                                .add_sized([200.0, 60.0], egui::Button::new("Quit"))
                                .clicked();

                            if quit_clicked {
                                gui.message_deque.push_back(Message::Quit);
                            }
                        });
                    });
            });
    }

    fn parse_seed(seed_string: &str) -> u64 {
        let mut hasher = DefaultHasher::new();

        seed_string.hash(&mut hasher);

        let seed = hasher.finish();

        tracing::info!("Seed: {:?}", seed);

        seed
    }

    fn show_hud(context: &egui::Context, gui: &mut GUI) {
        if gui.menu_active {
            return;
        }

        if gui.model.info_message_vec.len() > 0 {
            let info_message = &gui.model.info_message_vec[0];

            egui::Area::new(Id::new(0))
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
                .show(context, |ui| {
                    Self::show_hud_text(ui, Vec2::new(6.0, 6.0), info_message);
                });
        }
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        window_arc: Arc<winit::window::Window>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        egui_context: &egui::Context,
        gui: &mut GUI,
        egui_winit_state: &mut egui_winit::State,
        egui_renderer: &mut egui_wgpu::Renderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let full_output =
            Self::get_full_output(Arc::clone(&window_arc), egui_context, egui_winit_state, gui);

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

    pub fn apply_view(view: &View, gui: &mut GUI) {
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
            judge_view.sector_id.to_usize(),
        );

        let selected_block_kind_string =
            format!("Selected Block: {:?}\n", judge_view.selected_block_kind);

        let mut info_message = String::new();
        info_message.push_str(&position_string);
        info_message.push_str(&world_position_string);
        info_message.push_str(&sector_string);
        info_message.push_str(&selected_block_kind_string);

        gui.model.info_message_vec.clear();
        gui.model.info_message_vec.push(info_message);
    }

    fn show_hud_text(ui: &mut Ui, position: Vec2, text: &str) {
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

    pub fn handle_device_event(event: &DeviceEvent, gpu_context: &mut GPUContext) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            gpu_context.egui_winit_state.on_mouse_motion((*dx, *dy))
        };
    }

    pub fn handle_window_event(event: &WindowEvent, gpu_context: &mut GPUContext) -> bool {
        let event_response = gpu_context
            .egui_winit_state
            .on_window_event(&gpu_context.window_arc, event);

        event_response.consumed
    }

    pub fn set_menu_active(menu_active: bool, gui: &mut GUI, gpu_context: &mut GPUContext) {
        gui.menu_active = menu_active;

        if menu_active {
            gpu_context.window_arc.set_cursor_visible(true);

            gpu_context
                .window_arc
                .set_cursor_grab(winit::window::CursorGrabMode::None)
                .expect("Failed to grab cursor");
        } else {
            gpu_context.window_arc.set_cursor_visible(false);

            gpu_context
                .window_arc
                .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                .expect("Failed to grab cursor");
        }
    }

    pub fn toggle_menu_active(gui: &mut GUI, gpu_context: &mut GPUContext) {
        Self::set_menu_active(!gui.menu_active, gui, gpu_context);
    }
}
