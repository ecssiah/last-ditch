#[derive(Clone)]
pub struct BlockEntry {
    pub north_face: &'static str,
    pub west_face: &'static str,
    pub south_face: &'static str,
    pub east_face: &'static str,
    pub up_face: &'static str,
    pub down_face: &'static str,
}

impl BlockEntry {
    pub fn from_face(face: &'static str) -> Self {
        Self {
            north_face: face,
            west_face: face,
            south_face: face,
            east_face: face,
            up_face: face,
            down_face: face,
        }
    }
}
