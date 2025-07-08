pub mod face;
pub mod id;
pub mod kind;
pub mod meta;

pub use face::Face;
pub use id::ID;
pub use kind::Kind;
pub use meta::Meta;

use crate::simulation::state::world::chunk;
use glam::IVec3;

#[derive(Debug)]
pub struct Block {
    pub id: ID,
    pub chunk_id: chunk::ID,
    pub position: IVec3,
    pub kind: Kind,
    pub solid: bool,
}
