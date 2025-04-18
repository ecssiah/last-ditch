pub mod buffer;
pub mod repository;
pub mod view;

use crate::simulation::{
    admin::Admin,
    chunk,
    observation::{
        repository::Repository,
        view::{
            AdminView, AgentView, ChunkView, JudgeView, PopulationView, TimeView, View,
            WorldView,
        },
    },
    population::{entity, Entity, Population},
    state::State,
    time::Time,
    world::World,
    Chunk, USER_VIEW_RADIUS,
};
use glam::IVec3;
use std::{collections::HashMap, sync::{Arc, RwLock}};

pub struct Observation {
    repository: Arc<RwLock<repository::Repository>>,
}

impl Observation {
    pub fn new() -> Self {
        let repository = Arc::new(RwLock::new(Repository::new()));

        let observation = Self { repository };

        observation
    }

    pub fn generate(&self, state: &State) {
        if let Some(judge) = state.population.get_judge() {
            let admin_view = self.generate_admin_view(&state.admin);
            let time_view = self.generate_time_view(&state.time);
            let population_view = self.generate_population_view(&state.population);
            let world_view = self.generate_world_view(&judge, &state.world);

            let next_view = View {
                entity_id: judge.id,
                admin_view,
                time_view,
                population_view,
                world_view,
            };

            let repository = self.repository.write().unwrap();
            
            repository.set(&judge.id, next_view);
        }
    }

    fn generate_admin_view(&self, admin: &Admin) -> AdminView {
        AdminView { mode: admin.mode }
    }

    fn generate_time_view(&self, time: &Time) -> TimeView {
        TimeView {
            simulation_instant: time.simulation_instant,
            next_simulation_instant: time.simulation_instant,
        }
    }

    fn generate_population_view(&self, population: &Population) -> PopulationView {
        let judge_view = population.get_judge().map(|judge| JudgeView {
            id: judge.id,
            tick: judge.tick,
            position: judge.position,
            orientation: judge.orientation,
            next_tick: judge.tick,
            next_position: judge.position,
            next_orientation: judge.orientation,
        });

        let agent_views = population
            .agents
            .iter()
            .map(|(entity_id, entity)| {
                let entity_view = AgentView {
                    id: entity.id,
                    tick: entity.tick,
                    position: entity.position,
                    orientation: entity.orientation,
                    next_tick: entity.tick,
                    next_position: entity.position,
                    next_orientation: entity.orientation,
                };

                (*entity_id, entity_view)
            })
            .collect();

        PopulationView {
            tick: population.tick,
            judge_view,
            agent_views,
        }
    }

