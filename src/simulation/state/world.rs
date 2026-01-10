//! The simulated environment

pub mod area;
pub mod block;
pub mod cell;
pub mod grid;
pub mod sector;
pub mod tower;

use crate::{
    simulation::{
        constants::*,
        state::{
            world::{
                area::{area_id::AreaID, Area, AreaKind},
                block::{
                    block_kind::BlockKind,
                    block_shape::BlockShape,
                    block_state::{block_data::BlockData, BlockState},
                    Block,
                },
                cell::cell_index::CellIndex,
                grid::{direction_set::DirectionSet, Direction},
                sector::{sector_index::SectorIndex, Sector},
                tower::Tower,
            },
            Time,
        },
    },
    utils::{
        id_generator::IDGenerator,
        ldmath::rand_chacha_ext::{gen_bool, gen_range_i32},
    },
};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use tracing::instrument;
use ultraviolet::{IVec3, Vec3};

pub struct World {
    pub active: bool,
    pub rng: ChaCha8Rng,
    pub time: Time,
    pub sector_vec: Vec<Sector>,
    pub tower: Tower,
    pub area_id_generator: IDGenerator,
    pub structure_id_generator: IDGenerator,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let time = Time::new();
        let sector_vec = Self::setup_sector_vec();
        let tower = Tower::new();
        let area_id_generator = IDGenerator::new();
        let structure_id_generator = IDGenerator::new();

