use crate::simulation::{
    self,
    state::world::{grid::Grid, sector},
    viewer::view::sector_view::SectorView,
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
            grid: Grid::new(simulation::Kind::Empty),
            sector_view_map: HashMap::new(),
        }
    }
}
