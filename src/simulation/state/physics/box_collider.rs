use ultraviolet::{Vec2, Vec3};

#[derive(Copy, Clone, Debug, Default)]
pub struct BoxCollider {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoxCollider {
    pub fn new(center: Vec3, size: Vec3) -> Self {
        let radius = size * 0.5;
        let min = center - radius;
        let max = center + radius;

        Self { min, max }
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    pub fn radius(&self) -> Vec3 {
        self.size() * 0.5
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn set_size(&mut self, size: Vec3) {
        let center = self.center();
        let half_size = size * 0.5;

        self.min = center - half_size;
        self.max = center + half_size;
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn set_center(&mut self, x: f32, y: f32, z: f32) {
        let center = Vec3::new(x, y, z);
        let radius = (self.max - self.min) * 0.5;

        self.min = center - radius;
        self.max = center + radius;
    }

    pub fn bottom_center(&self) -> Vec3 {
        Vec3::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
            self.min.z,
        )
    }

    pub fn set_bottom_center(&mut self, world_position: Vec3) {
        let size = self.size();
        let xy_radius = Vec2::new(size.x, size.y) * 0.5;

        self.min = Vec3::new(
            world_position.x - xy_radius.x,
            world_position.y - xy_radius.y,
            world_position.z,
        );

        self.max = Vec3::new(
            world_position.x + xy_radius.x,
            world_position.y + xy_radius.y,
            world_position.z + size.z,
        );
    }

    pub fn translate(&self, displacement: Vec3) -> Self {
        Self {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn overlaps(&self, other: Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn overlap_axis(&self, axis_index: usize, cell_aabb: Self) -> f32 {
        let min = self.min[axis_index];
        let max = self.max[axis_index];

        let cell_min = cell_aabb.min[axis_index];
        let cell_max = cell_aabb.max[axis_index];

        if max > cell_min && min < cell_max {
            let offset_positive = cell_max - min;
            let offset_negative = max - cell_min;

            let center = (min + max) * 0.5;
            let cell_center = (cell_min + cell_max) * 0.5;

            if center < cell_center {
                offset_positive
            } else {
                -offset_negative
            }
        } else {
            0.0
        }
    }

    pub fn approx_eq(&self, other: Self, epsilon: f32) -> bool {
        (self.min - other.min).mag() <= epsilon && (self.max - other.max).mag() <= epsilon
    }

    pub fn approx_set_eq(list1: &[Self], list2: &[Self], epsilon: f32) -> bool {
        if list1.len() != list2.len() {
            return false;
        }

        list1
            .iter()
            .all(|box1| list2.iter().any(|box2| box1.approx_eq(*box2, epsilon)))
            && list2
                .iter()
                .all(|box2| list1.iter().any(|box1| box2.approx_eq(*box1, epsilon)))
    }

    pub fn sweep(box1: Self, box2: Self) -> Self {
        let min = Vec3::new(
            box1.min.x.min(box2.min.x),
            box1.min.y.min(box2.min.y),
            box1.min.z.min(box2.min.z),
        );

        let max = Vec3::new(
            box1.max.x.max(box2.max.x),
            box1.max.y.max(box2.max.y),
            box1.max.z.max(box2.max.z),
        );

        let center = (min + max) * 0.5;
        let size = max - min;

        Self::new(center, size)
    }
}
