//! Information displayed over World rendering

use crate::{
    interface::gpu::gpu_context::GPUContext,
    simulation::{manager::Message, viewer::View},
};
use egui::{FontId, FullOutput, Id, Ui};
use std::{collections::VecDeque, sync::Arc};
use ultraviolet::Vec2;
use winit::event::{DeviceEvent, WindowEvent};

#[derive(Default)]
pub struct GUI {
    pub menu_active: bool,
    pub message_deque: VecDeque<Message>,
}

impl GUI {
    pub fn new() -> Self {
        let menu_active = true;
        let message_deque = VecDeque::new();

        Self {
            menu_active,
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

        let mut gui_delegate = std::mem::take(gui);

        let full_output: FullOutput = egui_context.run(raw_input, |context| {
            Self::show(context, &mut gui_delegate);
        });

        *gui = gui_delegate;

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

        let mut start_clicked = false;
        let mut quit_clicked = false;

        egui::CentralPanel::default().show(context, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.4);

                start_clicked = ui
                    .add_sized([200.0, 60.0], egui::Button::new("Start"))
                    .clicked();

                quit_clicked = ui
                    .add_sized([200.0, 60.0], egui::Button::new("Exit"))
                    .clicked();
            });
        });

        if start_clicked {
            gui.message_deque.push_back(Message::Start);
        }

        if quit_clicked {
            gui.message_deque.push_back(Message::Quit);
        }
    }

    fn show_hud(context: &egui::Context, _gui: &mut GUI) {
        let message_str = "Hello Simulate!";

        egui::Area::new(Id::new(0))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(16.0, 16.0))
            .show(context, |ui| {
                Self::show_hud_text(ui, Vec2::new(6.0, 6.0), message_str);
            });
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

    pub fn apply_view(view: &View, _gui: &mut GUI) {
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

        let mut message = String::new();
        message.push_str(&position_string);
        message.push_str(&world_position_string);
        message.push_str(&sector_string);
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

    pub fn handle_device_event(event: &DeviceEvent, gpu_context: &mut GPUContext) -> bool {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            gpu_context.egui_winit_state.on_mouse_motion((*dx, *dy))
        };

        true
    }

    pub fn handle_window_event(event: &WindowEvent, gpu_context: &mut GPUContext) -> bool {
        let _event_response = gpu_context
            .egui_winit_state
            .on_window_event(&gpu_context.window_arc, event);

        true
    }

    pub fn toggle_menu(gui: &mut GUI, gpu_context: &mut GPUContext) {
        if gui.menu_active {
            gui.menu_active = false;

            gpu_context.window_arc.set_cursor_visible(false);

            gpu_context
                .window_arc
                .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                .expect("Failed to grab cursor");
        } else {
            gui.menu_active = true;

            gpu_context.window_arc.set_cursor_visible(true);

            gpu_context
                .window_arc
                .set_cursor_grab(winit::window::CursorGrabMode::None)
                .expect("Failed to grab cursor");
        }
    }
}
