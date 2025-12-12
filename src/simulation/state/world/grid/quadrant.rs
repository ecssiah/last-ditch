use ultraviolet::IVec3;

use crate::simulation::constants::WORLD_RADIUS_IN_CELLS;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Quadrant {
    NE,
    NW,
    SW,
    SE,
}

impl Quadrant {
    pub const ALL: [Self; 4] = [Self::NE, Self::NW, Self::SW, Self::SE];

    pub fn index(axis: Self) -> usize {
        axis as usize
    }

    pub fn min(quadrant: Quadrant) -> IVec3 {
        let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

        match quadrant {
            Quadrant::NE => IVec3::new(1, 1, 0),
            Quadrant::NW => IVec3::new(-world_radius_in_cells, 1, 0),
            Quadrant::SW => IVec3::new(-world_radius_in_cells, -world_radius_in_cells, 0),
            Quadrant::SE => IVec3::new(1, world_radius_in_cells, 0),
        }
    }

    pub fn max(quadrant: Quadrant) -> IVec3 {
        let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

        match quadrant {
            Quadrant::NE => IVec3::new(world_radius_in_cells, world_radius_in_cells, 0),
            Quadrant::NW => IVec3::new(-1, world_radius_in_cells, 0),
            Quadrant::SW => IVec3::new(-1, -1, 0),
            Quadrant::SE => IVec3::new(world_radius_in_cells, -1, 0),
        }
    }
}
