use crate::simulation::{agent, chunk, time::Tick};
use glam::{IVec3, Quat, Vec3};
use std::collections::HashMap;

#[derive(Clone)]
pub struct AgentView {
    pub id: agent::ID,
    pub tick: Tick,
    pub position: Vec3,
    pub orientation: Quat,
}

#[derive(Clone)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub tick: Tick,
    pub position: IVec3,
    pub mesh: chunk::Mesh,
}

#[derive(Clone)]
pub struct View {
    pub agent_view: AgentView,
    pub chunk_views: HashMap<chunk::ID, ChunkView>,
}
