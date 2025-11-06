pub mod axis;
pub mod cell_sample;
pub mod direction;
pub mod world_ray_iterator;

pub use axis::Axis;
pub use cell_sample::CellSample;
pub use direction::Direction;
pub use world_ray_iterator::WorldRayIterator;

use crate::simulation::{
    self,
    consts::*,
    state::{
        physics::aabb::AABB,
        world::{cell, sector},
    },
    utils::indexing,
};
use glam::{IVec3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Grid {
    pub cell_radius_in_meters: f32,
    pub cell_size_in_meters: f32,
    pub cell_area_in_meters: f32,
    pub cell_volume_in_meters: f32,

    pub sector_radius_in_cells: u32,
    pub sector_size_in_cells: u32,
    pub sector_area_in_cells: u32,
    pub sector_volume_in_cells: u32,
    pub sector_radius_in_meters: f32,
    pub sector_size_in_meters: f32,
    pub sector_area_in_meters: f32,
    pub sector_volume_in_meters: f32,

    pub world_radius_in_sectors: u32,
    pub world_size_in_sectors: u32,
    pub world_area_in_sectors: u32,
    pub world_volume_in_sectors: u32,
    pub world_radius_in_cells: u32,
    pub world_size_in_cells: u32,
    pub world_area_in_cells: u32,
    pub world_volume_in_cells: u32,
    pub world_radius_in_meters: f32,
    pub world_size_in_meters: f32,
    pub world_area_in_meters: f32,
    pub world_volume_in_meters: f32,
}

impl Grid {
    pub fn new(simulation_kind: simulation::Kind) -> Self {
        let config = simulation_kind.config();

        let cell_radius_in_meters = CELL_RADIUS;
        let cell_size_in_meters = 2.0 * cell_radius_in_meters;
        let cell_area_in_meters = cell_size_in_meters.powi(2);
        let cell_volume_in_meters = cell_size_in_meters.powi(3);

        let sector_radius_in_cells = config.sector_radius_in_cells;
        let sector_size_in_cells = 2 * sector_radius_in_cells + 1;
        let sector_area_in_cells = sector_size_in_cells.pow(2);
        let sector_volume_in_cells = sector_size_in_cells.pow(3);

        let sector_radius_in_meters =
            sector_radius_in_cells as f32 * cell_size_in_meters + cell_radius_in_meters;
        let sector_size_in_meters = 2.0 * sector_radius_in_meters;
        let sector_area_in_meters = sector_size_in_meters.powi(2);
        let sector_volume_in_meters = sector_size_in_meters.powi(3);

        let world_radius_in_sectors = config.world_radius_in_sectors;
        let world_size_in_sectors = 2 * world_radius_in_sectors + 1;
        let world_area_in_sectors = world_size_in_sectors.pow(2);
        let world_volume_in_sectors = world_size_in_sectors.pow(3);

        let world_radius_in_cells =
            world_radius_in_sectors * sector_size_in_cells + sector_radius_in_cells;
        let world_size_in_cells = 2 * world_radius_in_cells + 1;
        let world_area_in_cells = world_size_in_cells.pow(2);
        let world_volume_in_cells = world_size_in_cells.pow(3);

        let world_radius_in_meters =
            world_radius_in_cells as f32 * cell_size_in_meters + cell_radius_in_meters;
        let world_size_in_meters = 2.0 * world_radius_in_meters;
        let world_area_in_meters = world_size_in_meters.powi(2);
        let world_volume_in_meters = world_size_in_meters.powi(3);

        Self {
            cell_radius_in_meters,
            cell_size_in_meters,
            cell_area_in_meters,
            cell_volume_in_meters,

            sector_radius_in_cells,
            sector_size_in_cells,
            sector_area_in_cells,
            sector_volume_in_cells,
            sector_radius_in_meters,
            sector_size_in_meters,
            sector_area_in_meters,
            sector_volume_in_meters,

            world_radius_in_cells,
            world_size_in_cells,
            world_area_in_cells,
            world_volume_in_cells,
            world_radius_in_sectors,
            world_size_in_sectors,
            world_area_in_sectors,
            world_volume_in_sectors,
            world_radius_in_meters,
            world_size_in_meters,
            world_area_in_meters,
            world_volume_in_meters,
        }
    }

    pub fn cell_ids(grid: &Grid) -> Vec<cell::ID> {
        (0u32..grid.sector_volume_in_cells).map(cell::ID).collect()
    }

    pub fn sector_ids(grid: &Grid) -> Vec<sector::ID> {
        (0u32..grid.world_volume_in_sectors)
            .map(sector::ID)
            .collect()
    }

    pub fn sector_id_valid(grid: &Grid, sector_id: sector::ID) -> bool {
        (0u32..grid.world_volume_in_sectors).contains(&u32::from(sector_id))
    }

    pub fn cell_id_valid(grid: &Grid, cell_id: cell::ID) -> bool {
        (0u32..grid.sector_volume_in_cells).contains(&u32::from(cell_id))
    }

    pub fn position_valid(grid: &Grid, position: IVec3) -> bool {
        let in_x_range = position.x.unsigned_abs() <= grid.world_radius_in_cells;
        let in_y_range = position.y.unsigned_abs() <= grid.world_radius_in_cells;
        let in_z_range = position.z.unsigned_abs() <= grid.world_radius_in_cells;

        in_x_range && in_y_range && in_z_range
    }

    pub fn cell_id_to_cell_coordinates(grid: &Grid, cell_id: cell::ID) -> IVec3 {
        if Grid::cell_id_valid(grid, cell_id) {
            let cell_index = u32::from(cell_id);
            let cell_coordinates =
                indexing::index_to_vector(cell_index, grid.sector_radius_in_cells);

            cell_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn cell_coordinates_to_cell_id(grid: &Grid, cell_coordinates: IVec3) -> cell::ID {
        let cell_coordinates_indexable =
            indexing::indexable_vector(cell_coordinates, grid.sector_radius_in_cells);

        if cell_coordinates_indexable != IVec3::MAX {
            let cell_index =
                indexing::vector_to_index(cell_coordinates_indexable, grid.sector_radius_in_cells);

            cell::ID(cell_index)
        } else {
            cell::ID::MAX
        }
    }

    pub fn sector_id_to_sector_coordinates(grid: &Grid, sector_id: sector::ID) -> IVec3 {
        if Grid::sector_id_valid(grid, sector_id) {
            let sector_index = u32::from(sector_id);
            let sector_coordinates =
                indexing::index_to_vector(sector_index, grid.world_radius_in_sectors);

            sector_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn sector_coordinates_to_sector_id(grid: &Grid, sector_coordinates: IVec3) -> sector::ID {
        let sector_coordinates_indexable =
            indexing::indexable_vector(sector_coordinates, grid.world_radius_in_sectors);

        if sector_coordinates_indexable != IVec3::MAX {
            let sector_index = indexing::vector_to_index(
                sector_coordinates_indexable,
                grid.world_radius_in_sectors,
            );

            sector::ID(sector_index)
        } else {
            sector::ID::MAX
        }
    }

    pub fn sector_coordinates_to_position(grid: &Grid, sector_coordinates: IVec3) -> IVec3 {
        let position = sector_coordinates * grid.sector_size_in_cells as i32;

        if Grid::position_valid(grid, position) {
            position
        } else {
            IVec3::MAX
        }
    }

    pub fn sector_id_to_position(grid: &Grid, sector_id: sector::ID) -> IVec3 {
        let sector_coordinates = Grid::sector_id_to_sector_coordinates(grid, sector_id);

        if sector_coordinates != IVec3::MAX {
            Grid::sector_coordinates_to_position(grid, sector_coordinates)
        } else {
            IVec3::MAX
        }
    }

    pub fn position_to_sector_coordinates(grid: &Grid, position: IVec3) -> IVec3 {
        if Grid::position_valid(grid, position) {
            let position_indexable =
                indexing::indexable_vector(position, grid.world_radius_in_cells);

            if position_indexable != IVec3::MAX {
                let sector_coordinates_indexable =
                    position_indexable / grid.sector_size_in_cells as i32;

                let sector_coordinates = sector_coordinates_indexable
                    - IVec3::splat(grid.world_radius_in_sectors as i32);

                sector_coordinates
            } else {
                IVec3::MAX
            }
        } else {
            IVec3::MAX
        }
    }

    pub fn position_to_cell_coordinates(grid: &Grid, position: IVec3) -> IVec3 {
        if Grid::position_valid(grid, position) {
            let position_indexable =
                indexing::indexable_vector(position, grid.world_radius_in_cells);

            if position_indexable != IVec3::MAX {
                let cell_coordinates_indexable =
                    position_indexable % grid.sector_size_in_cells as i32;

                let cell_coordinates =
                    cell_coordinates_indexable - IVec3::splat(grid.sector_radius_in_cells as i32);

                cell_coordinates
            } else {
                IVec3::MAX
            }
        } else {
            IVec3::MAX
        }
    }

    pub fn position_to_sector_id(grid: &Grid, position: IVec3) -> sector::ID {
        let sector_coordinates = Grid::position_to_sector_coordinates(grid, position);

        if sector_coordinates != IVec3::MAX {
            Grid::sector_coordinates_to_sector_id(grid, sector_coordinates)
        } else {
            sector::ID::MAX
        }
    }

    pub fn position_to_cell_id(grid: &Grid, position: IVec3) -> cell::ID {
        let cell_coordinates = Grid::position_to_cell_coordinates(grid, position);

        if cell_coordinates != IVec3::MAX {
            Grid::cell_coordinates_to_cell_id(grid, cell_coordinates)
        } else {
            cell::ID::MAX
        }
    }

    pub fn position_to_ids(grid: &Grid, position: IVec3) -> (sector::ID, cell::ID) {
        let sector_id = Grid::position_to_sector_id(grid, position);
        let cell_id = Grid::position_to_cell_id(grid, position);

        (sector_id, cell_id)
    }

    pub fn ids_to_position(grid: &Grid, sector_id: sector::ID, cell_id: cell::ID) -> IVec3 {
        let sector_coordinates = Grid::sector_id_to_sector_coordinates(grid, sector_id);
        let cell_coordinates = Grid::cell_id_to_cell_coordinates(grid, cell_id);

        if sector_coordinates != IVec3::MAX && cell_coordinates != IVec3::MAX {
            grid.sector_size_in_cells as i32 * sector_coordinates + cell_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn world_to_position(grid: &Grid, world_position: Vec3) -> IVec3 {
        let position = (world_position + Vec3::splat(0.5)).floor().as_ivec3();

        if Grid::position_valid(grid, position) {
            position
        } else {
            IVec3::MAX
        }
    }

    pub fn world_to_sector_id(grid: &Grid, world_position: Vec3) -> sector::ID {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_sector_id(grid, position)
        } else {
            sector::ID::MAX
        }
    }

    pub fn world_to_sector_coordinates(grid: &Grid, world_position: Vec3) -> IVec3 {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_sector_coordinates(grid, position)
        } else {
            IVec3::MAX
        }
    }

    pub fn world_to_cell_id(grid: &Grid, world_position: Vec3) -> cell::ID {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_cell_id(grid, position)
        } else {
            cell::ID::MAX
        }
    }

    pub fn world_to_cell_coordinates(grid: &Grid, world_position: Vec3) -> IVec3 {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_cell_coordinates(grid, position)
        } else {
            IVec3::MAX
        }
    }

    pub fn cells_overlapping(grid: &Grid, aabb: AABB) -> Vec<AABB> {
        let mut aabb_vec = Vec::new();

        let min = aabb.min.round().as_ivec3();
        let max = aabb.max.round().as_ivec3();

        let size = Vec3::splat(grid.cell_size_in_meters);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let cell_position = IVec3::new(x, y, z);
                    let cell_aabb = AABB::new(cell_position.as_vec3(), size);

                    if cell_aabb.overlaps(aabb) {
                        aabb_vec.push(cell_aabb);
                    }
                }
            }
        }

        aabb_vec
    }

    pub fn on_sector_boundary(grid: &Grid, position: IVec3) -> bool {
        let cell_coordinates = Grid::position_to_cell_coordinates(grid, position);

        if cell_coordinates == IVec3::MAX {
            true
        } else {
            let sector_radius_in_cells = grid.sector_radius_in_cells as i32;

            cell_coordinates.x.abs() == sector_radius_in_cells
                || cell_coordinates.y.abs() == sector_radius_in_cells
                || cell_coordinates.z.abs() == sector_radius_in_cells
        }
    }

    pub fn on_world_radius(grid: &Grid, position: IVec3) -> bool {
        let world_radius_in_cells = grid.world_radius_in_cells as i32;

        position.x.abs() == world_radius_in_cells
            || position.y.abs() == world_radius_in_cells
            || position.z.abs() == world_radius_in_cells
    }

    pub fn offsets_in(radius: i32) -> impl Iterator<Item = IVec3> {
        (-radius..=radius).flat_map(move |x| {
            (-radius..=radius)
                .flat_map(move |y| (-radius..=radius).map(move |z| IVec3::new(x, y, z)))
        })
    }
}
