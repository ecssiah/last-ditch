use ultraviolet::Vec3;

pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn overlaps(bounding_box1: &Self, bounding_box2: &Self) -> bool {
        bounding_box1.min.x < bounding_box2.max.x
            && bounding_box1.max.x > bounding_box2.min.x
            && bounding_box1.min.y < bounding_box2.max.y
            && bounding_box1.max.y > bounding_box2.min.y
            && bounding_box1.min.z < bounding_box2.max.z
            && bounding_box1.max.z > bounding_box2.min.z
    }
}
