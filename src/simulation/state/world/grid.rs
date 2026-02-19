pub mod axis;
pub mod direction;
pub mod direction_set;
pub mod line;
pub mod quadrant;

pub use axis::Axis;
pub use direction::Direction;
pub use line::Line;
pub use quadrant::Quadrant;

use crate::{
    simulation::{
        constants::*,
        state::world::{cell::cell_index::CellIndex, sector::sector_index::SectorIndex},
    },
    utils::ldmath::{ivec3_ext, FloatBox, IntBox},
};
use ultraviolet::{IVec3, Vec3};

#[inline]
pub fn cell_index_vec() -> Vec<CellIndex> {
    (0usize..SECTOR_VOLUME_IN_CELLS)
        .map(CellIndex::new)
        .collect()
}

#[inline]
pub fn sector_index_vec() -> Vec<SectorIndex> {
    (0usize..WORLD_VOLUME_IN_SECTORS)
        .map(SectorIndex::new)
        .collect()
}

#[inline]
pub fn cell_index_is_valid(cell_index: CellIndex) -> bool {
    cell_index_vec().contains(&cell_index)
}

#[inline]
pub fn sector_index_is_valid(sector_index: SectorIndex) -> bool {
    sector_index_vec().contains(&sector_index)
}

#[inline]
pub fn cell_coordinate_is_valid(cell_coordinate: IVec3) -> bool {
    let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

    let in_x_range =
        cell_coordinate.x >= -sector_radius_in_cells && cell_coordinate.x <= sector_radius_in_cells;

    let in_y_range =
        cell_coordinate.y >= -sector_radius_in_cells && cell_coordinate.y <= sector_radius_in_cells;

    let in_z_range =
        cell_coordinate.z >= -sector_radius_in_cells && cell_coordinate.z <= sector_radius_in_cells;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn sector_coordinate_is_valid(sector_coordinate: IVec3) -> bool {
    let world_radius_in_sectors = WORLD_RADIUS_IN_SECTORS as i32;

    let in_x_range = sector_coordinate.x >= -world_radius_in_sectors
        && sector_coordinate.x <= world_radius_in_sectors;

    let in_y_range = sector_coordinate.y >= -world_radius_in_sectors
        && sector_coordinate.y <= world_radius_in_sectors;

    let in_z_range = sector_coordinate.z >= -world_radius_in_sectors
        && sector_coordinate.z <= world_radius_in_sectors;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn grid_position_is_valid(grid_position: IVec3) -> bool {
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
pub fn world_position_is_valid(world_position: Vec3) -> bool {
    let in_x_range =
        world_position.x >= -WORLD_RADIUS_IN_METERS && world_position.x <= WORLD_RADIUS_IN_METERS;

    let in_y_range =
        world_position.y >= -WORLD_RADIUS_IN_METERS && world_position.x <= WORLD_RADIUS_IN_METERS;

    let in_z_range =
        world_position.z >= -WORLD_RADIUS_IN_METERS && world_position.x <= WORLD_RADIUS_IN_METERS;

    in_x_range && in_y_range && in_z_range
}

#[inline]
pub fn cell_index_to_cell_coordinate(cell_index: CellIndex) -> IVec3 {
    let cell_coordinate =
        ivec3_ext::index_to_ivec3(CellIndex::as_index(&cell_index), SECTOR_RADIUS_IN_CELLS);

    cell_coordinate
}

#[inline]
pub fn cell_coordinate_to_cell_index(coordinate: IVec3) -> CellIndex {
    let index = ivec3_ext::ivec3_to_index(coordinate, SECTOR_RADIUS_IN_CELLS);

    let cell_index = CellIndex::new(index);

    cell_index
}

#[inline]
pub fn sector_index_to_sector_coordinate(sector_index: SectorIndex) -> IVec3 {
    let sector_coordinate = ivec3_ext::index_to_ivec3(
        SectorIndex::as_index(&sector_index),
        WORLD_RADIUS_IN_SECTORS,
    );

    sector_coordinate
}

#[inline]
pub fn sector_coordinate_to_sector_index(sector_coordinate: IVec3) -> SectorIndex {
    let index = ivec3_ext::ivec3_to_index(sector_coordinate, WORLD_RADIUS_IN_SECTORS);

    let sector_index = SectorIndex::new(index);

    sector_index
}

#[inline]
pub fn sector_coordinate_to_grid_position(sector_coordinate: IVec3) -> IVec3 {
    let grid_position = sector_coordinate * SECTOR_SIZE_IN_CELLS as i32;

    grid_position
}

#[inline]
pub fn sector_index_to_grid_position(sector_index: SectorIndex) -> IVec3 {
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
pub fn grid_position_to_sector_index(grid_position: IVec3) -> SectorIndex {
    let sector_coordinate = grid_position_to_sector_coordinate(grid_position);

    let sector_index = sector_coordinate_to_sector_index(sector_coordinate);

    sector_index
}

#[inline]
pub fn grid_position_to_cell_index(grid_position: IVec3) -> CellIndex {
    let cell_coordinate = grid_position_to_cell_coordinate(grid_position);

    let cell_index = cell_coordinate_to_cell_index(cell_coordinate);

    cell_index
}

#[inline]
pub fn grid_position_to_indices(grid_position: IVec3) -> (SectorIndex, CellIndex) {
    let sector_index = grid_position_to_sector_index(grid_position);
    let cell_index = grid_position_to_cell_index(grid_position);

    (sector_index, cell_index)
}

#[inline]
pub fn indices_to_grid_position(sector_index: SectorIndex, cell_index: CellIndex) -> IVec3 {
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
pub fn world_position_to_sector_index(world_position: Vec3) -> SectorIndex {
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
pub fn world_position_to_cell_index(world_position: Vec3) -> CellIndex {
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

#[inline]
pub fn get_cell_float_box(grid_position: IVec3) -> FloatBox {
    let world_position = grid_position_to_world_position(grid_position);
    let radius = Vec3::broadcast(CELL_RADIUS_IN_METERS);

    FloatBox::new(world_position, radius)
}

#[inline]
pub fn get_grid_int_box(grid_position: IVec3, size: IVec3) -> IntBox {
    let min = grid_position;
    let max = grid_position + size - IVec3::one();

    IntBox::new(min, max)
}

#[inline]
pub fn get_float_box_grid_overlap_vec(float_box: &FloatBox) -> Vec<IVec3> {
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

                if grid_position_is_valid(grid_position) {
                    let cell_float_box = get_cell_float_box(grid_position);

                    if FloatBox::overlap(float_box, &cell_float_box) {
                        grid_position_vec.push(grid_position);
                    }
                }
            }
        }
    }

    grid_position_vec
}

pub const FACE_OFFSET_ARRAY: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
];

pub const NEIGHBOR_OFFSET_ARRAY: [IVec3; 26] = [
    IVec3::new(-1, -1, -1),
    IVec3::new(0, -1, -1),
    IVec3::new(1, -1, -1),
    IVec3::new(-1, 0, -1),
    IVec3::new(0, 0, -1),
    IVec3::new(1, 0, -1),
    IVec3::new(-1, 1, -1),
    IVec3::new(0, 1, -1),
    IVec3::new(1, 1, -1),
    IVec3::new(-1, -1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(1, -1, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 1, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(1, 1, 0),
    IVec3::new(-1, -1, 1),
    IVec3::new(0, -1, 1),
    IVec3::new(1, -1, 1),
    IVec3::new(-1, 0, 1),
    IVec3::new(0, 0, 1),
    IVec3::new(1, 0, 1),
    IVec3::new(-1, 1, 1),
    IVec3::new(0, 1, 1),
    IVec3::new(1, 1, 1),
];

#[inline]
pub fn get_face_offset_vec() -> &'static [IVec3] {
    &FACE_OFFSET_ARRAY
}

#[inline]
pub fn get_neighbor_offset_vec() -> &'static [IVec3] {
    &NEIGHBOR_OFFSET_ARRAY
}

#[inline]
pub fn on_sector_boundary(grid_position: IVec3) -> bool {
    if grid_position_is_valid(grid_position) {
        let cell_coordinate = grid_position_to_cell_coordinate(grid_position);

        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

        cell_coordinate.x.abs() == sector_radius_in_cells
            || cell_coordinate.y.abs() == sector_radius_in_cells
            || cell_coordinate.z.abs() == sector_radius_in_cells
    } else {
        false
    }
}

#[inline]
pub fn on_world_boundary(grid_position: IVec3) -> bool {
    let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

    grid_position.x.abs() == world_radius_in_cells
        || grid_position.y.abs() == world_radius_in_cells
        || grid_position.z.abs() == world_radius_in_cells
}
