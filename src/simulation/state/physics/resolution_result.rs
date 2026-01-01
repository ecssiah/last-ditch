use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct ResolutionResult {
    pub delta_resolved: Vec3,
    pub velocity_mask: Vec3,
}

impl ResolutionResult {
    pub fn new() -> Self {
        Self {
            delta_resolved: Vec3::zero(),
            velocity_mask: Vec3::one(),
        }
    }
}