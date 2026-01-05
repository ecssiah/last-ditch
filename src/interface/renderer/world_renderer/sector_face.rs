use crate::{
    interface::renderer::texture::texture_location::TextureLocation,
    simulation::state::world::grid::{self, Direction},
};

#[derive(Clone, PartialEq)]
pub struct SectorFace {
    pub direction: Direction,
    pub texture_location: TextureLocation,
}

impl SectorFace {
    pub fn new() -> Self {
        Self {
            direction: grid::Direction::East,
            texture_location: TextureLocation::new(0, 0),
        }
    }
}

impl Default for SectorFace {
    fn default() -> Self {
        Self::new()
    }
}
