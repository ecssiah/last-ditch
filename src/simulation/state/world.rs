//! The simulated environment

pub mod block;
pub mod cell;
pub mod grid;
pub mod sector;

use crate::simulation::{
    self,
    constants::*,
    state::{
        physics::aabb::AABB,
        population::entity::{self, nation},
        world::{cell::Cell, grid::Grid, sector::Sector},
    },
};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct World {
    pub simulation_kind: simulation::Kind,
    pub grid: Grid,
    pub block_info_map: HashMap<block::Kind, block::Info>,
    pub sector_vec: Vec<sector::Sector>,
    pub flag_position_map: HashMap<nation::Kind, IVec3>,
}

impl World {
    pub fn new(simulation_kind: simulation::Kind) -> Self {
        let grid = Grid::new(simulation_kind);
        let block_info_map = block::Info::setup();
        let sector_vec = Self::setup_sector_vec(&grid);

        let flag_position_map = HashMap::from([
            (nation::Kind::Lion, IVec3::new(0, 0, 0)),
            (nation::Kind::Eagle, IVec3::new(0, 0, 0)),
            (nation::Kind::Horse, IVec3::new(0, 0, 0)),
            (nation::Kind::Wolf, IVec3::new(0, 0, 0)),
        ]);

        Self {
            simulation_kind,
            grid,
            block_info_map,
            sector_vec,
            flag_position_map,
        }
    }

    pub fn placeholder() -> Self {
        let simulation_kind = simulation::Kind::Placeholder;

        let grid = Grid::new(simulation_kind);
        let block_info_map = HashMap::default();
        let sector_vec = Vec::default();

        let flag_position_map = HashMap::default();

        Self {
            simulation_kind,
            grid,
            block_info_map,
            sector_vec,
            flag_position_map,
        }
    }

    pub fn get_flag(
        kind: entity::Kind,
        flag_position_map: HashMap<entity::Kind, IVec3>,
    ) -> Option<IVec3> {
        flag_position_map.get(&kind).cloned()
    }

    fn setup_sector_vec(grid: &Grid) -> Vec<sector::Sector> {
        Grid::sector_ids(grid)
            .into_iter()
            .map(|sector_id| {
                let position = Grid::sector_id_to_position(sector_id, grid);
                let version = 0;

                let aabb = AABB::new(
                    Vec3::from(position),
                    Vec3::broadcast(grid.sector_size_in_cells as f32),
                );

                let cell_vec = Self::setup_cell_vec(sector_id, grid);

                sector::Sector {
                    sector_id,
                    version,
                    position,
                    aabb,
                    cell_vec,
                }
            })
            .collect()
    }

    fn setup_cell_vec(sector_id: sector::ID, grid: &Grid) -> Vec<Cell> {
        Grid::cell_ids(grid)
            .into_iter()
            .map(|cell_id| {
                let position = Grid::ids_to_position(sector_id, cell_id, grid);

                Cell {
                    cell_id,
                    sector_id,
                    position,
                    block_kind: block::Kind::None,
                    solid: false,
                }
            })
            .collect()
    }

