use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct IntBox {
    pub min: IVec3,
    pub max: IVec3,
}

impl IntBox {
    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    pub fn get_size(int_box: Self) -> IVec3 {
        int_box.max - int_box.min + IVec3::one()
    }

    pub fn overlaps(int_box1: &Self, int_box2: &Self) -> bool {
        int_box1.min.x < int_box2.max.x
            && int_box1.max.x > int_box2.min.x
            && int_box1.min.y < int_box2.max.y
            && int_box1.max.y > int_box2.min.y
            && int_box1.min.z < int_box2.max.z
            && int_box1.max.z > int_box2.min.z
    }
}
