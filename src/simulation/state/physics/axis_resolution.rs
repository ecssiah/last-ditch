#[derive(Clone, Debug)]
pub struct AxisResolution {
    pub delta_resolved: f32,
    pub velocity_mask: f32,
}

impl AxisResolution {
    pub fn new() -> Self {
        Self {
            delta_resolved: 0.0,
            velocity_mask: 1.0,
        }
    }
}
