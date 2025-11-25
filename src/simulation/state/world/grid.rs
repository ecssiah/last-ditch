pub mod axis;
pub mod cell_sample;
pub mod direction;
pub mod world_ray_iterator;

pub use cell_sample::CellSample;
pub use direction::Direction;
pub use world_ray_iterator::WorldRayIterator;

use crate::{
    simulation::{
        self,
        constants::*,
        state::{
            physics::aabb::AABB,
            world::{cell, sector},
        },
    },
    utils::ld_math::indexing::Indexing,
};
use ultraviolet::{IVec3, Vec3};

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

    pub fn cell_id_valid(id: cell::ID, grid: &Grid) -> bool {
        (0u32..grid.sector_volume_in_cells).contains(&u32::from(id))
    }

    pub fn sector_id_valid(id: sector::ID, grid: &Grid) -> bool {
        (0u32..grid.world_volume_in_sectors).contains(&u32::from(id))
    }

    pub fn cell_coordinates_valid(coordinates: IVec3, grid: &Grid) -> bool {
        let in_x_range = coordinates.x.unsigned_abs() <= grid.sector_radius_in_cells;
        let in_y_range = coordinates.y.unsigned_abs() <= grid.sector_radius_in_cells;
        let in_z_range = coordinates.z.unsigned_abs() <= grid.sector_radius_in_cells;

        in_x_range && in_y_range && in_z_range
    }

    pub fn sector_coordinates_valid(coordinates: IVec3, grid: &Grid) -> bool {
        let in_x_range = coordinates.x.unsigned_abs() <= grid.world_radius_in_sectors;
        let in_y_range = coordinates.y.unsigned_abs() <= grid.world_radius_in_sectors;
        let in_z_range = coordinates.z.unsigned_abs() <= grid.world_radius_in_sectors;

        in_x_range && in_y_range && in_z_range
    }

    pub fn position_valid(position: IVec3, grid: &Grid) -> bool {
        let in_x_range = position.x.unsigned_abs() <= grid.world_radius_in_cells;
        let in_y_range = position.y.unsigned_abs() <= grid.world_radius_in_cells;
        let in_z_range = position.z.unsigned_abs() <= grid.world_radius_in_cells;

        in_x_range && in_y_range && in_z_range
    }

    pub fn world_position_valid(world_position: Vec3, grid: &Grid) -> bool {
        let in_x_range = world_position.x.abs() <= grid.world_radius_in_meters;
        let in_y_range = world_position.y.abs() <= grid.world_radius_in_meters;
        let in_z_range = world_position.z.abs() <= grid.world_radius_in_meters;

        in_x_range && in_y_range && in_z_range
    }

    pub fn cell_id_to_cell_coordinates(id: cell::ID, grid: &Grid) -> IVec3 {
        let index = u32::from(id);
        let coordinates = Indexing::to_ivec3(index, grid.sector_radius_in_cells);

        coordinates
    }

    pub fn cell_coordinates_to_cell_id(coordinates: IVec3, grid: &Grid) -> cell::ID {
        let index = Indexing::from_ivec3(coordinates, grid.sector_radius_in_cells);

        cell::ID(index)
    }

    pub fn sector_id_to_sector_coordinates(id: sector::ID, grid: &Grid) -> IVec3 {
        let index = u32::from(id);
        let coordinates = Indexing::to_ivec3(index, grid.world_radius_in_sectors);

        coordinates
    }

    pub fn sector_coordinates_to_sector_id(coordinates: IVec3, grid: &Grid) -> sector::ID {
        let index = Indexing::from_ivec3(coordinates, grid.world_radius_in_sectors);

        sector::ID(index)
    }

    pub fn sector_coordinates_to_position(sector_coordinates: IVec3, grid: &Grid) -> IVec3 {
        let position = sector_coordinates * grid.sector_size_in_cells as i32;

        position
    }

    pub fn sector_id_to_position(sector_id: sector::ID, grid: &Grid) -> IVec3 {
        let coordinates = Grid::sector_id_to_sector_coordinates(sector_id, grid);

        Grid::sector_coordinates_to_position(coordinates, grid)
    }

    pub fn position_to_sector_coordinates(position: IVec3, grid: &Grid) -> IVec3 {
        let world_radius_in_cells = grid.world_radius_in_cells as i32;

        let position_indexable = position + IVec3::broadcast(world_radius_in_cells);

        let sector_size_in_cells = grid.sector_size_in_cells as i32;

        let sector_coordinates_indexable = position_indexable / sector_size_in_cells;

        let world_radius_in_sectors = grid.world_radius_in_sectors as i32;

        let sector_coordinates =
            sector_coordinates_indexable - IVec3::broadcast(world_radius_in_sectors);

        sector_coordinates
    }

    pub fn position_to_cell_coordinates(position: IVec3, grid: &Grid) -> IVec3 {
        let world_radius_in_cells = grid.world_radius_in_cells as i32;

        let position_indexable = position + IVec3::broadcast(world_radius_in_cells);

        let sector_size_in_cells = grid.sector_size_in_cells as i32;

        let cell_coordinates_indexable = IVec3::new(
            position_indexable.x % sector_size_in_cells,
            position_indexable.y % sector_size_in_cells,
            position_indexable.z % sector_size_in_cells,
        );

        let sector_radius_in_cells = grid.sector_radius_in_cells as i32;

        let cell_coordinates =
            cell_coordinates_indexable - IVec3::broadcast(sector_radius_in_cells);

        cell_coordinates
    }

    pub fn position_to_sector_id(position: IVec3, grid: &Grid) -> sector::ID {
        let sector_coordinates = Grid::position_to_sector_coordinates(position, grid);

        Grid::sector_coordinates_to_sector_id(sector_coordinates, grid)
    }

    pub fn position_to_cell_id(position: IVec3, grid: &Grid) -> cell::ID {
        let cell_coordinates = Grid::position_to_cell_coordinates(position, grid);

        Grid::cell_coordinates_to_cell_id(cell_coordinates, grid)
    }

    pub fn position_to_ids(position: IVec3, grid: &Grid) -> (sector::ID, cell::ID) {
        let sector_id = Grid::position_to_sector_id(position, grid);
        let cell_id = Grid::position_to_cell_id(position, grid);

        (sector_id, cell_id)
    }

    pub fn ids_to_position(sector_id: sector::ID, cell_id: cell::ID, grid: &Grid) -> IVec3 {
        let sector_coordinates = Grid::sector_id_to_sector_coordinates(sector_id, grid);
        let cell_coordinates = Grid::cell_id_to_cell_coordinates(cell_id, grid);

        grid.sector_size_in_cells as i32 * sector_coordinates + cell_coordinates
    }

    pub fn world_position_to_position(world_position: Vec3) -> IVec3 {
        let position = IVec3::new(
            (world_position.x + CELL_RADIUS).floor() as i32,
            (world_position.y + CELL_RADIUS).floor() as i32,
            (world_position.z + CELL_RADIUS).floor() as i32,
        );

        position
    }

    pub fn world_position_to_sector_id(world_position: Vec3, grid: &Grid) -> sector::ID {
        let position = Grid::world_position_to_position(world_position);

        Grid::position_to_sector_id(position, grid)
    }

    pub fn world_position_to_sector_coordinates(world_position: Vec3, grid: &Grid) -> IVec3 {
        let position = Grid::world_position_to_position(world_position);

        Grid::position_to_sector_coordinates(position, grid)
    }

    pub fn world_to_cell_id(grid: &Grid, world_position: Vec3) -> cell::ID {
        let position = Grid::world_position_to_position(world_position);

        Grid::position_to_cell_id(position, grid)
    }

    pub fn world_to_cell_coordinates(world_position: Vec3, grid: &Grid) -> IVec3 {
        let position = Grid::world_position_to_position(world_position);

        Grid::position_to_cell_coordinates(position, grid)
    }

    pub fn cells_overlapping(aabb: AABB, grid: &Grid) -> Vec<AABB> {
        let mut aabb_vec = Vec::new();

        let min = IVec3::new(
            aabb.min.x.round() as i32,
            aabb.min.y.round() as i32,
            aabb.min.z.round() as i32,
        );

        let max = IVec3::new(
            aabb.max.x.round() as i32,
            aabb.max.y.round() as i32,
            aabb.max.z.round() as i32,
        );

        let size = Vec3::broadcast(grid.cell_size_in_meters);

        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let cell_position = IVec3::new(x, y, z);
                    let cell_aabb = AABB::new(Vec3::from(cell_position), size);

                    if cell_aabb.overlaps(aabb) {
                        aabb_vec.push(cell_aabb);
                    }
                }
            }
        }

        aabb_vec
    }

    pub fn on_sector_boundary(position: IVec3, grid: &Grid) -> bool {
        if !Grid::position_valid(position, grid) {
            true
        } else {
            let cell_coordinates = Grid::position_to_cell_coordinates(position, grid);

            let sector_radius_in_cells = grid.sector_radius_in_cells as i32;

            cell_coordinates.x.abs() == sector_radius_in_cells
                || cell_coordinates.y.abs() == sector_radius_in_cells
                || cell_coordinates.z.abs() == sector_radius_in_cells
        }
    }

    pub fn on_world_radius(position: IVec3, grid: &Grid) -> bool {
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
