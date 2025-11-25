use ultraviolet::IVec3;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    pub fn all() -> [Axis; 3] {
        [Axis::X, Axis::Y, Axis::Z]
    }

    pub fn unit(axis: Axis) -> IVec3 {
        match axis {
            Axis::X => IVec3::unit_x(),
            Axis::Y => IVec3::unit_y(),
            Axis::Z => IVec3::unit_z(),
        }
    }
}
