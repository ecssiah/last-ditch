//! Information displayed over World rendering

pub mod content;

use crate::{
    interface::{
        gpu::gpu_context::GPUContext,
        interface_mode::InterfaceMode,
        renderer::{overlay_renderer::content::Content, render_mode::RenderMode},
    },
    simulation::{
        state::{
            population::person::person_id::PersonID,
            world::grid::{self, Direction},
        },
        supervisor::{message::SeedData, viewer::view::View, Message},
    },
};
use egui::{FontId, FullOutput, Id, Ui};
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
use tracing::instrument;
use ultraviolet::Vec2;
use winit::event::{DeviceEvent, WindowEvent};

pub struct OverlayRenderer {
    pub content: Content,
    pub message_deque: VecDeque<Message>,
    pub egui_context: egui::Context,
    pub egui_winit_state: egui_winit::State,
    pub egui_renderer: egui_wgpu::Renderer,
}

impl OverlayRenderer {
    pub fn new(gpu_context: &GPUContext, surface_format: &wgpu::TextureFormat) -> Self {
        let egui_context = egui::Context::default();

        let egui_winit_state = egui_winit::State::new(
            egui_context.clone(),
            egui::ViewportId::ROOT,
            &gpu_context.window_arc,
            None,
            None,
            None,
        );

        let egui_renderer =
            egui_wgpu::Renderer::new(&gpu_context.device, surface_format.clone(), None, 1, false);

        let content = Content::new();
        let message_deque = VecDeque::new();

        Self {
            content,
            message_deque,
            egui_context,
            egui_winit_state,
            egui_renderer,
        }
    }

    fn get_full_output(
        interface_mode: &InterfaceMode,
        window_arc: Arc<winit::window::Window>,
        overlay_renderer: &mut Self,
    ) -> FullOutput {
        let raw_input = overlay_renderer
            .egui_winit_state
            .take_egui_input(&window_arc);

        let mut content_working = std::mem::take(&mut overlay_renderer.content);
        let mut message_deque = std::mem::take(&mut overlay_renderer.message_deque);

        let full_output: FullOutput = overlay_renderer.egui_context.run(raw_input, |context| {
            Self::show(
                context,
                interface_mode,
                &mut content_working,
                &mut message_deque,
            );
        });

        overlay_renderer.content = content_working;

        full_output
    }

    fn show(
        context: &egui::Context,
        interface_mode: &InterfaceMode,
        content: &mut Content,
        message_deque: &mut VecDeque<Message>,
    ) {
        match interface_mode {
            InterfaceMode::Setup => Self::show_setup(context, content),
            InterfaceMode::Menu => {
                Self::show_menu(context, content, message_deque);
            }
            InterfaceMode::Run => {
                Self::show_hud(context, content);
                Self::show_main_window(context, content);
            }
        }
    }

