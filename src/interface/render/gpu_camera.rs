#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUCamera {
    pub view_projection_matrix: [[f32; 4]; 4],
    pub camera_position: [f32; 3],
    pub _padding: f32,
}