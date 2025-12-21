pub struct FloatBounds {
    pub min: f32,
    pub max: f32,
}

impl FloatBounds {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn get_midpoint(float_bounds: &Self) -> f32 {
        (float_bounds.min + float_bounds.max) / 2.0
    }

    pub fn get_size(float_bounds: &Self) -> f32 {
        float_bounds.max - float_bounds.min
    }
}