    fn show_setup(context: &egui::Context, content: &mut Content) {
        let screen_rect = context.available_rect();

        let width = screen_rect.width() * 1.0;

        egui::Area::new(egui::Id::new("setup_area"))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(context, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Last Ditch")
                            .size(64.0)
                            .monospace()
                            .strong(),
                    );

                    ui.label(egui::RichText::new("Just Sky").size(24.0));

                    ui.add_space(24.0);

                    let progress_bar_height = 36.0;

                    ui.add_sized(
                        egui::vec2(width * 0.3, progress_bar_height * 1.2),
                        egui::ProgressBar::new(content.setup_content.progress)
                            .desired_height(progress_bar_height),
                    );

                    ui.add_space(12.0);

                    ui.label(
                        egui::RichText::new(&content.setup_content.loading_string)
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                    )
                });
            });
    }

    fn show_menu(
        context: &egui::Context,
        content: &mut Content,
        message_deque: &mut VecDeque<Message>,
    ) {
        let screen_rect = context.available_rect();

        let width = screen_rect.width() * 0.9;
        let height = screen_rect.height() * 0.9;

        egui::Area::new(egui::Id::new("menu_area"))
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
                                egui::TextEdit::singleline(
                                    &mut content.menu_content.seed_input_string,
                                )
                                .desired_width(120.0)
                                .horizontal_align(egui::Align::Center),
                            );

                            ui.add_space(20.0);

                            let generate_clicked = ui
                                .add_sized([200.0, 60.0], egui::Button::new("Generate"))
                                .clicked();

                            if generate_clicked {
                                let seed_data = SeedData {
                                    seed: Self::parse_seed(&content.menu_content.seed_input_string),
                                };

                                message_deque.push_back(Message::SetSeed(seed_data));

                                message_deque.push_back(Message::Generate);
                            }

                            ui.add_space(20.0);

                            let quit_clicked = ui
                                .add_sized([200.0, 60.0], egui::Button::new("Quit"))
                                .clicked();

                            if quit_clicked {
                                message_deque.push_back(Message::Quit);
                            }
                        });
                    });
            });
    }

    fn show_main_window(context: &egui::Context, content: &mut Content) {
        if !content.run_content.main_window_active {
            return;
        }

        let screen_rect = context.available_rect();

        let width = screen_rect.width() * 0.9;
        let height = screen_rect.height() * 0.9;

        egui::Area::new(egui::Id::new("main_window_area"))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(context, |ui| {
                egui::Frame::window(&context.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 20, 230))
                    .corner_radius(5.0)
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(width, height));

                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() * 0.3);

                            ui.label("Rules");
                        });
                    });
            });
    }

    fn show_hud(context: &egui::Context, content: &mut Content) {
        if content.run_content.main_window_active {
            return;
        }

        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(context, |ui| {
                Self::show_crosshair(ui);
            });

        if content.run_content.info_message_vec.len() > 0 {
            let info_message = &content.run_content.info_message_vec[0];

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

    pub fn apply_view_setup_mode(_view: &View, _overlay_renderer: &mut Self) {}

    pub fn apply_view_menu_mode(_view: &View, _overlay_renderer: &mut Self) {}

    pub fn apply_view_run_mode(view: &View, overlay_renderer: &mut Self) {
        if view
            .population_view
            .person_view_map
            .get(&PersonID::JUDGE_ID_1)
            .is_none()
        {
            return;
        }

        let judge_person_view = view
            .population_view
            .person_view_map
            .get(&PersonID::JUDGE_ID_1)
            .expect("Judge 1 does not exist in Run Mode");

        let grid_position =
            grid::world_position_to_grid_position(judge_person_view.transform.world_position);

        let position_string = format!(
            "Cell: ({:.0}, {:.0}, {:.0})\n",
            grid_position.x, grid_position.y, grid_position.z,
        );

        let world_position_string = format!(
            "World: ({:.2}, {:.2}, {:.2})\n",
            judge_person_view.transform.world_position.x,
            judge_person_view.transform.world_position.y,
            judge_person_view.transform.world_position.z,
        );

        let sector_coordinate =
            grid::world_position_to_sector_coordinate(judge_person_view.transform.world_position);

        let sector_string = format!(
            "Sector: ({:.0}, {:.0}, {:.0})\n",
            sector_coordinate.x, sector_coordinate.y, sector_coordinate.z,
        );

        let direction_string = format!(
            "Direction: {:?}\n",
            Direction::from_rotation(judge_person_view.transform.rotation_xy)
        );

        let contact_set_string = format!(
            "Contact Set: {}\n",
            judge_person_view.body.contact_set.to_string()
        );

        let motion_mode_string = format!(
            "Motion Mode: {}\n",
            judge_person_view.motion.mode.to_string()
        );

        let selected_block_kind_string = format!(
            "Selected Block: {:?}\n",
            judge_person_view.selected_block_kind
        );

        let mut info_message = String::new();

        info_message.push_str(&position_string);
        info_message.push_str(&world_position_string);
        info_message.push_str(&sector_string);
        info_message.push_str(&direction_string);
        info_message.push_str(&contact_set_string);
        info_message.push_str(&motion_mode_string);
        info_message.push_str(&selected_block_kind_string);

        overlay_renderer
            .content
            .run_content
            .info_message_vec
            .clear();
        overlay_renderer
            .content
            .run_content
            .info_message_vec
            .push(info_message);
    }

    #[instrument(skip_all)]
    pub fn render_setup_mode(
        render_mode: &RenderMode,
        surface_texture_view: &wgpu::TextureView,
        gpu_context: &mut GPUContext,
        overlay_renderer: &mut Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let full_output = Self::get_full_output(
            &InterfaceMode::Setup,
            gpu_context.window_arc.clone(),
            overlay_renderer,
        );

        let paint_jobs = overlay_renderer
            .egui_context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: gpu_context.window_arc.inner_size().into(),
            pixels_per_point: gpu_context.window_arc.scale_factor() as f32,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            overlay_renderer.egui_renderer.update_texture(
                &gpu_context.device,
                &gpu_context.queue,
                *id,
                image_delta,
            );
        }

        overlay_renderer.egui_renderer.update_buffers(
            &gpu_context.device,
            &gpu_context.queue,
            encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: surface_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: RenderMode::get_load_op(render_mode),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("EGUI Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        overlay_renderer.egui_renderer.render(
            &mut render_pass.forget_lifetime(),
            &paint_jobs,
            &screen_descriptor,
        )
    }

    #[instrument(skip_all)]
    pub fn render_menu_mode(
        render_mode: &RenderMode,
        surface_texture_view: &wgpu::TextureView,
        gpu_context: &mut GPUContext,
        overlay_renderer: &mut Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let full_output = Self::get_full_output(
            &InterfaceMode::Menu,
            gpu_context.window_arc.clone(),
            overlay_renderer,
        );

        let paint_jobs = overlay_renderer
            .egui_context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: gpu_context.window_arc.inner_size().into(),
            pixels_per_point: gpu_context.window_arc.scale_factor() as f32,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            overlay_renderer.egui_renderer.update_texture(
                &gpu_context.device,
                &gpu_context.queue,
                *id,
                image_delta,
            );
        }

        overlay_renderer.egui_renderer.update_buffers(
            &gpu_context.device,
            &gpu_context.queue,
            encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: surface_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: RenderMode::get_load_op(render_mode),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("EGUI Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        overlay_renderer.egui_renderer.render(
            &mut render_pass.forget_lifetime(),
            &paint_jobs,
            &screen_descriptor,
        )
    }

    #[instrument(skip_all)]
    pub fn render_run_mode(
        render_mode: &RenderMode,
        surface_texture_view: &wgpu::TextureView,
        gpu_context: &mut GPUContext,
        overlay_renderer: &mut Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let full_output = Self::get_full_output(
            &InterfaceMode::Run,
            gpu_context.window_arc.clone(),
            overlay_renderer,
        );

        let paint_jobs = overlay_renderer
            .egui_context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: gpu_context.window_arc.inner_size().into(),
            pixels_per_point: gpu_context.window_arc.scale_factor() as f32,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            overlay_renderer.egui_renderer.update_texture(
                &gpu_context.device,
                &gpu_context.queue,
                *id,
                image_delta,
            );
        }

        overlay_renderer.egui_renderer.update_buffers(
            &gpu_context.device,
            &gpu_context.queue,
            encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: surface_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: RenderMode::get_load_op(render_mode),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("EGUI Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        overlay_renderer.egui_renderer.render(
            &mut render_pass.forget_lifetime(),
            &paint_jobs,
            &screen_descriptor,
        )
    }

    pub fn handle_device_event(event: &DeviceEvent, overlay_renderer: &mut Self) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            overlay_renderer
                .egui_winit_state
                .on_mouse_motion((*dx, *dy))
        };
    }

    pub fn handle_window_event(
        event: &WindowEvent,
        gpu_context: &GPUContext,
        overlay_renderer: &mut Self,
    ) -> bool {
        let event_response = overlay_renderer
            .egui_winit_state
            .on_window_event(&gpu_context.window_arc, event);

        event_response.consumed
    }

    pub fn set_main_window_active(
        main_window_active: bool,
        gpu_context: &mut GPUContext,
        overlay_renderer: &mut Self,
    ) {
        overlay_renderer.content.run_content.main_window_active = main_window_active;

        if main_window_active {
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

    pub fn toggle_main_window_active(overlay_renderer: &mut Self, gpu_context: &mut GPUContext) {
        Self::set_main_window_active(
            !overlay_renderer.content.run_content.main_window_active,
            gpu_context,
            overlay_renderer,
        );
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
