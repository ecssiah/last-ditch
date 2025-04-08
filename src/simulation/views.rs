pub mod buffer;
pub mod repository;
pub mod view;

use crate::simulation::{
    chunk, population::{entity, Entity}, state::{self, State}, views::{
        repository::Repository,
        view::{ChunkView, EntityView, View},
    }, world::World, Chunk, VIEW_RADIUS
};
use glam::{IVec3, Vec3};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Views {
    mode: Arc<RwLock<state::Mode>>,
    repository: repository::Repository,
}

impl Views {
    pub fn new() -> Self {
        let mode = Arc::new(RwLock::new(state::Mode::Simulating));
        let repository = Repository::new();

        let views = Self { mode, repository };

        views
    }

    pub fn generate(&mut self, state: &State) {
        if let Some(entity) = state.population.get(&entity::ID::USER_ENTITY) {
            self.register_entity(entity);
        }
    }

    pub fn tick(&mut self, state: &State) {
        match state.mode {
            state::Mode::Simulating => {
                for entity_id in self.repository.entity_ids() {
                    let entity = state.population.get(&entity_id).unwrap();
                    let view = self.repository.get(&entity_id).unwrap();

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
            state::Mode::Exit => {
                let mut mode = self.mode.write().unwrap();

                *mode = state::Mode::Exit;
            }
        }
    }

    pub fn register_entity(&mut self, entity: &Entity) {
        let view = View {
            entity_view: self.generate_entity_view(entity),
            chunk_views: HashMap::new(),
        };

        self.repository.add(entity.id, view);
    }

    pub fn get_mode(&self) -> state::Mode {
        let mode = self.mode.read().unwrap();

        mode.clone()
    }

    pub fn get_view(&self, entity_id: entity::ID) -> Option<View> {
        if let Some(view) = self.repository.get(&entity_id) {
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
        chunk_views: &HashMap<chunk::ID, ChunkView>,
    ) -> HashMap<chunk::ID, ChunkView> {
        let mut new_chunk_views = HashMap::new();

        let grid_position = World::world_position_at(position);
        let chunk_position = Chunk::position_at(grid_position).unwrap();

        let x_range = (chunk_position.x - VIEW_RADIUS)..=(chunk_position.x + VIEW_RADIUS);
        let y_range = (chunk_position.y - VIEW_RADIUS)..=(chunk_position.y + VIEW_RADIUS);
        let z_range = (chunk_position.z - VIEW_RADIUS)..=(chunk_position.z + VIEW_RADIUS);

        for x in x_range {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let chunk_position = IVec3::new(x, y, z);

                    if let Some(chunk_id) = Chunk::id_at(chunk_position) {
                        if let Some(chunk) = state.world.get_chunk(chunk_id) {
                            let chunk_view = self.generate_chunk_view(state, chunk, chunk_views);

                            new_chunk_views.insert(chunk.id, chunk_view);
                        }
                    }
                }
            }
        }

        new_chunk_views
    }

    fn generate_chunk_view(
        &self,
        state: &State,
        chunk: &chunk::Chunk,
        chunk_views: &HashMap<chunk::ID, ChunkView>,
    ) -> ChunkView {
        let chunk_view;

        if let Some(old_chunk_view) = chunk_views.get(&chunk.id) {
            if old_chunk_view.tick < chunk.tick {
                chunk_view = ChunkView {
                    id: chunk.id,
                    tick: state.time.get_clock_tick(),
                    position: chunk.position,
                    mesh: chunk.mesh.clone(),
                };
            } else {
                chunk_view = old_chunk_view.clone();
            }
        } else {
            chunk_view = ChunkView {
                id: chunk.id,
                tick: state.time.get_clock_tick(),
                position: chunk.position,
                mesh: chunk.mesh.clone(),
            };
        }

        chunk_view
    }
}