    pub fn get_sector<'a>(
        sector_id: sector::ID,
        sector_vec_slice: &'a [Sector],
    ) -> &'a sector::Sector {
        &sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_sector_mut<'a>(
        sector_id: sector::ID,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut sector::Sector {
        &mut sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_sector_at<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a [Sector],
    ) -> &'a sector::Sector {
        let sector_id = Grid::position_to_sector_id(position, grid);

        &sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_sector_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut sector::Sector {
        let sector_id = Grid::position_to_sector_id(position, grid);

        &mut sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_cell(
        sector_id: sector::ID,
        cell_id: cell::ID,
        sector_vec_slice: &[Sector],
    ) -> &Cell {
        let sector = &sector_vec_slice[sector_id.to_usize()];

        &sector.cell_vec[cell_id.to_usize()]
    }

    pub fn get_cell_mut(
        sector_id: sector::ID,
        cell_id: cell::ID,
        sector_vec_slice: &mut [Sector],
    ) -> &mut Cell {
        let sector = &mut sector_vec_slice[sector_id.to_usize()];

        &mut sector.cell_vec[cell_id.to_usize()]
    }

    pub fn get_cell_at<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a [Sector],
    ) -> &'a Cell {
        let (sector_id, cell_id) = Grid::position_to_ids(position, grid);

        Self::get_cell(sector_id, cell_id, sector_vec_slice)
    }

    pub fn get_cell_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut Cell {
        let (sector_id, cell_id) = Grid::position_to_ids(position, grid);

        Self::get_cell_mut(sector_id, cell_id, sector_vec_slice)
    }

    pub fn get_clearance(position: IVec3, grid: &Grid, sector_vec_slice: &[Sector]) -> u32 {
        let ground_position = position + -1 * IVec3::unit_z();

        let ground_solid = if Grid::position_valid(ground_position, grid) {
            Self::get_cell_at(ground_position, grid, sector_vec_slice).solid
        } else {
            false
        };

        let mut clearance = 0;

        if ground_solid {
            for level in 0..MAXIMUM_CLEARANCE {
                let level_position = position + IVec3::new(0, 0, level as i32);

                if Grid::position_valid(level_position, grid) {
                    if !Self::get_cell_at(level_position, grid, sector_vec_slice).solid {
                        clearance += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        clearance
    }

    pub fn set_block(
        position: IVec3,
        block_kind: block::Kind,
        block_info_map: &HashMap<block::Kind, block::Info>,
        grid: &Grid,
        sector_vec_slice: &mut [Sector],
    ) {
        let (sector_id, cell_id) = Grid::position_to_ids(position, grid);

        if Grid::sector_id_valid(sector_id, grid) && Grid::cell_id_valid(cell_id, grid) {
            let block_info = block_info_map[&block_kind];

            let cell = Self::get_cell_mut(sector_id, cell_id, sector_vec_slice);
            cell.block_kind = block_kind;
            cell.solid = block_info.solid;

            let sector = Self::get_sector_mut(sector_id, sector_vec_slice);
            sector.version += 1;
        }
    }

    pub fn set_box(
        position1: IVec3,
        position2: IVec3,
        block_kind: block::Kind,
        block_info_map: &HashMap<block::Kind, block::Info>,
        grid: &Grid,
        sector_vec_slice: &mut [Sector],
    ) {
        let min = IVec3::new(
            position1.x.min(position2.x),
            position1.y.min(position2.y),
            position1.z.min(position2.z),
        );

        let max = IVec3::new(
            position1.x.max(position2.x),
            position1.y.max(position2.y),
            position1.z.max(position2.z),
        );

        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let mut on_boundary = false;

                    if min.x != max.x && (x == min.x || x == max.x) {
                        on_boundary = true;
                    }

                    if min.y != max.y && (y == min.y || y == max.y) {
                        on_boundary = true;
                    }

                    if min.z != max.z && (z == min.z || z == max.z) {
                        on_boundary = true;
                    }

                    let position = IVec3::new(x, y, z);

                    if on_boundary {
                        Self::set_block(
                            position,
                            block_kind,
                            block_info_map,
                            grid,
                            sector_vec_slice,
                        );
                    } else {
                        Self::set_block(
                            position,
                            block::Kind::None,
                            block_info_map,
                            grid,
                            sector_vec_slice,
                        );
                    }
                }
            }
        }
    }

    pub fn set_cube(
        position1: IVec3,
        position2: IVec3,
        block_kind: block::Kind,
        block_info_map: &HashMap<block::Kind, block::Info>,
        grid: &Grid,
        sector_vec_slice: &mut [Sector],
    ) {
        let min = IVec3::new(
            position1.x.min(position2.x),
            position1.y.min(position2.y),
            position1.z.min(position2.z),
        );

        let max = IVec3::new(
            position1.x.max(position2.x),
            position1.y.max(position2.y),
            position1.z.max(position2.z),
        );

        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_block(position, block_kind, block_info_map, grid, sector_vec_slice);
                }
            }
        }
    }
}
