use crate::{interface::mesh_data::MeshData, simulation::state::world::sector};

pub struct SectorRenderData {
    pub sector_id: sector::ID,
    pub mesh_data: MeshData,
}
