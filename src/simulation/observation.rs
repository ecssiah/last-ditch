use crate::simulation::{
    agent::Agent,
    id::{agent_id::AgentID, chunk_id::ChunkID},
    observation::{
        repository::Repository,
        view::{AgentView, ChunkView, View},
    },
    state::State,
    world::World,
};
use glam::{IVec3, Vec3};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub mod buffer;
pub mod repository;
pub mod view;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Status {
    Running,
    Saving,
    Loading,
    Shutdown,
}

pub struct Observation {
    status: Arc<RwLock<Status>>,
    repository: repository::Repository,
}

impl Observation {
    pub fn new() -> Self {
        let status = Arc::new(RwLock::new(Status::Running));
        let repository = Repository::new();

        let observation = Self { status, repository };

        observation
    }

    pub fn update(&mut self, state: &State) {
        if state.active {
            for agent_id in self.repository.list_agents() {
                if let Some(agent) = state.agents.get(&agent_id) {
                    if let Some(view) = self.repository.get(agent_id) {
                        let agent_view = self.generate_agent_view(agent);
                        let chunk_views = self.generate_chunk_views(state, agent.position, &view.chunk_views);

                        let new_view = View {
                            agent_view,
                            chunk_views,
                        };

                        self.repository.update(agent_id, new_view);
                    }
                }
            }
        } else {
            let mut status = self.status.write().unwrap();

            *status = Status::Shutdown;
        }
    }

    pub fn register_agent(&mut self, state: &State, agent_id: AgentID) {
        if let Some(agent) = state.agents.get(&agent_id) {
            let view = View {
                agent_view: self.generate_agent_view(agent),
                chunk_views: self.generate_chunk_views(state, agent.position, &HashMap::new()),
            };

            self.repository.add(agent_id, view);
        }
    }

    pub fn get_status(&self) -> Status {
        let status = self.status.read().unwrap();

        status.clone()
    }

    pub fn get_view(&self, agent_id: AgentID) -> Option<View> {
        if let Some(view) = self.repository.get(agent_id) {
            Some((*view).clone())
        } else {
            None
        }
    }

    fn generate_agent_view(&self, agent: &Agent) -> AgentView {
        AgentView {
            id: agent.id,
            tick: agent.tick,
            position: agent.position,
            orientation: agent.orientation,
        }
    }

    fn generate_chunk_views(&self, state: &State, position: Vec3, old_chunk_views: &HashMap<ChunkID, ChunkView>) -> HashMap<ChunkID, ChunkView> {
        let mut new_chunk_views = HashMap::new();
        let grid_position = World::world_position_at(position);

        for x in (grid_position.x - 1)..=(grid_position.x + 1) {
            for y in (grid_position.y - 1)..=(grid_position.y + 1) {
                for z in (grid_position.z - 1)..=(grid_position.z + 1) {
                    let chunk_position = IVec3::new(x, y, z);

                    if let Some(chunk) = state.world.get_chunk_at(chunk_position) {
                        if let Some(old_chunk_view) = old_chunk_views.get(&chunk.id) {
                            if old_chunk_view.tick < chunk.tick {
                                let new_chunk_view = ChunkView {
                                    id: chunk.id,
                                    tick: state.time.tick,
                                    position: chunk.position,
                                    mesh: chunk.mesh.clone(),
                                };

                                new_chunk_views.insert(new_chunk_view.id, new_chunk_view);
                            } else {
                                let new_chunk_view = old_chunk_view.clone();

                                new_chunk_views.insert(new_chunk_view.id, new_chunk_view);
                            }
                        } else {
                            let new_chunk_view = ChunkView {
                                id: chunk.id,
                                tick: state.time.tick,
                                position: chunk.position,
                                mesh: chunk.mesh.clone(),
                            };

                            new_chunk_views.insert(new_chunk_view.id, new_chunk_view);
                        }
                    }
                }
            }
        }

        new_chunk_views
    }
}
