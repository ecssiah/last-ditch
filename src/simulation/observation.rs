pub mod buffer;
pub mod repository;
pub mod view;

use crate::simulation::{
    chunk,
    population::{entity, Entity},
    observation::{
        repository::Repository,
        view::{ChunkView, EntityView, View},
    },
    state::State,
    world::World,
};
use glam::{IVec3, Vec3};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

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

    pub fn generate(&mut self, state: &State) {
        if let Some(entity) = state.population.get(&entity::ID::USER_ENTITY) {
            self.register_entity(entity);
        }
    }

    pub fn tick(&mut self, state: &State) {
        if state.active {
            for entity_id in self.repository.list_entities() {
                if let Some(entity) = state.population.get(&entity_id) {
                    if let Some(view) = self.repository.get(entity_id) {
                        let entity_view = self.generate_entity_view(entity);
                        let chunk_views =
                            self.generate_chunk_views(state, entity.position, &view.chunk_views);

                        let new_view = View {
                            entity_view,
                            chunk_views,
                        };

                        self.repository.update(entity_id, new_view);
                    }
                }
            }
        } else {
            let mut status = self.status.write().unwrap();

            *status = Status::Shutdown;
        }
    }

    pub fn register_entity(&mut self, entity: &Entity) {
        let view = View {
            entity_view: self.generate_entity_view(entity),
            chunk_views: HashMap::new(),
        };

        self.repository.add(entity.id, view);
    }

    pub fn get_status(&self) -> Status {
        let status = self.status.read().unwrap();

        status.clone()
    }

    pub fn get_view(&self, entity_id: entity::ID) -> Option<View> {
        if let Some(view) = self.repository.get(entity_id) {
            Some((*view).clone())
        } else {
            None
        }
    }

    fn generate_entity_view(&self, entity: &Entity) -> EntityView {
        EntityView {
            id: entity.id,
            tick: entity.tick,
            position: entity.position,
            orientation: entity.orientation,
        }
    }

    fn generate_chunk_views(
        &self,
        state: &State,
        position: Vec3,
        old_chunk_views: &HashMap<chunk::ID, ChunkView>,
    ) -> HashMap<chunk::ID, ChunkView> {
        let mut new_chunk_views = HashMap::new();
        let grid_position = World::world_position_at(position);

        for x in (grid_position.x - 3)..=(grid_position.x + 3) {
            for y in (grid_position.y - 3)..=(grid_position.y + 3) {
                for z in (grid_position.z - 3)..=(grid_position.z + 3) {
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
