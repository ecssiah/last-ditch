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
                        let agent_view = if state.last_update.agents > view.tick {
                            Some(AgentView {
                                agent_id: agent.id,
                                position: agent.position,
                                orientation: agent.orientation,
                            })
                        } else {
                            None
                        };

                        let chunk_views = if state.last_update.world > view.tick {
                            Some(self.generate_chunk_views(state, agent.position))
                        } else {
                            None
                        };

                        if agent_view.is_some() || chunk_views.is_some() {
                            let new_view = View {
                                agent_view: agent_view.unwrap_or_else(|| view.agent_view.clone()),
                                chunk_views: chunk_views
                                    .unwrap_or_else(|| view.chunk_views.clone()),
                                ..(*view).clone()
                            };

                            self.repository.update(agent_id, new_view);
                        }
                    }
                }
            }
        } else {
            let mut status = self.status.write().unwrap();

            *status = Status::Shutdown;
        }
    }

    pub fn get_status(&self) -> Status {
        let status = self.status.read().unwrap();

        status.clone()
    }

    pub fn register_agent(&mut self, state: &State, agent_id: AgentID) {
        if let Some(agent) = state.agents.get(&agent_id) {
            let view = self.generate_view(state, agent);

            self.repository.add(agent_id, view);
        }
    }

    fn generate_view(&self, state: &State, agent: &Agent) -> View {
        View {
            tick: state.time.tick,
            agent_view: AgentView {
                agent_id: agent.id,
                position: agent.position,
                orientation: agent.orientation,
            },
            chunk_views: self.generate_chunk_views(state, agent.position),
        }
    }

    fn generate_chunk_views(&self, state: &State, position: Vec3) -> HashMap<ChunkID, ChunkView> {
        let mut chunk_views = HashMap::new();
        let grid_position = World::world_position_at(position);

        for x in (grid_position.x - 1)..=(grid_position.x + 1) {
            for y in (grid_position.y - 1)..=(grid_position.y + 1) {
                for z in (grid_position.z - 1)..=(grid_position.z + 1) {
                    let chunk_position = IVec3::new(x, y, z);

                    if let Some(chunk) = state.world.get_chunk_at(chunk_position) {
                        let chunk_view = ChunkView {
                            tick: state.time.tick,
                            chunk_id: chunk.id,
                            position: chunk.position,
                            mesh: chunk.mesh.clone(),
                        };

                        chunk_views.insert(chunk_view.chunk_id, chunk_view);
                    }
                }
            }
        }

        chunk_views
    }
}
