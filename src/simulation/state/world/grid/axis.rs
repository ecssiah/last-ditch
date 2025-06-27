use glam::Vec3;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    pub fn from_usize(index: usize) -> Option<Self> {
        match index {
            0 => Some(Axis::X),
            1 => Some(Axis::Y),
            2 => Some(Axis::Z),
            _ => None,
        }
    }

    pub fn unit(&self) -> Vec3 {
        match self {
            Axis::X => Vec3::new(1.0, 0.0, 0.0),
            Axis::Y => Vec3::new(0.0, 1.0, 0.0),
            Axis::Z => Vec3::new(0.0, 0.0, 1.0),
        }
    }
}
