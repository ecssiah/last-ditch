//! Information displayed over World rendering

pub mod model;

pub use model::Model;

use crate::{
    interface::gpu::gpu_context::GPUContext,
    simulation::{
        constants::ID_JUDGE_1,
        manager::{message::SeedData, viewer::view::View, Message},
        state::world::grid::{self, Direction},
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

    fn get_full_output(
        window_arc: Arc<winit::window::Window>,
        egui_context: &egui::Context,
        egui_winit_state: &mut egui_winit::State,
        gui: &mut Self,
    ) -> FullOutput {
        let raw_input = egui_winit_state.take_egui_input(&window_arc);

        let mut gui_work = std::mem::take(gui);

        let full_output: FullOutput = egui_context.run(raw_input, |context| {
            Self::show(context, &mut gui_work);
        });

        *gui = gui_work;

        full_output
    }

    fn show(context: &egui::Context, gui: &mut Self) {
        Self::show_hud(context, gui);
        Self::show_menu(context, gui);
    }

    fn show_menu(context: &egui::Context, gui: &mut Self) {
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
                                    .desired_width(120.0)
                                    .horizontal_align(egui::Align::Center),
                            );

                            ui.add_space(20.0);

                            let generate_clicked = ui
                                .add_sized([200.0, 60.0], egui::Button::new("Generate"))
                                .clicked();

                            if generate_clicked {
                                let seed_data = SeedData {
                                    seed: Self::parse_seed(&gui.model.seed_input_string),
                                };

                                gui.message_deque.push_back(Message::SetSeed(seed_data));
                                gui.message_deque.push_back(Message::Generate);
                            }

                            ui.add_space(20.0);

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

    fn show_hud(context: &egui::Context, gui: &mut Self) {
        if gui.menu_active {
            return;
        }

        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(context, |ui| {
                Self::show_crosshair(ui);
            });

        if gui.model.info_message_vec.len() > 0 {
            let info_message = &gui.model.info_message_vec[0];

            egui::Area::new(Id::new(1))
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
                .show(context, |ui| {
                    Self::show_hud_text(ui, Vec2::new(6.0, 6.0), info_message);
                });
        }
    }

    fn show_crosshair(ui: &mut Ui) {
        let rect = ui.max_rect();
        let center = rect.center();

        let length = 10.0;
        let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);

        ui.painter().line_segment(
            [
                center + egui::vec2(-length, 0.0),
                center + egui::vec2(length, 0.0),
            ],
            stroke,
        );

        ui.painter().line_segment(
            [
                center + egui::vec2(0.0, -length),
                center + egui::vec2(0.0, length),
            ],
            stroke,
        );
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

    pub fn apply_view(view: &View, gui: &mut Self) {
        if let Some(person_view) = view.population_view.person_view_map.get(&ID_JUDGE_1) {
            let grid_position =
                grid::world_position_to_grid_position(person_view.transform.world_position);

            let position_string = format!(
                "Cell: ({:.0}, {:.0}, {:.0})\n",
                grid_position.x, grid_position.y, grid_position.z,
            );

            let world_position_string = format!(
                "World: ({:.2}, {:.2}, {:.2})\n",
                person_view.transform.world_position.x,
                person_view.transform.world_position.y,
                person_view.transform.world_position.z,
            );

            let sector_coordinate =
                grid::world_position_to_sector_coordinate(person_view.transform.world_position);

            let sector_string = format!(
                "Sector: ({:.0}, {:.0}, {:.0})\n",
                sector_coordinate.x, sector_coordinate.y, sector_coordinate.z,
            );

            let direction_string = format!(
                "Direction: {:?}\n",
                Direction::from_rotation(person_view.transform.rotation_xy)
            );

            let selected_block_kind_string =
                format!("Selected Block: {:?}\n", person_view.selected_block_kind);

            let mut info_message = String::new();
            info_message.push_str(&position_string);
            info_message.push_str(&world_position_string);
            info_message.push_str(&sector_string);
            info_message.push_str(&direction_string);
            info_message.push_str(&selected_block_kind_string);

            gui.model.info_message_vec.clear();
            gui.model.info_message_vec.push(info_message);
        }
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        window_arc: Arc<winit::window::Window>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        egui_context: &egui::Context,
        gui: &mut Self,
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

    pub fn set_menu_active(menu_active: bool, gui: &mut Self, gpu_context: &mut GPUContext) {
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

    pub fn toggle_menu_active(gui: &mut Self, gpu_context: &mut GPUContext) {
        Self::set_menu_active(!gui.menu_active, gui, gpu_context);
    }

    fn parse_seed(seed_string: &str) -> u64 {
        let mut hasher = DefaultHasher::new();

        seed_string.hash(&mut hasher);

        let seed = hasher.finish();

        tracing::info!("Seed: {:?}", seed);

        seed
    }

    pub fn get_message_deque(message_deque: &mut VecDeque<Message>) -> VecDeque<Message> {
        std::mem::take(message_deque)
    }
}
