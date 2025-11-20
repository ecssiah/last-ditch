use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Sight {
    pub position: Vec3,
    pub direction: Vec3,
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub max_distance: f32,
}

impl Sight {
    pub fn new() -> Self {
        let position = Vec3::zero();
        let direction = Vec3::zero();
        let horizontal_fov = 180.0;
        let vertical_fov = 60.0;
        let max_distance = 1200.0;

        Self {
            position,
            direction,
            horizontal_fov,
            vertical_fov,
            max_distance,
        }
    }

    pub fn contains(sight: &Sight, point: Vec3) -> bool {
        let to_point = point - sight.position;
        let distance = to_point.mag();

        if distance > sight.max_distance {
            return false;
        }

        let dot = to_point.normalized().dot(sight.direction);

        let angle = dot.acos();

        angle <= (sight.horizontal_fov * 0.5).to_radians()
    }
}
