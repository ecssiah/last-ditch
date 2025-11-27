#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformData {
    pub view_projection_matrix: [[f32; 4]; 4],
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub camera_position: [f32; 3],
    pub _padding: f32,
}

impl CameraUniformData {
    pub fn new() -> Self {
        Self {
            view_projection_matrix: [[0.0; 4]; 4],
            view_matrix: [[0.0; 4]; 4],
            projection_matrix: [[0.0; 4]; 4],
            camera_position: [0.0, 0.0, 0.0],
            _padding: 0.0,
        }
    }
}
