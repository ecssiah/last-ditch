use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct IntegrationResult {
    pub delta_intent: Vec3,
    pub velocity_intent: Vec3,
}

impl IntegrationResult {
    pub fn new() -> Self {
        Self {
            delta_intent: Vec3::zero(),
            velocity_intent: Vec3::zero(),
        }
    }
}