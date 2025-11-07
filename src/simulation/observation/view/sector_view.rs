use crate::simulation::{observation::view::FaceView, state::world::{cell::Cell, sector}};
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_id: sector::ID,
    pub world_position: Vec3,
    pub radius: Vec3,
    pub face_view_vec: Vec<FaceView>,
    pub cell_vec: Vec<Cell>,
}

impl SectorView {
    pub fn new() -> Self {
        Self {
            sector_id: sector::ID::MAX,
            world_position: Vec3::ZERO,
            radius: Vec3::ZERO,
            face_view_vec: Vec::new(),
            cell_vec: Vec::new(),
        }
    }
}
