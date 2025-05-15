use glam::{Vec3, Vec3Swizzles};

#[derive(Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(position: Vec3, size: Vec3) -> Self {
        let xz_radius = size.xz() * 0.5;

        let min = Vec3::new(
            position.x - xz_radius.x,
            position.y,
            position.z - xz_radius.y,
        );
        let max = Vec3::new(
            position.x + xz_radius.x,
            position.y + size.y,
            position.z + xz_radius.y,
        );

        Self { min, max }
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn position(&self) -> Vec3 {
        Vec3::new(
            (self.min.x + self.max.x) * 0.5,
            self.min.y,
            (self.min.z + self.max.z) * 0.5,
        )
    }

    pub fn set_position(&mut self, position: Vec3) {
        let size = self.size();
        let xz_radius = size.xz() * 0.5;

        self.min = Vec3::new(
            position.x - xz_radius.x,
            position.y,
            position.z - xz_radius.y,
        );

        self.max = Vec3::new(
            position.x + xz_radius.x,
            position.y + size.y,
            position.z + xz_radius.y,
        );
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}
