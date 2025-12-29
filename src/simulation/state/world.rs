//! The simulated environment

pub mod area;
pub mod block;
pub mod grid;
pub mod object;
pub mod sector;
pub mod tower;

use crate::{
    simulation::{
        constants::*,
        state::{
            world::{
                area::Area,
                block::Block,
                grid::{direction_set::DirectionSet, Direction},
                object::ObjectManager,
                sector::Sector,
                tower::Tower,
            },
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
use tracing::instrument;
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
                let object_manager = ObjectManager::new();

                Sector {
                    version,
                    sector_index,
                    grid_position,
                    block_vec,
                    object_manager,
                }
            })
            .collect()
    }

    pub fn get_sector(grid_position: IVec3, sector_vec_slice: &[Sector]) -> &Sector {
        let sector_index = grid::grid_position_to_sector_index(grid_position);

        let sector = &sector_vec_slice[sector_index];

        sector
    }

    pub fn get_sector_mut(grid_position: IVec3, sector_vec_slice: &mut [Sector]) -> &mut Sector {
        let sector_index = grid::grid_position_to_sector_index(grid_position);

        let sector = &mut sector_vec_slice[sector_index];

        sector
    }

    pub fn get_block(grid_position: IVec3, sector_vec_slice: &[Sector]) -> Option<&Block> {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &sector_vec_slice[sector_index];

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

            let sector = &mut sector_vec_slice[sector_index];

            Sector::get_block_mut(cell_index, &mut sector.block_vec)
        } else {
            None
        }
    }

    pub fn set_block(grid_position: IVec3, block_kind: &block::Kind, world: &mut Self) {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let block = Block::new(block_kind);

            let sector = &mut world.sector_vec[sector_index];
            sector.block_vec[cell_index] = Some(block);
            sector.version += 1;

            Self::update_block_exposure(grid_position, world);
        }
    }

    pub fn update_block_exposure(grid_position: IVec3, world: &mut Self) {
        if !grid::grid_position_is_valid(grid_position) {
            return;
        }

        let block_is_solid = World::is_block_solid_at(grid_position, world);

        for direction in Direction::ALL {
            let neighbor_grid_position = grid_position + Direction::to_ivec3(direction);

            if grid::grid_position_is_valid(neighbor_grid_position) {
                let neighbor_is_solid = World::is_block_solid_at(neighbor_grid_position, world);

                if block_is_solid && !neighbor_is_solid {
                    if let Some(block) = World::get_block_mut(grid_position, &mut world.sector_vec)
                    {
                        DirectionSet::add(*direction, &mut block.exposure_set);
                    }

                    if let Some(neighbor_block) =
                        World::get_block_mut(neighbor_grid_position, &mut world.sector_vec)
                    {
                        DirectionSet::remove(
                            Direction::to_opposing(direction),
                            &mut neighbor_block.exposure_set,
                        );
                    }
                } else if !block_is_solid && neighbor_is_solid {
                    if let Some(block) = World::get_block_mut(grid_position, &mut world.sector_vec)
                    {
                        DirectionSet::remove(*direction, &mut block.exposure_set);
                    }

                    if let Some(neighbor_block) =
                        World::get_block_mut(neighbor_grid_position, &mut world.sector_vec)
                    {
                        DirectionSet::add(
                            Direction::to_opposing(direction),
                            &mut neighbor_block.exposure_set,
                        );
                    }
                } else {
                    if let Some(block) = World::get_block_mut(grid_position, &mut world.sector_vec)
                    {
                        DirectionSet::remove(*direction, &mut block.exposure_set);
                    }

                    if let Some(neighbor_block) =
                        World::get_block_mut(neighbor_grid_position, &mut world.sector_vec)
                    {
                        DirectionSet::remove(
                            Direction::to_opposing(direction),
                            &mut neighbor_block.exposure_set,
                        );
                    }
                }
            } else {
                if block_is_solid {
                    if let Some(block) = World::get_block_mut(grid_position, &mut world.sector_vec)
                    {
                        DirectionSet::add(*direction, &mut block.exposure_set);
                    }
                }
            }
        }
    }

    pub fn set_block_wireframe(min: IVec3, max: IVec3, block_kind: &block::Kind, world: &mut Self) {
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

    pub fn set_block_box(min: IVec3, max: IVec3, block_kind: &block::Kind, world: &mut Self) {
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

    pub fn set_block_shell(min: IVec3, max: IVec3, block_kind: &block::Kind, world: &mut Self) {
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

    pub fn set_block_cube(min: IVec3, max: IVec3, block_kind: &block::Kind, world: &mut Self) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_block(position, block_kind, world);
                }
            }
        }
    }

    pub fn remove_block(grid_position: IVec3, world: &mut Self) {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &mut world.sector_vec[sector_index];

            sector.block_vec[cell_index] = None;
            sector.version += 1;

            Self::update_block_exposure(grid_position, world);
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

    pub fn is_block_solid_at(grid_position: IVec3, world: &World) -> bool {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &world.sector_vec[sector_index];

            Sector::get_block(cell_index, &sector.block_vec).is_some_and(|block| block.solid)
        } else {
            true
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
