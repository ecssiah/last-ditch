use crate::simulation::constants::CELL_RADIUS_IN_METERS;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct FloatBox {
    pub center_position: Vec3,
    pub radius: Vec3,
}

impl FloatBox {
    pub fn new(center_position: Vec3, radius: Vec3) -> Self {
        Self {
            center_position,
            radius,
        }
    }

    pub fn translated(displacement: Vec3, float_box: &Self) -> Self {
        Self::new(float_box.center_position + displacement, float_box.radius)
    }

    pub fn scaled(delta_radius: f32, float_box: &Self) -> Self {
        Self::new(
            float_box.center_position,
            float_box.radius + Vec3::broadcast(delta_radius),
        )
    }

    pub fn get_world_position(float_box: &Self) -> Vec3 {
        float_box.center_position
    }

    pub fn set_world_position(world_position: Vec3, float_box: &mut Self) {
        float_box.center_position = world_position
    }

    pub fn get_radius(float_box: &Self) -> Vec3 {
        float_box.radius
    }

    pub fn set_radius(radius: Vec3, float_box: &mut Self) {
        float_box.radius = radius;
    }

    pub fn get_size(float_box: &Self) -> Vec3 {
        float_box.radius * 2.0
    }

    pub fn get_min(float_box: &Self) -> Vec3 {
        float_box.center_position - float_box.radius
    }

    pub fn get_max(float_box: &Self) -> Vec3 {
        float_box.center_position + float_box.radius
    }

    pub fn overlap(left: &Self, right: &Self) -> bool {
        if ((left.center_position.x - right.center_position.x).abs()
            > (left.radius.x + right.radius.x))
            || ((left.center_position.y - right.center_position.y).abs()
                > (left.radius.y + right.radius.y))
            || ((left.center_position.z - right.center_position.z).abs()
                > (left.radius.z + right.radius.z))
        {
            false
        } else {
            true
        }
    }
}

impl Default for FloatBox {
    fn default() -> Self {
        let center_position = Vec3::zero();
        let radius = Vec3::broadcast(CELL_RADIUS_IN_METERS);

        Self::new(center_position, radius)
    }
}
