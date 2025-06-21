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
    repository: Arc<repository::Repository>,
}

impl Observation {
    pub fn new() -> Self {
        let repository = Arc::new(Repository::new());

        Self { repository }
    }

    pub fn tick(&self, state: &State) {
        self.update_view(&state);
    }

    pub fn get_view(&self) -> View {
        let view = self.repository.get();

        (*view).clone()
    }

    fn update_view(&self, state: &State) {
        let judge = state.population.get_judge();

        let view = self.repository.get();

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

        self.repository.set(next_view);
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
            let judge_distance_squared = (agent.spatial.world_position
                - population.judge.spatial.world_position)
                .length_squared();

            if judge_distance_squared > JUDGE_VIEW_RADIUS_SQUARED {
                continue;
            }

            if let Some(agent_view) = population_view.agent_view_map.get(&agent.id) {
                let next_agent_view = AgentView {
                    id: agent.id,
                    kind: agent.kind,
                    spatial: StatePair::new(agent_view.spatial.next, agent.spatial),
                    kinematic: StatePair::new(
                        population_view.judge_view.kinematic.next,
                        judge.kinematic,
                    ),
                };

                next_population_view
                    .agent_view_map
                    .insert(agent.id, next_agent_view);
            } else {
                let next_agent_view = AgentView {
                    id: agent.id,
                    kind: agent.kind,
                    spatial: StatePair::new(agent.spatial, agent.spatial),
                    kinematic: StatePair::new(agent.kinematic, agent.kinematic),
                };

                next_population_view
                    .agent_view_map
                    .insert(agent.id, next_agent_view);
            }
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
                if judge.viewpoint.intersects(&chunk.aabb) {
                    let chunk_view = self.update_chunk_view(chunk, world_view);

                    next_world_view.chunk_view_map.insert(chunk.id, chunk_view);
                }
            }
        }

        next_world_view
    }

    fn update_chunk_view(&self, chunk: &chunk::Chunk, world_view: &WorldView) -> ChunkView {
        let next_chunk_view;

        if let Some(chunk_view) = world_view.chunk_view_map.get(&chunk.id) {
            next_chunk_view = ChunkView {
                id: chunk.id,
                geometry: StatePair::new(chunk_view.geometry.next.clone(), chunk.geometry.clone()),
            };
        } else {
            next_chunk_view = ChunkView {
                id: chunk.id,
                geometry: StatePair::new(chunk.geometry.clone(), chunk.geometry.clone()),
            };
        }

        next_chunk_view
    }
}
