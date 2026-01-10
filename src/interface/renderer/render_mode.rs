use crate::interface::constants::WINDOW_CLEAR_COLOR;

#[derive(Clone)]
pub enum RenderMode {
    Load,
    Clear,
}

impl RenderMode {
    pub fn get_load_op(render_mode: &Self) -> wgpu::LoadOp<wgpu::Color> {
        match render_mode {
            RenderMode::Clear => wgpu::LoadOp::Clear(wgpu::Color {
                r: WINDOW_CLEAR_COLOR[0],
                g: WINDOW_CLEAR_COLOR[1],
                b: WINDOW_CLEAR_COLOR[2],
                a: WINDOW_CLEAR_COLOR[3],
            }),
            RenderMode::Load => wgpu::LoadOp::Load,
        }
    }
}
