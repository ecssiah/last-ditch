use crate::simulation::state::{population::entity, world::chunk};

pub struct Info {
    pub entity_id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub kind: entity::Kind,
    pub nation: entity::Nation,
}
