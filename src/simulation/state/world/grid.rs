pub mod axis;
pub mod direction;
pub mod line;
pub mod quadrant;

pub use axis::Axis;
pub use direction::Direction;
pub use line::Line;
pub use quadrant::Quadrant;

use crate::{
    simulation::{
        constants::*,
        state::world::{cell::Cell, grid},
    },
    utils::ldmath::{ivec3_ext, FloatBox, IntBox},
};
use ultraviolet::{IVec3, Vec3};

#[inline]
pub fn cell_index_vec() -> Vec<usize> {
    (0usize..SECTOR_VOLUME_IN_CELLS).collect()
}

#[inline]
pub fn sector_index_vec() -> Vec<usize> {
    (0usize..WORLD_VOLUME_IN_SECTORS).collect()
}

#[inline]
pub fn is_cell_index_valid(id: usize) -> bool {
    (0usize..SECTOR_VOLUME_IN_CELLS).contains(&id)
}

#[inline]
pub fn is_sector_index_valid(id: usize) -> bool {
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
pub fn cell_index_to_cell_coordinate(id: usize) -> IVec3 {
    let cell_coordinate = ivec3_ext::index_to_ivec3(id, SECTOR_RADIUS_IN_CELLS);

    cell_coordinate
}

#[inline]
pub fn cell_coordinate_to_cell_index(coordinate: IVec3) -> usize {
    let cell_index = ivec3_ext::ivec3_to_index(coordinate, SECTOR_RADIUS_IN_CELLS);

    cell_index
}

#[inline]
pub fn sector_index_to_sector_coordinate(sector_index: usize) -> IVec3 {
    let sector_coordinate = ivec3_ext::index_to_ivec3(sector_index, WORLD_RADIUS_IN_SECTORS);

    sector_coordinate
}

#[inline]
pub fn sector_coordinate_to_sector_index(sector_coordinate: IVec3) -> usize {
    let sector_index = ivec3_ext::ivec3_to_index(sector_coordinate, WORLD_RADIUS_IN_SECTORS);

    sector_index
}

#[inline]
pub fn sector_coordinate_to_grid_position(sector_coordinate: IVec3) -> IVec3 {
    let grid_position = sector_coordinate * SECTOR_SIZE_IN_CELLS as i32;

    grid_position
}

#[inline]
pub fn sector_index_to_grid_position(sector_index: usize) -> IVec3 {
    let sector_coordinate = sector_index_to_sector_coordinate(sector_index);

    let grid_position = sector_coordinate_to_grid_position(sector_coordinate);

    grid_position
}

#[inline]
pub fn grid_position_to_world_position(grid_position: IVec3) -> Vec3 {
    let world_position = Vec3::from(grid_position);

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
pub fn grid_position_to_sector_index(grid_position: IVec3) -> usize {
    let sector_coordinate = grid_position_to_sector_coordinate(grid_position);

    let sector_index = sector_coordinate_to_sector_index(sector_coordinate);

    sector_index
}

#[inline]
pub fn grid_position_to_cell_index(grid_position: IVec3) -> usize {
    let cell_coordinate = grid_position_to_cell_coordinate(grid_position);

    let cell_index = cell_coordinate_to_cell_index(cell_coordinate);

    cell_index
}

#[inline]
pub fn grid_position_to_ids(grid_position: IVec3) -> (usize, usize) {
    let sector_index = grid_position_to_sector_index(grid_position);
    let cell_index = grid_position_to_cell_index(grid_position);

    (sector_index, cell_index)
}

#[inline]
pub fn ids_to_grid_position(sector_index: usize, cell_index: usize) -> IVec3 {
    let sector_coordinate = sector_index_to_sector_coordinate(sector_index);
    let cell_coordinate = cell_index_to_cell_coordinate(cell_index);

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
pub fn world_position_to_sector_index(world_position: Vec3) -> usize {
    let grid_position = world_position_to_grid_position(world_position);

    let sector_index = grid_position_to_sector_index(grid_position);

    sector_index
}

#[inline]
pub fn world_position_to_sector_coordinate(world_position: Vec3) -> IVec3 {
    let grid_position = world_position_to_grid_position(world_position);

    let sector_coordinate = grid_position_to_sector_coordinate(grid_position);

    sector_coordinate
}

#[inline]
pub fn world_position_to_cell_index(world_position: Vec3) -> usize {
    let grid_position = world_position_to_grid_position(world_position);

    let cell_index = grid_position_to_cell_index(grid_position);

    cell_index
}

#[inline]
pub fn world_position_to_cell_coordinate(world_position: Vec3) -> IVec3 {
    let grid_position = world_position_to_grid_position(world_position);

    let cell_coordinate = grid_position_to_cell_coordinate(grid_position);

    cell_coordinate
}

pub fn get_grid_int_box(grid_position: IVec3, size: IVec3) -> IntBox {
    IntBox::new(grid_position, grid_position + size - IVec3::one())
}

#[inline]
pub fn get_grid_overlap_vec(float_box: &FloatBox) -> Vec<IVec3> {
    let mut grid_position_vec = Vec::new();

    let min = FloatBox::get_min(float_box);
    let max = FloatBox::get_max(float_box);

    let grid_int_box = IntBox::new(
        IVec3::new(
            (min.x - CELL_RADIUS_IN_METERS).ceil() as i32,
            (min.y - CELL_RADIUS_IN_METERS).ceil() as i32,
            (min.z - CELL_RADIUS_IN_METERS).ceil() as i32,
        ),
        IVec3::new(
            (max.x + CELL_RADIUS_IN_METERS).floor() as i32,
            (max.y + CELL_RADIUS_IN_METERS).floor() as i32,
            (max.z + CELL_RADIUS_IN_METERS).floor() as i32,
        ),
    );

    for z in grid_int_box.min.z..=grid_int_box.max.z {
        for y in grid_int_box.min.y..=grid_int_box.max.y {
            for x in grid_int_box.min.x..=grid_int_box.max.x {
                let grid_position = IVec3::new(x, y, z);

                if grid::is_grid_position_valid(grid_position) {
                    let cell_float_box = Cell::get_float_box(grid_position);

                    if FloatBox::overlap(float_box, &cell_float_box) {
                        grid_position_vec.push(grid_position);
                    }
                }
            }
        }
    }

    grid_position_vec
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
