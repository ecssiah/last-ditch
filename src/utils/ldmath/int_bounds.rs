pub struct IntBounds {
    pub min: i32,
    pub max: i32,
}

impl IntBounds {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    pub fn get_midpoint(int_bounds: &Self) -> i32 {
        (int_bounds.min + int_bounds.max) / 2
    }

    pub fn get_size(int_bounds: &Self) -> i32 {
        (int_bounds.max - int_bounds.min) + 1
    }
}
