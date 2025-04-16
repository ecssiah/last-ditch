pub mod buffer;
pub mod repository;
pub mod view;

use crate::simulation::{
    admin::{self, Admin},
    chunk,
    observation::{
        repository::Repository,
        view::{AdminView, ChunkView, EntityView, PopulationView, TimeView, View, WorldView},
    },
    population::{entity, Entity, Population},
    state::State,
    time::Time,
    world::World,
    Chunk, USER_VIEW_RADIUS,
};
use glam::IVec3;
use std::collections::HashMap;

pub struct Observation {
    repository: repository::Repository,
}

impl Observation {
    pub fn new() -> Self {
        let repository = Repository::new();

        let observation = Self { repository };

        observation
    }

    pub fn generate_view(&self, entity_id: &entity::ID, state: &State) {
        if let Some(entity) = state.population.get(entity_id) {
            let admin_view = self.generate_admin_view(&state.admin);

            let time_view = self.generate_time_view(&state.time);

            let population_view = self.generate_population_view(&entity, &state.population);

            let world_view = self.generate_world_view(&entity, &state.world);

            let next_view = View {
                entity_id: entity.id,
                admin_view,
                time_view,
                population_view,
                world_view,
            };

            self.repository.set(&entity.id, next_view);
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

    fn generate_population_view(
        &self,
        _entity: &Entity,
        population: &Population,
    ) -> PopulationView {
        let population_view = PopulationView {
            tick: population.tick,
            entity_views: population
                .entities
                .iter()
                .map(|(entity_id, entity)| {
                    let entity_view = EntityView {
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
                .collect(),
        };

        population_view
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

    pub fn tick(&mut self, state: &State) {
        match state.admin.mode {
            admin::Mode::Load => {}
            admin::Mode::Simulate => {
                self.update_view(&entity::ID::USER_ENTITY1, &state);
            }
            admin::Mode::Shutdown => {}
            admin::Mode::Exit => {}
        }
    }

    pub fn get_view(&self, entity_id: &entity::ID) -> Option<View> {
        if let Some(view) = self.repository.get(&entity_id) {
            Some((*view).clone())
        } else {
            None
        }
    }

    fn update_view(&self, entity_id: &entity::ID, state: &State) {
        if let Some(entity) = state.population.get(entity_id) {
            let view = self.repository.get(&entity_id).unwrap();

            let admin_view = self.update_admin_view(&state.admin);

            let time_view = self.update_time_view(&state.time, &view.time_view);

            let population_view =
                self.update_population_view(&entity, &state.population, &view.population_view);

            let world_view = self.update_world_view(&entity, &state.world, &view.world_view);

            let next_view = View {
                entity_id: entity.id,
                admin_view,
                time_view,
                population_view,
                world_view,
            };

            self.repository.set(&entity.id, next_view);
        }
    }

    fn update_admin_view(&self, admin: &Admin) -> AdminView {
        AdminView { mode: admin.mode }
    }

    fn update_time_view(&self, time: &Time, time_view: &TimeView) -> TimeView {
        TimeView {
            simulation_instant: time.simulation_instant,
            next_simulation_instant: time_view.next_simulation_instant,
        }
    }

    fn update_population_view(
        &self,
        _entity: &Entity,
        population: &Population,
        population_view: &PopulationView,
    ) -> PopulationView {
        let next_population_view = PopulationView {
            tick: population.tick,
            entity_views: population
                .entities
                .iter()
                .filter_map(|(entity_id, entity)| {
                    population_view
                        .entity_views
                        .get(entity_id)
                        .map(|entity_view| {
                            let entity_view = EntityView {
                                id: entity.id,
                                tick: entity_view.tick,
                                position: entity_view.position,
                                orientation: entity_view.orientation,
                                next_tick: entity.tick,
                                next_position: entity.position,
                                next_orientation: entity.orientation,
                            };

                            (*entity_id, entity_view)
                        })
                })
                .collect(),
        };

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
