use glam::{Vec3, Vec3Swizzles};

#[derive(Copy, Clone, Debug, Default)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(center: Vec3, size: Vec3) -> Self {
        let extent = size * 0.5;
        let min = center - extent;
        let max = center + extent;

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

    pub fn extent(&self) -> Vec3 {
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
        let extent = (self.max - self.min) * 0.5;

        self.min = center - extent;
        self.max = center + extent;
    }

    pub fn bottom_center(&self) -> Vec3 {
        Vec3::new(
            (self.min.x + self.max.x) * 0.5,
            self.min.y,
            (self.min.z + self.max.z) * 0.5,
        )
    }

    pub fn set_bottom_center(&mut self, x: f32, y: f32, z: f32) {
        let size = self.size();
        let xz_extent = size.xz() * 0.5;

        self.min = Vec3::new(x - xz_extent.x, y, z - xz_extent.y);
        self.max = Vec3::new(x + xz_extent.x, y + size.y, z + xz_extent.y);
    }

    pub fn translate(&self, displacement: Vec3) -> AABB {
        AABB {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }

    pub fn intersects(&self, other: AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn overlaps(&self, other: AABB) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn overlap_axis(&self, axis_index: usize, cell_aabb: AABB) -> f32 {
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

    pub fn approx_eq(&self, other: AABB, epsilon: f32) -> bool {
        self.min.abs_diff_eq(other.min, epsilon) && self.max.abs_diff_eq(other.max, epsilon)
    }

    pub fn approx_set_eq(list1: &[AABB], list2: &[AABB], epsilon: f32) -> bool {
        if list1.len() != list2.len() {
            return false;
        }

        list1
            .iter()
            .all(|aabb1| list2.iter().any(|aabb2| aabb1.approx_eq(*aabb2, epsilon)))
            && list2
                .iter()
                .all(|aabb2| list1.iter().any(|aabb1| aabb2.approx_eq(*aabb1, epsilon)))
    }

    pub fn sweep(aabb1: AABB, aabb2: AABB) -> AABB {
        let min = aabb1.min.min(aabb2.min);
        let max = aabb1.max.max(aabb2.max);

        let center = (min + max) * 0.5;
        let size = max - min;

        AABB::new(center, size)
    }
}
