//! The simulated environment

pub mod area;
pub mod block;
pub mod cell;
pub mod grid;
pub mod object;
pub mod sector;
pub mod tower;

pub use area::Area;
pub use block::Block;
pub use cell::Cell;
pub use object::Object;
pub use sector::Sector;

use crate::{
    simulation::{
        constants::*,
        state::{
            population::nation,
            world::{grid::Direction, tower::Tower},
            Time,
        },
        utils::IDGenerator,
    },
    utils::ldmath::rand_chacha_ext::{gen_bool, gen_range_i32},
};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct World {
    pub active: bool,
    pub rng: ChaCha8Rng,
    pub time: Time,
    pub sector_vec: Vec<Sector>,
    pub tower: Tower,
    pub area_id_generator: IDGenerator,
    pub object_id_generator: IDGenerator,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let time = Time::new();
        let sector_vec = Self::setup_sector_vec();
        let tower = Tower::new();
        let area_id_generator = IDGenerator::new();
        let object_id_generator = IDGenerator::new();

        Self {
            active,
            rng,
            time,
            sector_vec,
            tower,
            area_id_generator,
            object_id_generator,
        }
    }

    pub fn tick(world: &mut Self) {
        let _ = tracing::info_span!("world_tick").entered();

        Time::tick(&mut world.time);
    }

    pub fn reset(world: &mut Self) {
        world.sector_vec = Self::setup_sector_vec();

        Tower::reset(&mut world.tower);
    }

    pub fn get_flag(
        nation_kind: nation::Kind,
        home_position_map: HashMap<nation::Kind, IVec3>,
    ) -> Option<IVec3> {
        home_position_map.get(&nation_kind).cloned()
    }

    fn setup_sector_vec() -> Vec<Sector> {
        grid::sector_id_vec()
            .into_iter()
            .map(|sector_id| {
                let grid_position = grid::sector_id_to_grid_position(sector_id);
                let version = 0;

                let cell_vec = Self::setup_cell_vec(sector_id);

                Sector {
                    sector_id,
                    version,
                    grid_position,
                    cell_vec,
                }
            })
            .collect()
    }

    pub fn set_object(
        grid_position: IVec3,
        object_kind: object::Kind,
        direction: grid::Direction,
        world: &mut Self,
    ) {
        if !grid::is_grid_position_valid(grid_position) {
            return;
        }

        let cell = World::get_cell_at_mut(grid_position, &mut world.sector_vec);

        let mut object = Object::new(object_kind);
        object.direction = direction;

        cell.object = Some(object);
    }

    pub fn set_object_cube(
        min: IVec3,
        max: IVec3,
        direction: Direction,
        object_kind: object::Kind,
        world: &mut Self,
    ) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let grid_position = IVec3::new(x, y, z);

                    Self::set_object(grid_position, object_kind, direction, world);
                }
            }
        }
    }

    fn setup_cell_vec(sector_id: usize) -> Vec<Cell> {
        grid::cell_id_vec()
            .into_iter()
            .map(|cell_id| {
                let grid_position = grid::ids_to_grid_position(sector_id, cell_id);

                Cell {
                    cell_id,
                    sector_id,
                    grid_position,
                    block: None,
                    object: None,
                }
            })
            .collect()
    }

    pub fn get_sector<'a>(sector_id: usize, sector_vec_slice: &'a [Sector]) -> &'a sector::Sector {
        let sector = &sector_vec_slice[sector_id];

        sector
    }

    pub fn get_sector_mut<'a>(
        sector_id: usize,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut sector::Sector {
        let sector = &mut sector_vec_slice[sector_id];

        sector
    }

    pub fn get_sector_at<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a [Sector],
    ) -> &'a sector::Sector {
        let sector_id = grid::grid_position_to_sector_id(grid_position);

        let sector = &sector_vec_slice[sector_id];

        sector
    }

    pub fn get_sector_at_mut<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut sector::Sector {
        let sector_id = grid::grid_position_to_sector_id(grid_position);

        let sector = &mut sector_vec_slice[sector_id];

        sector
    }

    pub fn get_cell(sector_id: usize, cell_id: usize, sector_vec_slice: &[Sector]) -> &Cell {
        let sector = &sector_vec_slice[sector_id];
        let cell = &sector.cell_vec[cell_id];

        cell
    }

    pub fn get_cell_mut(
        sector_id: usize,
        cell_id: usize,
        sector_vec_slice: &mut [Sector],
    ) -> &mut Cell {
        let sector = &mut sector_vec_slice[sector_id];

        let cell = &mut sector.cell_vec[cell_id];

        cell
    }

    pub fn get_cell_at<'a>(grid_position: IVec3, sector_vec_slice: &'a [Sector]) -> &'a Cell {
        let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

        let cell = Self::get_cell(sector_id, cell_id, sector_vec_slice);

        cell
    }

    pub fn get_cell_at_mut<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut Cell {
        let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

        let cell = Self::get_cell_mut(sector_id, cell_id, sector_vec_slice);

        cell
    }

    pub fn get_block<'a>(
        sector_id: usize,
        cell_id: usize,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a Block> {
        let cell = World::get_cell(sector_id, cell_id, sector_vec_slice);

        cell.block.as_ref()
    }

    pub fn get_block_at<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a Block> {
        let cell = World::get_cell_at(grid_position, sector_vec_slice);

        cell.block.as_ref()
    }

    pub fn get_block_mut<'a>(
        sector_id: usize,
        cell_id: usize,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut Block> {
        let cell = World::get_cell_mut(sector_id, cell_id, sector_vec_slice);

        cell.block.as_mut()
    }

    pub fn get_block_at_mut<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut Block> {
        let cell = World::get_cell_at_mut(grid_position, sector_vec_slice);

        cell.block.as_mut()
    }

    pub fn get_object<'a>(
        sector_id: usize,
        cell_id: usize,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a Object> {
        let cell = World::get_cell(sector_id, cell_id, sector_vec_slice);

        cell.object.as_ref()
    }

    pub fn get_object_at<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a Object> {
        let cell = World::get_cell_at(grid_position, sector_vec_slice);

        cell.object.as_ref()
    }

    pub fn get_object_mut<'a>(
        sector_id: usize,
        cell_id: usize,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut Object> {
        let cell = World::get_cell_mut(sector_id, cell_id, sector_vec_slice);

        cell.object.as_mut()
    }

    pub fn get_object_at_mut<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut Object> {
        let cell = World::get_cell_at_mut(grid_position, sector_vec_slice);

        cell.object.as_mut()
    }

    pub fn is_block_solid_at(grid_position: IVec3, world: &World) -> bool {
        if grid::is_grid_position_valid(grid_position) {
            Self::get_block_at(grid_position, &world.sector_vec).is_some_and(|block| block.solid)
        } else {
            true
        }
    }

    pub fn get_clearance(grid_position: IVec3, world: &Self) -> u32 {
        let is_ground_solid = Self::is_block_solid_at(grid_position + -1 * IVec3::unit_z(), world);

        let mut clearance = 0;

        if is_ground_solid {
            for level in 0..MAXIMUM_CLEARANCE {
                let level_grid_position = grid_position + IVec3::new(0, 0, level as i32);

                if Self::is_block_solid_at(level_grid_position, world) {
                    clearance += 1;
                } else {
                    break;
                }
            }
        }

        clearance
    }

    pub fn set_block(grid_position: IVec3, block_kind: block::Kind, world: &mut Self) {
        if grid::is_grid_position_valid(grid_position) {
            let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

            let cell = World::get_cell_mut(sector_id, cell_id, &mut world.sector_vec);

            let block = Block::new(block_kind);

            cell.block = Some(block);

            let sector = Self::get_sector_mut(sector_id, &mut world.sector_vec);
            sector.version += 1;
        }
    }

    pub fn remove_block(grid_position: IVec3, world: &mut Self) {
        if grid::is_grid_position_valid(grid_position) {
            let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

            let cell = World::get_cell_mut(sector_id, cell_id, &mut world.sector_vec);

            cell.block = None;

            let sector = Self::get_sector_mut(sector_id, &mut world.sector_vec);
            sector.version += 1;
        }
    }

    pub fn remove_block_cube(min: IVec3, max: IVec3, world: &mut Self) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let grid_position = IVec3::new(x, y, z);

                    Self::remove_block(grid_position, world);
                }
            }
        }
    }

    pub fn remove_object(grid_position: IVec3, world: &mut Self) {
        if grid::is_grid_position_valid(grid_position) {
            let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

            let cell = World::get_cell_mut(sector_id, cell_id, &mut world.sector_vec);

            cell.object = None;

            let sector = Self::get_sector_mut(sector_id, &mut world.sector_vec);
            sector.version += 1;
        }
    }

    pub fn set_block_wireframe(min: IVec3, max: IVec3, block_kind: block::Kind, world: &mut Self) {
        for x in min.x..=max.x {
            Self::set_block(IVec3::new(x, min.y, min.z), block_kind, world);
            Self::set_block(IVec3::new(x, max.y, min.z), block_kind, world);
            Self::set_block(IVec3::new(x, min.y, max.z), block_kind, world);
            Self::set_block(IVec3::new(x, max.y, max.z), block_kind, world);
        }

        for y in min.y..=max.y {
            Self::set_block(IVec3::new(min.x, y, min.z), block_kind, world);
            Self::set_block(IVec3::new(max.x, y, min.z), block_kind, world);
            Self::set_block(IVec3::new(min.x, y, max.z), block_kind, world);
            Self::set_block(IVec3::new(max.x, y, max.z), block_kind, world);
        }

        for z in min.z..=max.z {
            Self::set_block(IVec3::new(min.x, min.y, z), block_kind, world);
            Self::set_block(IVec3::new(min.x, max.y, z), block_kind, world);
            Self::set_block(IVec3::new(max.x, min.y, z), block_kind, world);
            Self::set_block(IVec3::new(max.x, max.y, z), block_kind, world);
        }
    }

    pub fn set_block_box(min: IVec3, max: IVec3, block_kind: block::Kind, world: &mut Self) {
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

                    let grid_position = IVec3::new(x, y, z);

                    if on_boundary {
                        Self::set_block(grid_position, block_kind, world);
                    } else {
                        Self::remove_block(grid_position, world);
                    }
                }
            }
        }
    }

    pub fn set_block_shell(min: IVec3, max: IVec3, block_kind: block::Kind, world: &mut Self) {
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

                    let grid_position = IVec3::new(x, y, z);

                    if on_boundary {
                        Self::set_block(grid_position, block_kind, world);
                    }
                }
            }
        }
    }

    pub fn set_block_cube(min: IVec3, max: IVec3, block_kind: block::Kind, world: &mut Self) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_block(position, block_kind, world);
                }
            }
        }
    }

    pub fn raycast_to_block(
        origin: Vec3,
        direction: Vec3,
        range: f32,
        world: &Self,
    ) -> Option<(IVec3, IVec3)> {
        let direction = direction.normalized();

        let mut cell_grid_position = grid::world_position_to_grid_position(origin);

        let step = IVec3::new(
            if direction.x > 0.0 { 1 } else { -1 },
            if direction.y > 0.0 { 1 } else { -1 },
            if direction.z > 0.0 { 1 } else { -1 },
        );

        let t_max = Vec3 {
            x: if direction.x != 0.0 {
                let boundary = if direction.x > 0.0 {
                    cell_grid_position.x as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_grid_position.x as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.x) / direction.x
            } else {
                f32::INFINITY
            },
            y: if direction.y != 0.0 {
                let boundary = if direction.y > 0.0 {
                    cell_grid_position.y as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_grid_position.y as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.y) / direction.y
            } else {
                f32::INFINITY
            },
            z: if direction.z != 0.0 {
                let boundary = if direction.z > 0.0 {
                    cell_grid_position.z as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_grid_position.z as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.z) / direction.z
            } else {
                f32::INFINITY
            },
        };

        let t_delta = Vec3::new(
            if direction.x != 0.0 {
                (1.0 / direction.x).abs()
            } else {
                f32::INFINITY
            },
            if direction.y != 0.0 {
                (1.0 / direction.y).abs()
            } else {
                f32::INFINITY
            },
            if direction.z != 0.0 {
                (1.0 / direction.z).abs()
            } else {
                f32::INFINITY
            },
        );

        let mut t_max = t_max;
        let mut distance_traveled = 0.0;

        while distance_traveled < range {
            let hit_normal;

            if t_max.x < t_max.y && t_max.x < t_max.z {
                cell_grid_position.x += step.x;
                distance_traveled = t_max.x;
                t_max.x += t_delta.x;

                if distance_traveled > range {
                    return None;
                }

                hit_normal = -step.x * IVec3::unit_x();
            } else if t_max.y < t_max.z {
                cell_grid_position.y += step.y;
                distance_traveled = t_max.y;
                t_max.y += t_delta.y;

                if distance_traveled > range {
                    return None;
                }

                hit_normal = -step.y * IVec3::unit_y();
            } else {
                cell_grid_position.z += step.z;
                distance_traveled = t_max.z;
                t_max.z += t_delta.z;

                if distance_traveled > range {
                    return None;
                }

                hit_normal = -step.z * IVec3::unit_z();
            }

            if Self::is_block_solid_at(cell_grid_position, world) {
                return Some((cell_grid_position, hit_normal));
            }
        }

        None
    }

    pub fn subdivide_area(
        area: &Area,
        area_id_generator: &mut IDGenerator,
        rng: &mut impl RngCore,
    ) -> Option<(Area, Area)> {
        let tower_area_size_min = TOWER_AREA_SIZE_MIN as i32;

        if gen_bool(rng) {
            let split_position = gen_range_i32(2, area.size.x - 2, rng);

            let west_size = split_position;
            let east_size = area.size.x - split_position + 1;

            if west_size >= tower_area_size_min && east_size >= tower_area_size_min {
                let west_area = Area {
                    area_id: IDGenerator::allocate(area_id_generator),
                    floor_number: area.floor_number,
                    kind: area::Kind::LowerRoom,
                    style: area::Style::GenericRoom,
                    grid_position: area.grid_position,
                    size: IVec3::new(west_size, area.size.y, area.size.z),
                    direction: area.direction,
                    connection_vec: Vec::new(),
                };

                let east_area = Area {
                    area_id: IDGenerator::allocate(area_id_generator),
                    floor_number: area.floor_number,
                    kind: area::Kind::LowerRoom,
                    style: area::Style::GenericRoom,
                    grid_position: area.grid_position + IVec3::new(split_position - 1, 0, 0),
                    size: IVec3::new(east_size, area.size.y, area.size.z),
                    direction: area.direction,
                    connection_vec: Vec::new(),
                };

                return Some((west_area, east_area));
            } else {
                return None;
            }
        } else {
            let split_position = gen_range_i32(2, area.size.y - 2, rng);

            let south_size = split_position;
            let north_size = area.size.y - split_position + 1;

            if south_size >= tower_area_size_min && north_size >= tower_area_size_min {
                let south_area = Area {
                    area_id: IDGenerator::allocate(area_id_generator),
                    floor_number: area.floor_number,
                    kind: area::Kind::LowerRoom,
                    style: area::Style::GenericRoom,
                    grid_position: area.grid_position,
                    size: IVec3::new(area.size.x, south_size, area.size.z),
                    direction: area.direction,
                    connection_vec: Vec::new(),
                };

                let north_area = Area {
                    area_id: IDGenerator::allocate(area_id_generator),
                    floor_number: area.floor_number,
                    kind: area::Kind::LowerRoom,
                    style: area::Style::GenericRoom,
                    grid_position: area.grid_position + IVec3::new(0, split_position - 1, 0),
                    size: IVec3::new(area.size.x, north_size, area.size.z),
                    direction: area.direction,
                    connection_vec: Vec::new(),
                };

                return Some((south_area, north_area));
            } else {
                return None;
            }
        }
    }
}
