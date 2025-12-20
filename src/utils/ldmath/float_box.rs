use ultraviolet::Vec3;

use crate::simulation::constants::CELL_RADIUS_IN_METERS;

#[derive(Clone, Debug)]
pub struct FloatBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl FloatBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn translated(displacement: Vec3, float_box: &Self) -> Self {
        Self::new(float_box.min + displacement, float_box.max + displacement)
    }

    pub fn get_world_position(float_box: &Self) -> Vec3 {
        (float_box.min + float_box.max) / 2.0
    }

    pub fn get_radius(float_box: &Self) -> Vec3 {
        Self::get_size(float_box) / 2.0
    }

    pub fn get_size(float_box: &Self) -> Vec3 {
        float_box.max - float_box.min
    }

    pub fn set_size(size: Vec3, float_box: &mut Self) {
        let world_position = Self::get_world_position(float_box);
        let radius = size / 2.0;

        float_box.min = world_position - radius;
        float_box.max = world_position + radius;
    }

    pub fn overlaps(float_box1: &Self, float_box2: &Self) -> bool {
        float_box1.min.x < float_box2.max.x
            && float_box1.max.x > float_box2.min.x
            && float_box1.min.y < float_box2.max.y
            && float_box1.max.y > float_box2.min.y
            && float_box1.min.z < float_box2.max.z
            && float_box1.max.z > float_box2.min.z
    }
}

impl Default for FloatBox {
    fn default() -> Self {
        Self::new(
            Vec3::broadcast(-CELL_RADIUS_IN_METERS),
            Vec3::broadcast(CELL_RADIUS_IN_METERS),
        )
    }
}