    fn generate_world_view(&self, entity: &Entity, world: &World) -> WorldView {
        let mut world_view = WorldView {
            tick: entity.tick,
            chunk_views: HashMap::new(),
        };

        let grid_position = World::grid_position_at(entity.position).unwrap();
        let chunk_position = Chunk::position_at(grid_position).unwrap();

        let x_range = (chunk_position.x - USER_VIEW_RADIUS)..=(chunk_position.x + USER_VIEW_RADIUS);
        let y_range = (chunk_position.y - USER_VIEW_RADIUS)..=(chunk_position.y + USER_VIEW_RADIUS);
        let z_range = (chunk_position.z - USER_VIEW_RADIUS)..=(chunk_position.z + USER_VIEW_RADIUS);

        for x in x_range {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let chunk_position = IVec3::new(x, y, z);

                    if let Some(chunk_id) = Chunk::id_at(chunk_position) {
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
            tick: chunk.tick,
            position: chunk.position,
            mesh: chunk.mesh.clone(),
        };

        chunk_view
    }

    pub fn tick(&self, state: &State) {
        self.update_view(&state);
    }

    pub fn get_view(&self, entity_id: &entity::ID) -> Option<View> {
        let repository = self.repository.read().unwrap();

        if let Some(view) = repository.get(&entity_id) {
            Some((*view).clone())
        } else {
            None
        }
    }

    fn update_view(&self, state: &State) {
        if let Some(judge) = state.population.get_judge() {
            let view = {
                let repository = self.repository.read().unwrap();

                repository.get(&judge.id)           
            };

            if let Some(view) = view {
                let admin_view = self.update_admin_view(&state.admin);
                let time_view = self.update_time_view(&state.time, &view.time_view);
                let population_view =
                    self.update_population_view(&state.population, &view.population_view);
                let world_view = self.update_world_view(&judge, &state.world, &view.world_view);
    
                let next_view = View {
                    entity_id: judge.id,
                    admin_view,
                    time_view,
                    population_view,
                    world_view,
                };
    
                let repository = self.repository.write().unwrap();

                repository.set(&judge.id, next_view);
            }
        }
    }

    fn update_admin_view(&self, admin: &Admin) -> AdminView {
        AdminView { mode: admin.mode }
    }

    fn update_time_view(&self, time: &Time, time_view: &TimeView) -> TimeView {
        TimeView {
            simulation_instant: time_view.simulation_instant,
            next_simulation_instant: time.simulation_instant,
        }
    }

    fn update_population_view(
        &self,
        population: &Population,
        population_view: &PopulationView,
    ) -> PopulationView {
        let mut next_population_view = PopulationView {
            tick: population.tick,
            judge_view: None,
            agent_views: HashMap::new(),
        };

        if let Some(judge) = population.get_judge() {
            next_population_view.judge_view =
                population_view
                    .judge_view
                    .as_ref()
                    .map(|judge_view| JudgeView {
                        id: judge.id,
                        tick: judge_view.tick,
                        position: judge_view.position,
                        orientation: judge_view.orientation,
                        next_tick: judge.tick,
                        next_position: judge.position,
                        next_orientation: judge.orientation,
                    });
        }

        for agent in population.all_agents() {
            if let Some(agent_view) = population_view.agent_views.get(&agent.id) {
                let next_agent_view = AgentView {
                    id: agent.id,
                    tick: agent_view.tick,
                    position: agent_view.position,
                    orientation: agent_view.orientation,
                    next_tick: agent.tick,
                    next_position: agent.position,
                    next_orientation: agent.orientation,
                };

                next_population_view
                    .agent_views
                    .insert(agent.id, next_agent_view);
            }
        }

        next_population_view
    }

    fn update_world_view(
        &self,
        entity: &Entity,
        world: &World,
        world_view: &WorldView,
    ) -> WorldView {
        if !entity.chunk_update {
            return world_view.clone();
        }

        let mut next_world_view = WorldView {
            tick: entity.tick,
            chunk_views: HashMap::new(),
        };

        let grid_position = World::grid_position_at(entity.position).unwrap();
        let chunk_position = Chunk::position_at(grid_position).unwrap();

        let x_range = (chunk_position.x - USER_VIEW_RADIUS)..=(chunk_position.x + USER_VIEW_RADIUS);
        let y_range = (chunk_position.y - USER_VIEW_RADIUS)..=(chunk_position.y + USER_VIEW_RADIUS);
        let z_range = (chunk_position.z - USER_VIEW_RADIUS)..=(chunk_position.z + USER_VIEW_RADIUS);

        for x in x_range {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let chunk_position = IVec3::new(x, y, z);

                    if let Some(chunk_id) = Chunk::id_at(chunk_position) {
                        if let Some(chunk) = world.get_chunk(chunk_id) {
                            let chunk_view = self.update_chunk_view(chunk, world, world_view);

                            next_world_view.chunk_views.insert(chunk.id, chunk_view);
                        }
                    }
                }
            }
        }

        next_world_view
    }

    fn update_chunk_view(
        &self,
        chunk: &chunk::Chunk,
        _world: &World,
        world_view: &WorldView,
    ) -> ChunkView {
        let chunk_view;

        if let Some(old_chunk_view) = world_view.chunk_views.get(&chunk.id) {
            if old_chunk_view.tick < chunk.tick {
                chunk_view = ChunkView {
                    id: chunk.id,
                    tick: chunk.tick,
                    position: chunk.position,
                    mesh: chunk.mesh.clone(),
                };
            } else {
                chunk_view = old_chunk_view.clone();
            }
        } else {
            chunk_view = ChunkView {
                id: chunk.id,
                tick: chunk.tick,
                position: chunk.position,
                mesh: chunk.mesh.clone(),
            };
        }

        chunk_view
    }
}
