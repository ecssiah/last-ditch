use ultraviolet::{Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    pub fn new(normal: Vec3, d: f32) -> Self {
        Self { normal, d }
    }

    pub fn normalized(&self) -> Self {
        let inv_len = 1.0 / self.normal.mag();
        Self {
            normal: self.normal * inv_len,
            d: self.d * inv_len,
        }
    }

    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.d
    }
}