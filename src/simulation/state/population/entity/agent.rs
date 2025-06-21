use glam::Vec3;

use crate::simulation::{
    observation::state_pair::StatePair,
    state::{
        physics::aabb::AABB,
        population::entity::{self, Kinematic, Nation, Spatial},
        world::{chunk, World},
    },
};

pub struct Agent {
    pub id: entity::ID,
    pub chunk_id: StatePair<chunk::ID>,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub kind: entity::Kind,
    pub nation: Nation,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            id: entity::ID::allocate(),
            chunk_id: StatePair::new(chunk::ID::zero(), chunk::ID::zero()),
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            kind: entity::Kind::Eagle,
            nation: Nation {
                kind: entity::Kind::Eagle,
            },
        }
    }

    pub fn tick(&mut self, world: &World) {
        if let Some(chunk_id) = world.grid.world_to_chunk_id(self.spatial.world_position) {
            self.chunk_id.set(chunk_id);
        }
    }

    pub fn chunk_updated(&self) -> bool {
        self.chunk_id.current != self.chunk_id.next
    }

    pub fn set_world_position(&mut self, world_position: Vec3) {
        self.spatial.world_position = world_position;

        self.spatial
            .aabb
            .set_bottom_center(world_position.x, world_position.y, world_position.z);
    }

    pub fn set_size(&mut self, size: Vec3) {
        self.spatial.aabb = AABB::new(self.spatial.aabb.center(), size);
    }
}
