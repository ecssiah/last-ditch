use crate::simulation::state::{
    physics::aabb::AABB,
    world::{cell::Cell, grid},
};
use ultraviolet::IVec3;

pub struct Sector {
    pub sector_id: usize,
    pub version: u64,
    pub position: IVec3,
    pub aabb: AABB,
    pub cell_vec: Vec<Cell>,
}

impl Sector {
    pub fn get_cell<'a>(cell_id: usize, sector: &'a Sector) -> &'a Cell {
        let cell = &sector.cell_vec[cell_id];

        cell
    }

    pub fn get_cell_mut<'a>(cell_id: usize, sector: &'a mut Sector) -> &'a mut Cell {
        let cell = &mut sector.cell_vec[cell_id];

        cell
    }

    pub fn get_cell_at<'a>(cell_coordinate: IVec3, sector: &'a Sector) -> &'a Cell {
        let cell_id = grid::cell_coordinate_to_cell_id(cell_coordinate);

        let cell = Self::get_cell(cell_id, sector);

        cell
    }

    pub fn get_cell_at_mut<'a>(cell_coordinate: IVec3, sector: &'a mut Sector) -> &'a mut Cell {
        let cell_id = grid::cell_coordinate_to_cell_id(cell_coordinate);

        let cell = Self::get_cell_mut(cell_id, sector);

        cell
    }
}
