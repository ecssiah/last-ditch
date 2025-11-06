use crate::simulation::{
    self,
    observation::view::sector_view::SectorView,
    state::world::{grid::Grid, sector},
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub grid: Grid,
    pub sector_view_map: HashMap<sector::ID, SectorView>,
}

impl WorldView {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(simulation::Kind::EmptyWorld),
            sector_view_map: HashMap::new(),
        }
    }
}
