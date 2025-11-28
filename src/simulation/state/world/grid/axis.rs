use ultraviolet::Vec3;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const ALL: [Axis; 3] = [
        Axis::X, Axis::Y, Axis::Z
    ];

    pub fn unit(axis: Axis) -> Vec3 {
        match axis {
            Axis::X => Vec3::unit_x(),
            Axis::Y => Vec3::unit_y(),
            Axis::Z => Vec3::unit_z(),
        }
    }
}
