pub mod axis;
pub mod direction;
pub mod line;
pub mod quadrant;

pub use axis::Axis;
pub use direction::Direction;
pub use line::Line;
pub use quadrant::Quadrant;

use crate::{
    simulation::{constants::*, state::physics::box_collider::BoxCollider},
    utils::ldmath::indexing,
};
use ultraviolet::{IVec3, Vec3};

#[inline]
pub fn cell_id_vec() -> Vec<usize> {
    (0usize..SECTOR_VOLUME_IN_CELLS).collect()
}

#[inline]
pub fn sector_id_vec() -> Vec<usize> {
    (0usize..WORLD_VOLUME_IN_SECTORS).collect()
}

#[inline]
pub fn is_cell_id_valid(id: usize) -> bool {
    (0usize..SECTOR_VOLUME_IN_CELLS).contains(&id)
}

#[inline]
pub fn is_sector_id_valid(id: usize) -> bool {
    (0usize..WORLD_VOLUME_IN_SECTORS).contains(&id)
}

#[inline]
pub fn is_cell_coordinate_valid(coordinate: IVec3) -> bool {
    let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

    let in_x_range =
        coordinate.x >= -sector_radius_in_cells && coordinate.x <= sector_radius_in_cells;

    let in_y_range =
        coordinate.y >= -sector_radius_in_cells && coordinate.y <= sector_radius_in_cells;

    let in_z_range =
        coordinate.z >= -sector_radius_in_cells && coordinate.z <= sector_radius_in_cells;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn is_sector_coordinate_valid(coordinate: IVec3) -> bool {
    let world_radius_in_sectors = WORLD_RADIUS_IN_SECTORS as i32;

    let in_x_range =
        coordinate.x >= -world_radius_in_sectors && coordinate.x <= world_radius_in_sectors;

    let in_y_range =
        coordinate.y >= -world_radius_in_sectors && coordinate.y <= world_radius_in_sectors;

    let in_z_range =
        coordinate.z >= -world_radius_in_sectors && coordinate.z <= world_radius_in_sectors;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn is_grid_position_valid(grid_position: IVec3) -> bool {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

    let in_x_range =
        grid_position.x >= -world_radius_in_cells && grid_position.x <= world_radius_in_cells;

    let in_y_range =
        grid_position.y >= -world_radius_in_cells && grid_position.y <= world_radius_in_cells;

    let in_z_range =
        grid_position.z >= -world_radius_in_cells && grid_position.z <= world_radius_in_cells;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn is_world_position_valid(world_position: Vec3) -> bool {
    let in_x_range =
        world_position.x >= -WORLD_RADIUS_IN_METERS && world_position.x <= WORLD_RADIUS_IN_METERS;

    let in_y_range =
        world_position.y >= -WORLD_RADIUS_IN_METERS && world_position.x <= WORLD_RADIUS_IN_METERS;

    let in_z_range =
        world_position.z >= -WORLD_RADIUS_IN_METERS && world_position.x <= WORLD_RADIUS_IN_METERS;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn cell_id_to_cell_coordinate(id: usize) -> IVec3 {
    let cell_coordinate = indexing::to_ivec3(id, SECTOR_RADIUS_IN_CELLS);

    cell_coordinate
}

#[inline]
pub fn cell_coordinate_to_cell_id(coordinate: IVec3) -> usize {
    let cell_id = indexing::from_ivec3(coordinate, SECTOR_RADIUS_IN_CELLS);

    cell_id
}

#[inline]
pub fn sector_id_to_sector_coordinate(id: usize) -> IVec3 {
    let sector_coordinate = indexing::to_ivec3(id, WORLD_RADIUS_IN_SECTORS);

    sector_coordinate
}

#[inline]
pub fn sector_coordinate_to_sector_id(coordinate: IVec3) -> usize {
    let sector_id = indexing::from_ivec3(coordinate, WORLD_RADIUS_IN_SECTORS);

    sector_id
}

#[inline]
pub fn sector_coordinate_to_grid_position(sector_coordinate: IVec3) -> IVec3 {
    let grid_position = sector_coordinate * SECTOR_SIZE_IN_CELLS as i32;

    grid_position
}

#[inline]
pub fn sector_id_to_grid_position(sector_id: usize) -> IVec3 {
    let sector_coordinate = sector_id_to_sector_coordinate(sector_id);

    let grid_position = sector_coordinate_to_grid_position(sector_coordinate);

    grid_position
}

#[inline]
pub fn grid_position_to_world_position(grid_position: IVec3) -> Vec3 {
    let world_position = Vec3::new(
        grid_position.x as f32,
        grid_position.y as f32,
        grid_position.z as f32,
    );

    world_position
}

#[inline]
pub fn grid_position_to_sector_coordinate(grid_position: IVec3) -> IVec3 {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;
    let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;
    let world_radius_in_sectors = WORLD_RADIUS_IN_SECTORS as i32;

    let grid_position_indexable = grid_position + IVec3::broadcast(world_radius_in_cells);
    let sector_coordinate_indexable = grid_position_indexable / sector_size_in_cells;

    let sector_coordinate = sector_coordinate_indexable - IVec3::broadcast(world_radius_in_sectors);

    sector_coordinate
}

#[inline]
pub fn grid_position_to_cell_coordinate(grid_position: IVec3) -> IVec3 {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;
    let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;
    let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

    let grid_position_indexable = grid_position + IVec3::broadcast(world_radius_in_cells);

    let cell_coordinate_indexable = IVec3::new(
        grid_position_indexable.x % sector_size_in_cells,
        grid_position_indexable.y % sector_size_in_cells,
        grid_position_indexable.z % sector_size_in_cells,
    );

    let cell_coordinate = cell_coordinate_indexable - IVec3::broadcast(sector_radius_in_cells);

    cell_coordinate
}

#[inline]
pub fn grid_position_to_sector_id(grid_position: IVec3) -> usize {
    let sector_coordinate = grid_position_to_sector_coordinate(grid_position);

    let sector_id = sector_coordinate_to_sector_id(sector_coordinate);

    sector_id
}

#[inline]
pub fn grid_position_to_cell_id(grid_position: IVec3) -> usize {
    let cell_coordinate = grid_position_to_cell_coordinate(grid_position);

    let cell_id = cell_coordinate_to_cell_id(cell_coordinate);

    cell_id
}

#[inline]
pub fn grid_position_to_ids(grid_position: IVec3) -> (usize, usize) {
    let sector_id = grid_position_to_sector_id(grid_position);
    let cell_id = grid_position_to_cell_id(grid_position);

    (sector_id, cell_id)
}

#[inline]
pub fn ids_to_grid_position(sector_id: usize, cell_id: usize) -> IVec3 {
    let sector_coordinate = sector_id_to_sector_coordinate(sector_id);
    let cell_coordinate = cell_id_to_cell_coordinate(cell_id);

    let grid_position = SECTOR_SIZE_IN_CELLS as i32 * sector_coordinate + cell_coordinate;

    grid_position
}

#[inline]
pub fn world_position_to_grid_position(world_position: Vec3) -> IVec3 {
    let grid_position = IVec3::new(
        (world_position.x + CELL_RADIUS_IN_METERS).floor() as i32,
        (world_position.y + CELL_RADIUS_IN_METERS).floor() as i32,
        (world_position.z + CELL_RADIUS_IN_METERS).floor() as i32,
    );

    grid_position
}

#[inline]
pub fn world_position_to_sector_id(world_position: Vec3) -> usize {
    let grid_position = world_position_to_grid_position(world_position);

    let sector_id = grid_position_to_sector_id(grid_position);

    sector_id
}

#[inline]
pub fn world_position_to_sector_coordinate(world_position: Vec3) -> IVec3 {
    let grid_position = world_position_to_grid_position(world_position);

    let sector_coordinate = grid_position_to_sector_coordinate(grid_position);

    sector_coordinate
}

#[inline]
pub fn world_position_to_cell_id(world_position: Vec3) -> usize {
    let grid_position = world_position_to_grid_position(world_position);

    let cell_id = grid_position_to_cell_id(grid_position);

    cell_id
}

#[inline]
pub fn world_position_to_cell_coordinate(world_position: Vec3) -> IVec3 {
    let grid_position = world_position_to_grid_position(world_position);

    let cell_coordinate = grid_position_to_cell_coordinate(grid_position);

    cell_coordinate
}

#[inline]
pub fn cells_overlapping(box_collider: BoxCollider) -> Vec<BoxCollider> {
    let mut aabb_vec = Vec::new();

    let min = IVec3::new(
        box_collider.min.x.round() as i32,
        box_collider.min.y.round() as i32,
        box_collider.min.z.round() as i32,
    );

    let max = IVec3::new(
        box_collider.max.x.round() as i32,
        box_collider.max.y.round() as i32,
        box_collider.max.z.round() as i32,
    );

    let size = Vec3::broadcast(CELL_SIZE_IN_METERS);

    for z in min.z..=max.z {
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let cell_position = IVec3::new(x, y, z);
                let cell_aabb = BoxCollider::new(Vec3::from(cell_position), size);

                if cell_aabb.overlaps(box_collider) {
                    aabb_vec.push(cell_aabb);
                }
            }
        }
    }

    aabb_vec
}

#[inline]
pub fn on_sector_boundary(grid_position: IVec3) -> bool {
    if !is_grid_position_valid(grid_position) {
        true
    } else {
        let cell_coordinate = grid_position_to_cell_coordinate(grid_position);

        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

        cell_coordinate.x.abs() == sector_radius_in_cells
            || cell_coordinate.y.abs() == sector_radius_in_cells
            || cell_coordinate.z.abs() == sector_radius_in_cells
    }
}

#[inline]
pub fn on_world_radius(grid_position: IVec3) -> bool {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

    grid_position.x.abs() == world_radius_in_cells
        || grid_position.y.abs() == world_radius_in_cells
        || grid_position.z.abs() == world_radius_in_cells
}

#[inline]
pub fn offsets_in(radius: i32) -> impl Iterator<Item = IVec3> {
    (-radius..=radius).flat_map(move |x| {
        (-radius..=radius).flat_map(move |y| (-radius..=radius).map(move |z| IVec3::new(x, y, z)))
    })
}

#[inline]
pub fn get_bounds(grid_position: IVec3, size: IVec3) -> (IVec3, IVec3) {
    let min = grid_position;
    let max = grid_position + size - IVec3::one();

    (min, max)
}
