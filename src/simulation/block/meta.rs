use crate::simulation::block;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Meta {
    pub direction: block::Direction,
    pub visibility: block::Face,
    pub neighbors: block::Neighbors,
}
