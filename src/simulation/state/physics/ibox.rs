use ultraviolet::IVec3;

pub struct IBox {
    pub min: IVec3,
    pub max: IVec3,
}

impl IBox {
    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    pub fn overlaps(bounding_box1: &Self, bounding_box2: &Self) -> bool {
        bounding_box1.min.x < bounding_box2.max.x
            && bounding_box1.max.x > bounding_box2.min.x
            && bounding_box1.min.y < bounding_box2.max.y
            && bounding_box1.max.y > bounding_box2.min.y
            && bounding_box1.min.z < bounding_box2.max.z
            && bounding_box1.max.z > bounding_box2.min.z
    }
}
