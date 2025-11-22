pub mod id;

pub use id::ID;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{
        cell::{self, Cell},
        grid::Grid,
        sector,
    },
};
use ultraviolet::IVec3;

pub struct Sector {
    pub sector_id: sector::ID,
    pub version: u64,
    pub position: IVec3,
    pub aabb: AABB,
    pub cell_vec: Vec<Cell>,
}

impl Sector {
    pub fn get_cell<'a>(cell_id: cell::ID, sector: &'a Sector) -> &'a Cell {
        &sector.cell_vec[usize::from(cell_id)]
    }

    pub fn get_cell_mut<'a>(cell_id: cell::ID, sector: &'a mut Sector) -> &'a mut Cell {
        &mut sector.cell_vec[usize::from(cell_id)]
    }

    pub fn get_cell_at<'a>(coordinates: IVec3, grid: &Grid, sector: &'a Sector) -> &'a Cell {
        let cell_id = Grid::cell_coordinates_to_cell_id(coordinates, grid);

        Self::get_cell(cell_id, sector)
    }

    pub fn get_cell_at_mut<'a>(
        coordinates: IVec3,
        grid: &Grid,
        sector: &'a mut Sector,
    ) -> &'a mut Cell {
        let cell_id = Grid::cell_coordinates_to_cell_id(coordinates, grid);

        Self::get_cell_mut(cell_id, sector)
    }
}