        Self {
            active,
            rng,
            time,
            sector_vec,
            tower,
            area_id_generator,
            structure_id_generator,
        }
    }

    #[instrument(skip_all)]
    pub fn tick(world: &mut Self) {
        Time::tick(&mut world.time);
    }

    pub fn reset(world: &mut Self) {
        world.sector_vec = Self::setup_sector_vec();

        Tower::reset(&mut world.tower);
    }

    fn setup_sector_vec() -> Vec<Sector> {
        grid::sector_index_vec()
            .into_iter()
            .map(|sector_index| {
                let version = 0;
                let grid_position = grid::sector_index_to_grid_position(sector_index);
                let block_vec = vec![None; SECTOR_VOLUME_IN_CELLS];

                Sector {
                    version,
                    sector_index: sector_index,
                    grid_position,
                    block_vec,
                }
            })
            .collect()
    }

    pub fn get_sector(grid_position: IVec3, sector_vec_slice: &[Sector]) -> &Sector {
        let sector_index = grid::grid_position_to_sector_index(grid_position);

        let sector = Self::get_sector_by_index(&sector_index, sector_vec_slice);

        sector
    }

    pub fn get_sector_mut(grid_position: IVec3, sector_vec_slice: &mut [Sector]) -> &mut Sector {
        let sector_index = grid::grid_position_to_sector_index(grid_position);

        let sector = Self::get_sector_mut_by_index(&sector_index, sector_vec_slice);

        sector
    }

    pub fn get_sector_by_index<'a>(
        sector_index: &SectorIndex,
        sector_vec_slice: &'a [Sector],
    ) -> &'a Sector {
        &sector_vec_slice[SectorIndex::as_index(sector_index)]
    }

    pub fn get_sector_mut_by_index<'a>(
        sector_index: &SectorIndex,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut Sector {
        &mut sector_vec_slice[SectorIndex::as_index(sector_index)]
    }

    pub fn get_block(grid_position: IVec3, sector_vec_slice: &[Sector]) -> Option<&Block> {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector: &Sector = &sector_vec_slice[SectorIndex::as_index(&sector_index)];

            Sector::get_block(cell_index, &sector.block_vec)
        } else {
            None
        }
    }

    pub fn get_block_mut(
        grid_position: IVec3,
        sector_vec_slice: &mut [Sector],
    ) -> Option<&mut Block> {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &mut sector_vec_slice[SectorIndex::as_index(&sector_index)];

            Sector::get_block_mut(cell_index, &mut sector.block_vec)
        } else {
            None
        }
    }

    pub fn set_block(
        grid_position: IVec3,
        direction: &Direction,
        block_kind: &BlockKind,
        world: &mut Self,
    ) {
        if grid::grid_position_is_valid(grid_position) {
            let block_shape = BlockKind::get_block_shape(block_kind);

            let block = match block_shape {
                BlockShape::Block => {
                    let mut exposure_set = DirectionSet::ALL;

                    for direction in Direction::ALL {
                        let neighbor_grid_position = grid_position + Direction::to_ivec3(direction);

                        if grid::grid_position_is_valid(neighbor_grid_position) {
                            if let Some(neighbor_block) =
                                World::get_block_mut(neighbor_grid_position, &mut world.sector_vec)
                            {
                                let BlockState::Block(neighbor_block_data) =
                                    &mut neighbor_block.block_state
                                else {
                                    panic!("neighbor block is missing block state")
                                };

                                DirectionSet::remove(
                                    &Direction::to_opposing(direction),
                                    &mut neighbor_block_data.exposure_set,
                                );

                                DirectionSet::remove(direction, &mut exposure_set);
                            }
                        }
                    }

                    let mut block = Block::new(block_kind, grid_position, direction);
                    block.block_state = BlockState::Block(BlockData { exposure_set });

                    block
                }
                BlockShape::DoorLower => Block::new(block_kind, grid_position, direction),
                BlockShape::DoorUpper => Block::new(block_kind, grid_position, direction),
                BlockShape::Ladder => Block::new(block_kind, grid_position, direction),
                BlockShape::Stairs => Block::new(block_kind, grid_position, direction),
            };

            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &mut world.sector_vec[SectorIndex::as_index(&sector_index)];
            sector.block_vec[CellIndex::as_index(&cell_index)] = Some(block);
            sector.version += 1;
        }
    }

    pub fn set_block_wireframe(
        min: IVec3,
        max: IVec3,
        direction: &Direction,
        block_kind: &BlockKind,
        world: &mut Self,
    ) {
        for x in min.x..=max.x {
            Self::set_block(IVec3::new(x, min.y, min.z), direction, block_kind, world);
            Self::set_block(IVec3::new(x, max.y, min.z), direction, block_kind, world);
            Self::set_block(IVec3::new(x, min.y, max.z), direction, block_kind, world);
            Self::set_block(IVec3::new(x, max.y, max.z), direction, block_kind, world);
        }

        for y in min.y..=max.y {
            Self::set_block(IVec3::new(min.x, y, min.z), direction, block_kind, world);
            Self::set_block(IVec3::new(max.x, y, min.z), direction, block_kind, world);
            Self::set_block(IVec3::new(min.x, y, max.z), direction, block_kind, world);
            Self::set_block(IVec3::new(max.x, y, max.z), direction, block_kind, world);
        }

        for z in min.z..=max.z {
            Self::set_block(IVec3::new(min.x, min.y, z), direction, block_kind, world);
            Self::set_block(IVec3::new(min.x, max.y, z), direction, block_kind, world);
            Self::set_block(IVec3::new(max.x, min.y, z), direction, block_kind, world);
            Self::set_block(IVec3::new(max.x, max.y, z), direction, block_kind, world);
        }
    }

    pub fn set_block_box(
        min: IVec3,
        max: IVec3,
        direction: &Direction,
        block_kind: &BlockKind,
        world: &mut Self,
    ) {
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
                        Self::set_block(grid_position, direction, block_kind, world);
                    } else {
                        Self::remove_block(grid_position, world);
                    }
                }
            }
        }
    }

    pub fn set_block_shell(
        min: IVec3,
        max: IVec3,
        direction: &Direction,
        block_kind: &BlockKind,
        world: &mut Self,
    ) {
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
                        Self::set_block(grid_position, direction, block_kind, world);
                    }
                }
            }
        }
    }

    pub fn set_block_cube(
        min: IVec3,
        max: IVec3,
        direction: &Direction,
        block_kind: &BlockKind,
        world: &mut Self,
    ) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_block(position, direction, block_kind, world);
                }
            }
        }
    }

    pub fn remove_block(grid_position: IVec3, world: &mut Self) {
        if grid::grid_position_is_valid(grid_position) {
            for direction in Direction::ALL {
                let neighbor_grid_position = grid_position + Direction::to_ivec3(direction);

                if grid::grid_position_is_valid(neighbor_grid_position) {
                    if let Some(block) =
                        World::get_block_mut(neighbor_grid_position, &mut world.sector_vec)
                    {
                        let BlockState::Block(block_data) = &mut block.block_state else {
                            panic!("block should have block data")
                        };

                        DirectionSet::add(
                            &Direction::to_opposing(&direction),
                            &mut block_data.exposure_set,
                        );
                    }
                }
            }

            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &mut world.sector_vec[SectorIndex::as_index(&sector_index)];

            sector.block_vec[CellIndex::as_index(&cell_index)] = None;
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

    pub fn raycast_to_block(
        origin: Vec3,
        forward: Vec3,
        range: f32,
        world: &Self,
    ) -> Option<(IVec3, IVec3)> {
        let forward = forward.normalized();

        let mut cell_grid_position = grid::world_position_to_grid_position(origin);

        let step = IVec3::new(
            if forward.x > 0.0 { 1 } else { -1 },
            if forward.y > 0.0 { 1 } else { -1 },
            if forward.z > 0.0 { 1 } else { -1 },
        );

        let t_max = Vec3 {
            x: if forward.x != 0.0 {
                let boundary = if forward.x > 0.0 {
                    cell_grid_position.x as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_grid_position.x as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.x) / forward.x
            } else {
                f32::INFINITY
            },
            y: if forward.y != 0.0 {
                let boundary = if forward.y > 0.0 {
                    cell_grid_position.y as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_grid_position.y as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.y) / forward.y
            } else {
                f32::INFINITY
            },
            z: if forward.z != 0.0 {
                let boundary = if forward.z > 0.0 {
                    cell_grid_position.z as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_grid_position.z as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.z) / forward.z
            } else {
                f32::INFINITY
            },
        };

        let t_delta = Vec3::new(
            if forward.x != 0.0 {
                (1.0 / forward.x).abs()
            } else {
                f32::INFINITY
            },
            if forward.y != 0.0 {
                (1.0 / forward.y).abs()
            } else {
                f32::INFINITY
            },
            if forward.z != 0.0 {
                (1.0 / forward.z).abs()
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

            if World::get_block(cell_grid_position, &world.sector_vec).is_some() {
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
                    area_id: AreaID::new(IDGenerator::allocate(area_id_generator)),
                    area_kind: AreaKind::LowerRoom,
                    floor_number: area.floor_number,
                    style: area::Style::GenericRoom,
                    grid_position: area.grid_position,
                    size: IVec3::new(west_size, area.size.y, area.size.z),
                    direction: area.direction,
                    connection_vec: Vec::new(),
                };

                let east_area = Area {
                    area_id: AreaID::new(IDGenerator::allocate(area_id_generator)),
                    area_kind: AreaKind::LowerRoom,
                    floor_number: area.floor_number,
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
                    area_id: AreaID::new(IDGenerator::allocate(area_id_generator)),
                    area_kind: AreaKind::LowerRoom,
                    floor_number: area.floor_number,
                    style: area::Style::GenericRoom,
                    grid_position: area.grid_position,
                    size: IVec3::new(area.size.x, south_size, area.size.z),
                    direction: area.direction,
                    connection_vec: Vec::new(),
                };

                let north_area = Area {
                    area_id: AreaID::new(IDGenerator::allocate(area_id_generator)),
                    area_kind: AreaKind::LowerRoom,
                    floor_number: area.floor_number,
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
