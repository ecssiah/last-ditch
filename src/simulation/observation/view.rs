use crate::simulation::{
    self,
    id::{agent_id::AgentID, chunk_id::ChunkID},
    time::Tick,
};
use glam::{IVec3, Quat, Vec3};
use std::collections::HashMap;

#[derive(Clone)]
pub struct AgentView {
    pub agent_id: AgentID,
    pub position: Vec3,
    pub orientation: Quat,
}

#[derive(Clone)]
pub struct ChunkView {
    pub tick: Tick,
    pub chunk_id: ChunkID,
    pub position: IVec3,
    pub mesh: simulation::chunk::mesh::Mesh,
}

#[derive(Clone)]
pub struct View {
    pub tick: Tick,

    pub agent_view: AgentView,
    pub chunk_views: HashMap<ChunkID, ChunkView>,
}
