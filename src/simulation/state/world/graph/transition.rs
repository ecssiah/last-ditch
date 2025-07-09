use glam::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct Transition {
    pub position1: IVec3,
    pub position2: IVec3,
}

impl Transition {
    pub fn new(position1: IVec3, position2: IVec3) -> Self {
        Self {
            position1,
            position2,
        }
    }
}
