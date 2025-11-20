use crate::simulation::{observation::view::FaceView, state::world::sector};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_id: sector::ID,
    pub world_position: Vec3,
    pub radius: f32,
    pub face_view_vec: Vec<FaceView>,
}

impl SectorView {
    pub fn new() -> Self {
        Self {
            sector_id: sector::ID::MAX,
            world_position: Vec3::broadcast(0.0),
            radius: 0.0,
            face_view_vec: Vec::new(),
        }
    }
}
