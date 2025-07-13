use glam::IVec3;

#[derive(Clone, Debug)]
pub enum Kind {
    Region,
    Local,
}

#[derive(Clone, Debug)]
pub struct Path {
    pub valid: bool,
    pub kind: Kind,
    pub position_vec: Vec<IVec3>,
}

impl Path {
    pub fn new(kind: Kind) -> Self {
        Self {
            valid: false,
            kind,
            position_vec: Vec::new(),
        }
    }
}
