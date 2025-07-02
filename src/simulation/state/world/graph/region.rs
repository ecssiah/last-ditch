use glam::IVec3;

#[derive(Debug)]
pub struct Region {
    pub coordinates: IVec3,
    pub min: IVec3,
    pub max: IVec3,
}

impl Region {
    pub fn size(&self) -> IVec3 {
        self.max + IVec3::ONE - self.min
    }
}
