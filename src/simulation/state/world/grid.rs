pub mod axis;
pub mod cell_sample;
pub mod direction;
pub mod world_ray_iterator;

pub use cell_sample::CellSample;
pub use direction::Direction;
pub use world_ray_iterator::WorldRayIterator;

use crate::{
    simulation::{
        constants::*,
        state::{
            physics::aabb::AABB,
            world::{cell, sector},
        },
    },
    utils::ld_math::indexing,
};
use ultraviolet::{IVec3, Vec3};

#[inline]
pub fn cell_ids() -> Vec<cell::ID> {
    (0usize..SECTOR_VOLUME_IN_CELLS).map(cell::ID).collect()
}

#[inline]
pub fn sector_ids() -> Vec<sector::ID> {
    (0usize..WORLD_VOLUME_IN_SECTORS).map(sector::ID).collect()
}

#[inline]
pub fn cell_id_valid(id: cell::ID) -> bool {
    (0usize..SECTOR_VOLUME_IN_CELLS).contains(&cell::ID::to_usize(&id))
}

#[inline]
pub fn sector_id_valid(id: sector::ID) -> bool {
    (0usize..WORLD_VOLUME_IN_SECTORS).contains(&sector::ID::to_usize(&id))
}

#[inline]
pub fn cell_coordinates_valid(coordinates: IVec3) -> bool {
    let in_x_range = coordinates.x.unsigned_abs() <= SECTOR_RADIUS_IN_CELLS as u32;
    let in_y_range = coordinates.y.unsigned_abs() <= SECTOR_RADIUS_IN_CELLS as u32;
    let in_z_range = coordinates.z.unsigned_abs() <= SECTOR_RADIUS_IN_CELLS as u32;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn sector_coordinates_valid(coordinates: IVec3) -> bool {
    let in_x_range = coordinates.x.unsigned_abs() <= WORLD_RADIUS_IN_SECTORS as u32;
    let in_y_range = coordinates.y.unsigned_abs() <= WORLD_RADIUS_IN_SECTORS as u32;
    let in_z_range = coordinates.z.unsigned_abs() <= WORLD_RADIUS_IN_SECTORS as u32;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn position_valid(position: IVec3) -> bool {
    let in_x_range = position.x.unsigned_abs() <= WORLD_RADIUS_IN_CELLS as u32;
    let in_y_range = position.y.unsigned_abs() <= WORLD_RADIUS_IN_CELLS as u32;
    let in_z_range = position.z.unsigned_abs() <= WORLD_RADIUS_IN_CELLS as u32;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn world_position_valid(world_position: Vec3) -> bool {
    let in_x_range = world_position.x.abs() <= WORLD_RADIUS_IN_METERS;
    let in_y_range = world_position.y.abs() <= WORLD_RADIUS_IN_METERS;
    let in_z_range = world_position.z.abs() <= WORLD_RADIUS_IN_METERS;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn cell_id_to_cell_coordinates(id: cell::ID) -> IVec3 {
    let index = id.to_usize();
    let coordinates = indexing::to_ivec3(index, SECTOR_RADIUS_IN_CELLS);

    coordinates
}

#[inline]
pub fn cell_coordinates_to_cell_id(coordinates: IVec3) -> cell::ID {
    let index = indexing::from_ivec3(coordinates, SECTOR_RADIUS_IN_CELLS);

    cell::ID(index)
}

#[inline]
pub fn sector_id_to_sector_coordinates(id: sector::ID) -> IVec3 {
    let index = id.to_usize();
    let coordinates = indexing::to_ivec3(index, WORLD_RADIUS_IN_SECTORS);

    coordinates
}

#[inline]
pub fn sector_coordinates_to_sector_id(coordinates: IVec3) -> sector::ID {
    let index = indexing::from_ivec3(coordinates, WORLD_RADIUS_IN_SECTORS);

    sector::ID(index)
}

#[inline]
pub fn sector_coordinates_to_position(sector_coordinates: IVec3) -> IVec3 {
    let position = sector_coordinates * SECTOR_SIZE_IN_CELLS as i32;

    position
}

#[inline]
pub fn sector_id_to_position(sector_id: sector::ID) -> IVec3 {
    let coordinates = sector_id_to_sector_coordinates(sector_id);

    sector_coordinates_to_position(coordinates)
}

#[inline]
pub fn position_to_sector_coordinates(position: IVec3) -> IVec3 {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

    let position_indexable = position + IVec3::broadcast(world_radius_in_cells);

    let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

    let sector_coordinates_indexable = position_indexable / sector_size_in_cells;

    let world_radius_in_sectors = WORLD_RADIUS_IN_SECTORS as i32;

    let sector_coordinates =
        sector_coordinates_indexable - IVec3::broadcast(world_radius_in_sectors);

    sector_coordinates
}

#[inline]
pub fn position_to_cell_coordinates(position: IVec3) -> IVec3 {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

    let position_indexable = position + IVec3::broadcast(world_radius_in_cells);

    let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

    let cell_coordinates_indexable = IVec3::new(
        position_indexable.x % sector_size_in_cells,
        position_indexable.y % sector_size_in_cells,
        position_indexable.z % sector_size_in_cells,
    );

    let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

    let cell_coordinates = cell_coordinates_indexable - IVec3::broadcast(sector_radius_in_cells);

    cell_coordinates
}

#[inline]
pub fn position_to_sector_id(position: IVec3) -> sector::ID {
    let sector_coordinates = position_to_sector_coordinates(position);

    sector_coordinates_to_sector_id(sector_coordinates)
}

#[inline]
pub fn position_to_cell_id(position: IVec3) -> cell::ID {
    let cell_coordinates = position_to_cell_coordinates(position);

    cell_coordinates_to_cell_id(cell_coordinates)
}

#[inline]
pub fn position_to_ids(position: IVec3) -> (sector::ID, cell::ID) {
    let sector_id = position_to_sector_id(position);
    let cell_id = position_to_cell_id(position);

    (sector_id, cell_id)
}

#[inline]
pub fn ids_to_position(sector_id: sector::ID, cell_id: cell::ID) -> IVec3 {
    let sector_coordinates = sector_id_to_sector_coordinates(sector_id);
    let cell_coordinates = cell_id_to_cell_coordinates(cell_id);

    SECTOR_SIZE_IN_CELLS as i32 * sector_coordinates + cell_coordinates
}

#[inline]
pub fn world_position_to_position(world_position: Vec3) -> IVec3 {
    let position = IVec3::new(
        (world_position.x + CELL_RADIUS_IN_METERS).floor() as i32,
        (world_position.y + CELL_RADIUS_IN_METERS).floor() as i32,
        (world_position.z + CELL_RADIUS_IN_METERS).floor() as i32,
    );

    position
}

#[inline]
pub fn world_position_to_sector_id(world_position: Vec3) -> sector::ID {
    let position = world_position_to_position(world_position);

    position_to_sector_id(position)
}

#[inline]
pub fn world_position_to_sector_coordinates(world_position: Vec3) -> IVec3 {
    let position = world_position_to_position(world_position);

    position_to_sector_coordinates(position)
}

#[inline]
pub fn world_to_cell_id(world_position: Vec3) -> cell::ID {
    let position = world_position_to_position(world_position);

    position_to_cell_id(position)
}

#[inline]
pub fn world_to_cell_coordinates(world_position: Vec3) -> IVec3 {
    let position = world_position_to_position(world_position);

    position_to_cell_coordinates(position)
}

#[inline]
pub fn cells_overlapping(aabb: AABB) -> Vec<AABB> {
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

    let size = Vec3::broadcast(CELL_SIZE_IN_METERS);

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

#[inline]
pub fn on_sector_boundary(position: IVec3) -> bool {
    if !position_valid(position) {
        true
    } else {
        let cell_coordinates = position_to_cell_coordinates(position);

        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

        cell_coordinates.x.abs() == sector_radius_in_cells
            || cell_coordinates.y.abs() == sector_radius_in_cells
            || cell_coordinates.z.abs() == sector_radius_in_cells
    }
}

#[inline]
pub fn on_world_radius(position: IVec3) -> bool {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

    position.x.abs() == world_radius_in_cells
        || position.y.abs() == world_radius_in_cells
        || position.z.abs() == world_radius_in_cells
}

#[inline]
pub fn offsets_in(radius: i32) -> impl Iterator<Item = IVec3> {
    (-radius..=radius).flat_map(move |x| {
        (-radius..=radius).flat_map(move |y| (-radius..=radius).map(move |z| IVec3::new(x, y, z)))
    })
}
