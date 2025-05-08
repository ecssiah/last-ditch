pub mod buffer;
pub mod repository;
pub mod state_pair;
pub mod view;

use crate::simulation::{
    admin::Admin,
    consts::*,
    observation::{
        repository::Repository,
        state_pair::StatePair,
        view::{
            AdminView, AgentView, ChunkView, JudgeView, PopulationView, TimeView, View, WorldView,
        },
    },
    population::{Judge, Population},
    state::State,
    time::Time,
    world::{chunk, grid, World},
};
use std::{collections::HashMap, sync::Arc};

pub struct Observation {
    repository: Arc<repository::Repository>,
}

impl Observation {
    pub fn new() -> Self {
        let repository = Arc::new(Repository::new());

        let observation = Self { repository };

        observation
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

    fn update_admin_view(&self, admin_view: &AdminView, admin: &Admin) -> AdminView {
        AdminView {
            tick: StatePair::new(admin_view.tick.next, admin.tick),
            mode: admin.mode,
            message: admin.message.clone(),
        }
    }

    fn update_time_view(&self, time_view: &TimeView, time: &Time) -> TimeView {
        TimeView {
            tick: StatePair::new(time_view.tick.next, time.tick),
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
            tick: StatePair::new(population_view.tick.next, population.tick),
            judge_view: JudgeView {
                id: judge.id,
                tick: StatePair::new(population_view.judge_view.tick.next, judge.tick),
                position: StatePair::new(population_view.judge_view.position.next, judge.position),
                size: StatePair::new(population_view.judge_view.size.next, judge.size),
                orientation: StatePair::new(
                    population_view.judge_view.orientation.next,
                    judge.orientation,
                ),
            },
            agent_view_map: HashMap::new(),
        };

        for agent in population.get_agent_map() {
            let judge_distance_squared =
                (agent.position - population.judge.position).length_squared();

            if judge_distance_squared > POPULATION_VIEW_RADIUS_SQUARED {
                continue;
            }

            if let Some(agent_view) = population_view.agent_view_map.get(&agent.id) {
                let next_agent_view = AgentView {
                    id: agent.id,
                    kind: agent.kind.clone(),
                    height: agent.height,
                    tick: StatePair::new(agent_view.tick.next, agent.tick),
                    position: StatePair::new(agent_view.position.next, agent.position),
                    target: StatePair::new(agent_view.target.next, agent.target),
                };

                next_population_view
                    .agent_view_map
                    .insert(agent.id, next_agent_view);
            } else {
                let next_agent_view = AgentView {
                    id: agent.id,
                    kind: agent.kind.clone(),
                    height: agent.height,
                    tick: StatePair::new(agent.tick, agent.tick),
                    position: StatePair::new(agent.position, agent.position),
                    target: StatePair::new(agent.target, agent.target),
                };

                next_population_view
                    .agent_view_map
                    .insert(agent.id, next_agent_view);
            }
        }

        next_population_view
    }

    fn update_world_view(&self, judge: &Judge, world_view: &WorldView, world: &World) -> WorldView {
        if !judge.chunk_update {
            return world_view.clone();
        }

        let mut next_world_view = WorldView {
            tick: StatePair::new(world_view.tick.next, world.tick),
            chunk_view_map: HashMap::new(),
        };

        log::info!("View: {:?}", judge.position);
        log::info!("Grid: {:?}", grid::world_to_grid(judge.position));

        let grid_position = grid::world_to_grid(judge.position).unwrap();
        let current_chunk_id = grid::get_chunk_id(grid_position).unwrap();

        let visible_chunk_id_list = World::get_visible_chunk_id_list(current_chunk_id);

        for chunk_id in visible_chunk_id_list {
            if let Some(chunk) = world.get_chunk(chunk_id) {
                let chunk_view = self.update_chunk_view(chunk, world_view);

                next_world_view.chunk_view_map.insert(chunk.id, chunk_view);
            }
        }

        next_world_view
    }

    fn update_chunk_view(&self, chunk: &chunk::Chunk, world_view: &WorldView) -> ChunkView {
        let next_chunk_view;

        if let Some(chunk_view) = world_view.chunk_view_map.get(&chunk.id) {
            if chunk_view.tick.next < chunk.tick {
                next_chunk_view = ChunkView {
                    id: chunk.id,
                    tick: StatePair::new(chunk_view.tick.next, chunk.tick),
                    position: StatePair::new(chunk_view.position.next, chunk.position),
                    geometry: StatePair::new(
                        chunk_view.geometry.next.clone(),
                        chunk.geometry.clone(),
                    ),
                };
            } else {
                next_chunk_view = chunk_view.clone();
            }
        } else {
            next_chunk_view = ChunkView {
                id: chunk.id,
                tick: StatePair::new(chunk.tick, chunk.tick),
                position: StatePair::new(chunk.position, chunk.position),
                geometry: StatePair::new(chunk.geometry.clone(), chunk.geometry.clone()),
            };
        }

        next_chunk_view
    }
}
