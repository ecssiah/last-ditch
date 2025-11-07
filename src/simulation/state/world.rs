//! The simulated environment

pub mod block;
pub mod cell;
pub mod grid;
pub mod sector;

use crate::simulation::{
    self,
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{
            entity::{self},
            nation,
        },
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
                let position = Grid::sector_id_to_position(grid, sector_id);
                let modified = sector::Modified::new();

                let aabb = AABB::new(
                    Vec3::from(position),
                    Vec3::broadcast(grid.sector_size_in_cells as f32),
                );

                let cell_vec = Self::setup_cell_vec(grid, sector_id);

                sector::Sector {
                    sector_id,
                    modified,
                    position,
                    aabb,
                    cell_vec,
                }
            })
            .collect()
    }

    fn setup_cell_vec(grid: &Grid, sector_id: sector::ID) -> Vec<Cell> {
        Grid::cell_ids(grid)
            .into_iter()
            .map(|cell_id| {
                let position = Grid::ids_to_position(grid, sector_id, cell_id);

                Cell {
                    cell_id,
                    sector_id,
                    position,
                    block_kind: block::Kind::None,
                    solid: false,
                    face_array: Cell::face_array(),
                }
            })
            .collect()
    }

    pub fn get_sector<'a>(
        sector_id: sector::ID,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a sector::Sector> {
        sector_vec_slice.get(usize::from(sector_id))
    }

    pub fn get_sector_at<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a sector::Sector> {
        let sector_id = Grid::position_to_sector_id(grid, position);

        if sector_id != sector::ID::MAX {
            sector_vec_slice.get(usize::from(sector_id))
        } else {
            None
        }
    }

    pub fn get_sector_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut sector::Sector> {
        let sector_id = Grid::position_to_sector_id(grid, position);

        if sector_id != sector::ID::MAX {
            sector_vec_slice.get_mut(usize::from(sector_id))
        } else {
            None
        }
    }

    pub fn get_cell(
        sector_id: sector::ID,
        cell_id: cell::ID,
        sector_vec_slice: &[Sector],
    ) -> Option<&Cell> {
        let sector = sector_vec_slice.get(usize::from(sector_id))?;

        sector.cell_vec.get(usize::from(cell_id))
    }

    pub fn get_cell_mut(
        sector_id: sector::ID,
        cell_id: cell::ID,
        sector_vec_slice: &mut [Sector],
    ) -> Option<&mut Cell> {
        let sector = sector_vec_slice.get_mut(usize::from(sector_id))?;

        sector.cell_vec.get_mut(usize::from(cell_id))
    }

    pub fn get_cell_at<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a Cell> {
        let (sector_id, cell_id) = Grid::position_to_ids(grid, position);

        if sector_id != sector::ID::MAX && cell_id != cell::ID::MAX {
            Self::get_cell(sector_id, cell_id, sector_vec_slice)
        } else {
            None
        }
    }

    pub fn get_cell_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut Cell> {
        let (sector_id, cell_id) = Grid::position_to_ids(grid, position);

        if sector_id != sector::ID::MAX && cell_id != cell::ID::MAX {
            Self::get_cell_mut(sector_id, cell_id, sector_vec_slice)
        } else {
            None
        }
    }

    pub fn get_clearance(position: IVec3, grid: &Grid, sector_vec_slice: &[Sector]) -> u32 {
        let ground_is_solid =
            Self::get_cell_at(position + -1 * IVec3::unit_y(), grid, sector_vec_slice)
                .is_some_and(|cell| cell.solid);

        let mut clearance = 0;

        if ground_is_solid {
            for level in 0..MAXIMUM_CLEARANCE {
                let level_position = position + IVec3::new(0, level as i32, 0);

                if let Some(cell) = Self::get_cell_at(level_position, grid, sector_vec_slice) {
                    if !cell.solid {
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

    fn mark_updates(position1: IVec3, grid: &Grid, sector_vec_slice: &mut [Sector]) {
        let mut sector1_id = None;

        if let Some(sector1) = Self::get_sector_at_mut(position1, grid, sector_vec_slice) {
            sector1_id = Some(sector1.sector_id);
            sector1.modified.cell = true;

            if Grid::on_sector_boundary(grid, position1) {
                sector1.modified.boundary = true;
            }
        }

        if let Some(sector1_id) = sector1_id {
            for direction_offset in grid::Direction::face_offset_array() {
                let position2 = position1 + direction_offset;
                let sector_id2 = Grid::position_to_sector_id(grid, position2);

                if sector_id2 != sector::ID::MAX && sector1_id != sector_id2 {
                    if let Some(sector2) = sector_vec_slice.get_mut(usize::from(sector_id2)) {
                        sector2.modified.boundary = true;
                    }
                }
            }
        }
    }

    pub fn set_block(
        position: IVec3,
        block_kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
        sector_vec_slice: &mut [Sector],
    ) -> bool {
        let (sector_id, cell_id) = Grid::position_to_ids(grid, position);

        if sector_id != sector::ID::MAX && cell_id != cell::ID::MAX {
            let block_info = block_info_map.get(&block_kind).cloned().unwrap();

            if let Some(cell) = Self::get_cell_mut(sector_id, cell_id, sector_vec_slice) {
                cell.block_kind = block_kind;
                cell.solid = block_info.solid;
            }

            Self::mark_updates(position, grid, sector_vec_slice);

            true
        } else {
            log::info!(
                "{:?} cell cannot be set at invalid location: {:?}",
                block_kind,
                position
            );

            false
        }
    }

    pub fn set_box(
        position1: IVec3,
        position2: IVec3,
        block_kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
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

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
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
                            grid,
                            block_info_map,
                            sector_vec_slice,
                        );
                    } else {
                        Self::set_block(
                            position,
                            block::Kind::None,
                            grid,
                            block_info_map,
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
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
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

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let position = IVec3::new(x, y, z);

                    Self::set_block(position, block_kind, grid, block_info_map, sector_vec_slice);
                }
            }
        }
    }

    pub fn update_sectors(grid: &Grid, sector_vec_slice: &mut [Sector]) {
        for sector_id in Grid::sector_ids(grid) {
            Self::update_sector(grid, sector_id, sector_vec_slice);
        }
    }

    fn update_sector(grid: &Grid, sector_id: sector::ID, sector_vec_slice: &mut [Sector]) {
        if Self::get_sector(sector_id, sector_vec_slice)
            .map_or(false, |sector| sector.modified.cell)
        {
            for cell_id in Grid::cell_ids(grid) {
                let cell = Self::get_cell(sector_id, cell_id, sector_vec_slice)
                    .expect("All grid cells should exist");

                let face_exposure =
                    Self::compute_face_exposure(cell.position, grid, sector_vec_slice);

                if let Some(cell) = Self::get_cell_mut(sector_id, cell_id, sector_vec_slice) {
                    for (face, &exposed) in cell.face_array.iter_mut().zip(face_exposure.iter()) {
                        face.exposed = exposed;
                    }
                }
            }
        }
    }

    fn compute_face_exposure(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &[Sector],
    ) -> [bool; 6] {
        let mut face_exposure = [false; 6];

        for (index, direction) in grid::Direction::face_array().iter().enumerate() {
            let neighbor_pos = position + direction.offset();

            if let Some(neighbor_cell) = World::get_cell_at(neighbor_pos, grid, sector_vec_slice) {
                face_exposure[index] = neighbor_cell.block_kind == block::Kind::None;
            } else {
                face_exposure[index] = true;
            }
        }

        face_exposure
    }
}
