#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FogUniformData {
    pub color: [f32; 3],
    pub _padding0: f32,
    pub start: f32,
    pub end: f32,
    pub _padding1: [f32; 2],
}
