use crate::simulation::block;

#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub direction: block::Direction,
    pub visibility: Vec<block::Direction>,
    pub neighbors: block::Neighbors,
}
