//! Exposes Simulation data for Interface

pub mod buffer;
pub mod repository;
pub mod view;

use crate::simulation::{
    consts::JUDGE_VIEW_RADIUS_SQUARED,
    observation::{
        repository::Repository,
        view::{
            AdminView, AgentView, ChunkView, JudgeView, PopulationView, TimeView, View, WorldView,
        },
    },
    state::State,
};
use std::{collections::HashMap, sync::Arc};

pub struct Observation {
    repository_arc: Arc<repository::Repository>,
}

impl Observation {
    pub fn new() -> Self {
        let repository_arc = Arc::new(Repository::new());

        Self { repository_arc }
    }

    pub fn tick(&self, state: &State) {
        self.update_view(state);
    }

    pub fn get_view(&self) -> View {
        let view = self.repository_arc.get();

        (*view).clone()
    }

    fn update_view(&self, state: &State) {
        let admin_view = self.update_admin_view(&state);
        let time_view = self.update_time_view(&state);
        let population_view = self.update_population_view(&state);
        let world_view = self.update_world_view(&state);

        let view = View {
            judge_id: state.population.judge.id,
            admin_view,
            time_view,
            population_view,
            world_view,
        };

        self.repository_arc.set(view);
    }

    fn update_admin_view(&self, state: &State) -> AdminView {
        AdminView {
            mode: state.admin.mode,
            message: state.admin.message.clone(),
        }
    }

    fn update_time_view(&self, state: &State) -> TimeView {
        TimeView {
            instant: state.time.instant,
        }
    }

    fn update_population_view(&self, state: &State) -> PopulationView {
        let judge = state.population.get_judge();

        let mut population_view = PopulationView {
            judge_view: JudgeView {
                id: judge.id,
                position: judge.spatial.position(),
                world_position: judge.spatial.world_position,
                chunk_id: judge.chunk_id,
                chunk_coordinates: state
                    .world
                    .grid
                    .chunk_id_to_chunk_coordinates(judge.chunk_id)
                    .unwrap(),
                size: judge.size(),
                quaternion: judge.spatial.quaternion,
            },
            agent_view_map: HashMap::new(),
        };

        for agent in state.population.get_agent_map() {
            let distance_to_judge_squared = (agent.spatial.world_position
                - state.population.judge.spatial.world_position)
                .length_squared();

            if distance_to_judge_squared > JUDGE_VIEW_RADIUS_SQUARED {
                continue;
            }

            let agent_view = AgentView {
                id: agent.id,
                kind: agent.kind,
                spatial: agent.spatial,
                kinematic: agent.kinematic,
                detection: agent.detection,
            };

            population_view.agent_view_map.insert(agent.id, agent_view);
        }

        population_view
    }

    fn update_world_view(&self, state: &State) -> WorldView {
        let mut world_view = WorldView {
            chunk_view_map: HashMap::new(),
        };

        let visible_chunk_id_vec = state
            .world
            .get_visible_chunk_id_vec(&state.population.judge);

        for chunk_id in visible_chunk_id_vec {
            if let Some(chunk) = state.world.get_chunk(chunk_id) {
                let chunk_view = ChunkView {
                    id: chunk.id,
                    geometry: chunk.geometry.clone(),
                };

                world_view.chunk_view_map.insert(chunk.id, chunk_view);
            }
        }

        world_view
    }
}

impl Default for Observation {
    fn default() -> Self {
        Self::new()
    }
}
