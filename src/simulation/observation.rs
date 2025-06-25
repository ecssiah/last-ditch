//! Exposes Simulation data for Interface

pub mod buffer;
pub mod repository;
pub mod state_pair;
pub mod view;

use crate::simulation::{
    consts::JUDGE_VIEW_RADIUS_SQUARED,
    observation::{
        repository::Repository,
        state_pair::StatePair,
        view::{
            AdminView, AgentView, ChunkView, JudgeView, PopulationView, TimeView, View, WorldView,
        },
    },
    state::{
        population::{entity::Judge, Population},
        time::Time,
        world::{chunk, World},
        Admin, State,
    },
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
        let judge = state.population.get_judge();

        let view = self.repository_arc.get();

        let admin_view = self.update_admin_view(&view.admin_view, &state.admin);
        let time_view = self.update_time_view(&view.time_view, &state.time);
        let population_view = self.update_population_view(&view.population_view, &state.population);
        let world_view = self.update_world_view(judge, &view.world_view, &state.world);

        let next_view = View {
            judge_id: judge.id,
            admin_view,
            time_view,
            population_view,
            world_view,
        };

        self.repository_arc.set(next_view);
    }

    fn update_admin_view(&self, _admin_view: &AdminView, admin: &Admin) -> AdminView {
        AdminView {
            mode: admin.mode,
            message: admin.message.clone(),
        }
    }

    fn update_time_view(&self, time_view: &TimeView, time: &Time) -> TimeView {
        TimeView {
            instant: StatePair::new(time_view.instant.next, time.instant),
        }
    }

    fn update_population_view(
        &self,
        population_view: &PopulationView,
        population: &Population,
    ) -> PopulationView {
        let judge = population.get_judge();

        let mut next_population_view = PopulationView {
            judge_view: JudgeView {
                id: judge.id,
                spatial: StatePair::new(population_view.judge_view.spatial.next, judge.spatial),
                kinematic: StatePair::new(
                    population_view.judge_view.kinematic.next,
                    judge.kinematic,
                ),
            },
            agent_view_map: HashMap::new(),
        };

        for agent in population.get_agent_map() {
            let distance_to_judge_squared = (agent.spatial.world_position
                - population.judge.spatial.world_position)
                .length_squared();

            if distance_to_judge_squared > JUDGE_VIEW_RADIUS_SQUARED {
                continue;
            }

            let new_agent_view =
                if let Some(agent_view) = population_view.agent_view_map.get(&agent.id) {
                    AgentView {
                        id: agent.id,
                        kind: agent.kind,
                        spatial: StatePair::new(agent_view.spatial.next, agent.spatial),
                        kinematic: StatePair::new(
                            population_view.judge_view.kinematic.next,
                            judge.kinematic,
                        ),
                    }
                } else {
                    AgentView {
                        id: agent.id,
                        kind: agent.kind,
                        spatial: StatePair::new(agent.spatial, agent.spatial),
                        kinematic: StatePair::new(agent.kinematic, agent.kinematic),
                    }
                };

            next_population_view
                .agent_view_map
                .insert(agent.id, new_agent_view);
        }

        next_population_view
    }

    fn update_world_view(&self, judge: &Judge, world_view: &WorldView, world: &World) -> WorldView {
        let mut next_world_view = WorldView {
            chunk_view_map: HashMap::new(),
        };

        let visible_chunk_id_list = world.get_visible_chunk_id_list(judge);

        for chunk_id in visible_chunk_id_list {
            if let Some(chunk) = world.get_chunk(chunk_id) {
                let chunk_view = self.update_chunk_view(chunk, world_view);

                next_world_view.chunk_view_map.insert(chunk.id, chunk_view);
            }
        }

        next_world_view
    }

    fn update_chunk_view(&self, chunk: &chunk::Chunk, world_view: &WorldView) -> ChunkView {
        let current_chunk_geometry = world_view
            .chunk_view_map
            .get(&chunk.id)
            .map(|view| view.geometry.next.clone())
            .unwrap_or_else(|| chunk.geometry.clone());

        ChunkView {
            id: chunk.id,
            geometry: StatePair::new(current_chunk_geometry, chunk.geometry.clone()),
        }
    }
}

impl Default for Observation {
    fn default() -> Self {
        Self::new()
    }
}
