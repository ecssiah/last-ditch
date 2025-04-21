pub mod buffer;
pub mod repository;
pub mod state_pair;
pub mod view;

use crate::simulation::{
    admin::Admin,
    chunk,
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
    world::World,
    Chunk,
};
use glam::IVec3;
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

    pub fn generate(&self, state: &State) {
        let judge = state.population.get_judge();

        let admin_view = self.generate_admin_view(&state.admin);
        let time_view = self.generate_time_view(&state.time);
        let population_view = self.generate_population_view(&state.population);
        let world_view = self.generate_world_view(&judge, &state.world);

        let next_view = View {
            judge_id: judge.id,
            admin_view,
            time_view,
            population_view,
            world_view,
        };

        self.repository.set(next_view);
    }

    fn generate_admin_view(&self, admin: &Admin) -> AdminView {
        AdminView { mode: admin.mode }
    }

    fn generate_time_view(&self, time: &Time) -> TimeView {
        TimeView {
            tick: StatePair::new(time.tick, time.tick),
            instant: StatePair::new(time.instant, time.instant),
        }
    }

    fn generate_population_view(&self, population: &Population) -> PopulationView {
        let judge = population.get_judge();

        let judge_view = JudgeView {
            id: judge.id,
            tick: StatePair::new(judge.tick, judge.tick),
            position: StatePair::new(judge.position, judge.position),
            orientation: StatePair::new(judge.orientation, judge.orientation),
        };

        let agent_views = population
            .agents
            .values()
            .filter_map(|agent| {
                let judge_distance = (agent.position - judge_view.position.current).length();

                if judge_distance < WORLD_VIEW_RADIUS as f32 {
                    let agent_view = AgentView {
                        id: agent.id,
                        tick: StatePair::new(agent.tick, agent.tick),
                        position: StatePair::new(agent.position, agent.position),
                        target: StatePair::new(agent.position, agent.position),
                    };

                    Some((agent.id, agent_view))
                } else {
                    None
                }
            })
            .collect();

        PopulationView {
            tick: StatePair::new(population.tick, population.tick),
            judge_view,
            agent_views,
        }
    }

    fn generate_world_view(&self, judge: &Judge, world: &World) -> WorldView {
        let mut world_view = WorldView {
            tick: StatePair::new(judge.tick, judge.tick),
            chunk_views: HashMap::new(),
        };

        let grid_position = World::grid_position_at(judge.position).unwrap();
        let chunk_position = Chunk::position_at(grid_position).unwrap();

        let x_range =
            (chunk_position.x - WORLD_VIEW_RADIUS)..=(chunk_position.x + WORLD_VIEW_RADIUS);
        let y_range =
            (chunk_position.y - WORLD_VIEW_RADIUS)..=(chunk_position.y + WORLD_VIEW_RADIUS);
        let z_range =
            (chunk_position.z - WORLD_VIEW_RADIUS)..=(chunk_position.z + WORLD_VIEW_RADIUS);

        for x in x_range {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let chunk_position = IVec3::new(x, y, z);

                    if let Some(chunk_id) = World::id_at(chunk_position) {
                        if let Some(chunk) = world.get_chunk(chunk_id) {
                            let chunk_view = self.generate_chunk_view(chunk);

                            world_view.chunk_views.insert(chunk.id, chunk_view);
                        }
                    }
                }
            }
        }

        world_view
    }

    fn generate_chunk_view(&self, chunk: &chunk::Chunk) -> ChunkView {
        let chunk_view = ChunkView {
            id: chunk.id,
            tick: StatePair::new(chunk.tick, chunk.tick),
            position: StatePair::new(chunk.position, chunk.position),
            mesh: StatePair::new(chunk.mesh.clone(), chunk.mesh.clone()),
        };

        chunk_view
    }

    fn update_view(&self, state: &State) {
        let judge = state.population.get_judge();

        let view = self.repository.get();

        let admin_view = self.update_admin_view(&state.admin);
        let time_view = self.update_time_view(&view.time_view, &state.time);
        let population_view = self.update_population_view(&view.population_view, &state.population);
        let world_view = self.update_world_view(&judge, &view.world_view, &state.world);

        let next_view = View {
            judge_id: judge.id,
            admin_view,
            time_view,
            population_view,
            world_view,
        };

        self.repository.set(next_view);
    }

    fn update_admin_view(&self, admin: &Admin) -> AdminView {
        AdminView { mode: admin.mode }
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
                position: StatePair::new(
                    population_view.judge_view.position.next,
                    judge.position,
                ),
                orientation: StatePair::new(
                    population_view.judge_view.orientation.next,
                    judge.orientation,
                ),
            },
            agent_views: HashMap::new(),
        };

        for agent in population.all_agents() {
            let judge_distance = (agent.position - population.judge.position).length();

            if judge_distance > POPULATION_VIEW_RADIUS {
                continue;
            }

            if let Some(agent_view) = population_view.agent_views.get(&agent.id) {
                let next_agent_view = AgentView {
                    id: agent.id,
                    tick: StatePair::new(agent_view.tick.next, agent.tick),
                    position: StatePair::new(agent_view.position.next, agent.position),
                    target: StatePair::new(agent_view.target.next, agent.target),
                };

                next_population_view
                    .agent_views
                    .insert(agent.id, next_agent_view);
            } else {
                let next_agent_view = AgentView {
                    id: agent.id,
                    tick: StatePair::new(agent.tick, agent.tick),
                    position: StatePair::new(agent.position, agent.position),
                    target: StatePair::new(agent.target, agent.target),
                };

                next_population_view
                    .agent_views
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
            tick: StatePair::new(world_view.tick.current, world.tick),
            chunk_views: HashMap::new(),
        };

        let grid_position = World::grid_position_at(judge.position).unwrap();
        let current_chunk_id = World::id_at_grid(grid_position).unwrap();

        let visible_chunk_ids = World::visible_chunk_ids(current_chunk_id);

        for chunk_id in visible_chunk_ids {
            if let Some(chunk) = world.get_chunk(chunk_id) {
                let chunk_view = self.update_chunk_view(chunk, world_view);

                next_world_view.chunk_views.insert(chunk.id, chunk_view);
            }
        }

        next_world_view
    }

    fn update_chunk_view(&self, chunk: &chunk::Chunk, world_view: &WorldView) -> ChunkView {
        let next_chunk_view;

        if let Some(chunk_view) = world_view.chunk_views.get(&chunk.id) {
            if chunk_view.tick.next < chunk.tick {
                next_chunk_view = ChunkView {
                    id: chunk.id,
                    tick: StatePair::new(chunk_view.tick.next, chunk.tick),
                    position: StatePair::new(chunk_view.position.next, chunk.position),
                    mesh: StatePair::new(chunk_view.mesh.next.clone(), chunk.mesh.clone()),
                };
            } else {
                next_chunk_view = chunk_view.clone();
            }
        } else {
            next_chunk_view = ChunkView {
                id: chunk.id,
                tick: StatePair::new(chunk.tick, chunk.tick),
                position: StatePair::new(chunk.position, chunk.position),
                mesh: StatePair::new(chunk.mesh.clone(), chunk.mesh.clone()),
            };
        }

        next_chunk_view
    }
}
