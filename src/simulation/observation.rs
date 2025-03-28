use crate::simulation::{chunk::ChunkID, id::AgentID, world::Tick};
use glam::Vec3;
use std::collections::HashMap;

struct AgentView {}

struct ChunkView {}

struct View {
    pub tick: Tick,
    pub position: Vec3,

    pub agent_snapshots: HashMap<AgentID, AgentView>,
    pub chunk_snapshots: HashMap<ChunkID, ChunkView>,
}

pub struct Repository {}

struct Observation {}
