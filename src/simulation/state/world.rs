//! The simulated environment

pub mod block;
pub mod cell;
pub mod grid;
pub mod sector;

use crate::simulation::{
    constants::*,
    state::{
        physics::aabb::AABB,
        population::nation,
        world::{cell::Cell, sector::Sector},
        Time,
    },
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct World {
    pub rng: ChaCha8Rng,
    pub time: Time,
    pub block_info_map: HashMap<block::Kind, block::Info>,
    pub sector_vec: Vec<sector::Sector>,
    pub flag_position_map: HashMap<nation::Kind, IVec3>,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let time = Time::new();
        let block_info_map = block::Info::setup();
        let sector_vec = Self::setup_sector_vec();

        let flag_position_map = HashMap::from([
            (nation::Kind::Eagle, IVec3::new(0, 0, 0)),
            (nation::Kind::Lion, IVec3::new(0, 0, 0)),
            (nation::Kind::Wolf, IVec3::new(0, 0, 0)),
            (nation::Kind::Horse, IVec3::new(0, 0, 0)),
        ]);

        Self {
            rng,
            time,
            block_info_map,
            sector_vec,
            flag_position_map,
        }
    }

    pub fn get_flag(
        nation_kind: nation::Kind,
        flag_position_map: HashMap<nation::Kind, IVec3>,
    ) -> Option<IVec3> {
        flag_position_map.get(&nation_kind).cloned()
    }

    fn setup_sector_vec() -> Vec<sector::Sector> {
        grid::sector_ids()
            .into_iter()
            .map(|sector_id| {
                let position = grid::sector_id_to_position(sector_id);
                let version = 0;

                let aabb = AABB::new(
                    Vec3::from(position),
                    Vec3::broadcast(SECTOR_SIZE_IN_CELLS as f32),
                );

                let cell_vec = Self::setup_cell_vec(sector_id);

                sector::Sector {
                    sector_id,
                    version,
                    position,
                    aabb,
                    cell_vec,
                }
            })
            .collect()
    }

    fn setup_cell_vec(sector_id: sector::ID) -> Vec<Cell> {
        grid::cell_ids()
            .into_iter()
            .map(|cell_id| {
                let position = grid::ids_to_position(sector_id, cell_id);

                Cell {
                    cell_id,
                    sector_id,
                    position,
                    block_kind: block::Kind::None,
                    solid: false,
                }
            })
            .collect()
    }

    pub fn get_sector<'a>(
        sector_id: sector::ID,
        sector_vec_slice: &'a [Sector],
    ) -> &'a sector::Sector {
        &sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_sector_mut<'a>(
        sector_id: sector::ID,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut sector::Sector {
        &mut sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_sector_at<'a>(
        position: IVec3,
        sector_vec_slice: &'a [Sector],
    ) -> &'a sector::Sector {
        let sector_id = grid::position_to_sector_id(position);

        &sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_sector_at_mut<'a>(
        position: IVec3,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut sector::Sector {
        let sector_id = grid::position_to_sector_id(position);

        &mut sector_vec_slice[sector_id.to_usize()]
    }

    pub fn get_cell(
        sector_id: sector::ID,
        cell_id: cell::ID,
        sector_vec_slice: &[Sector],
    ) -> &Cell {
        let sector = &sector_vec_slice[sector_id.to_usize()];

        &sector.cell_vec[cell_id.to_usize()]
    }

    pub fn get_cell_mut(
        sector_id: sector::ID,
        cell_id: cell::ID,
        sector_vec_slice: &mut [Sector],
    ) -> &mut Cell {
        let sector = &mut sector_vec_slice[sector_id.to_usize()];

        &mut sector.cell_vec[cell_id.to_usize()]
    }

    pub fn get_cell_at<'a>(position: IVec3, sector_vec_slice: &'a [Sector]) -> &'a Cell {
        let (sector_id, cell_id) = grid::position_to_ids(position);

        Self::get_cell(sector_id, cell_id, sector_vec_slice)
    }

    pub fn get_cell_at_mut<'a>(
        position: IVec3,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut Cell {
        let (sector_id, cell_id) = grid::position_to_ids(position);

        Self::get_cell_mut(sector_id, cell_id, sector_vec_slice)
    }

    pub fn get_clearance(position: IVec3, sector_vec_slice: &[Sector]) -> u32 {
        let ground_position = position + -1 * IVec3::unit_z();

        let ground_solid = if grid::position_valid(ground_position) {
            Self::get_cell_at(ground_position, sector_vec_slice).solid
        } else {
            false
        };

        let mut clearance = 0;

        if ground_solid {
            for level in 0..MAXIMUM_CLEARANCE {
                let level_position = position + IVec3::new(0, 0, level as i32);

                if grid::position_valid(level_position) {
                    if !Self::get_cell_at(level_position, sector_vec_slice).solid {
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

    pub fn set_block(
        position: IVec3,
        block_kind: block::Kind,
        block_info_map: &HashMap<block::Kind, block::Info>,
        sector_vec_slice: &mut [Sector],
    ) {
        let (sector_id, cell_id) = grid::position_to_ids(position);

        if grid::sector_id_valid(sector_id) && grid::cell_id_valid(cell_id) {
            let block_info = block_info_map[&block_kind];

            let cell = Self::get_cell_mut(sector_id, cell_id, sector_vec_slice);
            cell.block_kind = block_kind;
            cell.solid = block_info.solid;

            let sector = Self::get_sector_mut(sector_id, sector_vec_slice);
            sector.version += 1;
        }
    }

    pub fn set_box(
        position1: IVec3,
        position2: IVec3,
        block_kind: block::Kind,
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

                    let position = IVec3::new(x, y, z);

                    if on_boundary {
                        Self::set_block(position, block_kind, block_info_map, sector_vec_slice);
                    } else {
                        Self::set_block(
                            position,
                            block::Kind::None,
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

        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_block(position, block_kind, block_info_map, sector_vec_slice);
                }
            }
        }
    }

    pub fn raycast_to_block(
        origin: Vec3,
        direction: Vec3,
        range: f32,
        world: &World,
    ) -> Option<(IVec3, IVec3)> {
        let direction = direction.normalized();

        let mut cell_position = grid::world_position_to_position(origin);

        let step = IVec3::new(
            if direction.x > 0.0 { 1 } else { -1 },
            if direction.y > 0.0 { 1 } else { -1 },
            if direction.z > 0.0 { 1 } else { -1 },
        );

        let t_max = Vec3 {
            x: if direction.x != 0.0 {
                let boundary = if direction.x > 0.0 {
                    cell_position.x as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_position.x as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.x) / direction.x
            } else {
                f32::INFINITY
            },
            y: if direction.y != 0.0 {
                let boundary = if direction.y > 0.0 {
                    cell_position.y as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_position.y as f32 - CELL_RADIUS_IN_METERS
                };

                (boundary - origin.y) / direction.y
            } else {
                f32::INFINITY
            },
            z: if direction.z != 0.0 {
                let boundary = if direction.z > 0.0 {
                    cell_position.z as f32 + CELL_RADIUS_IN_METERS
                } else {
                    cell_position.z as f32 - CELL_RADIUS_IN_METERS
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
                cell_position.x += step.x;
                distance_traveled = t_max.x;
                t_max.x += t_delta.x;

                if distance_traveled > range {
                    return None;
                }

                hit_normal = -step.x * IVec3::unit_x();
            } else if t_max.y < t_max.z {
                cell_position.y += step.y;
                distance_traveled = t_max.y;
                t_max.y += t_delta.y;

                if distance_traveled > range {
                    return None;
                }

                hit_normal = -step.y * IVec3::unit_y();
            } else {
                cell_position.z += step.z;
                distance_traveled = t_max.z;
                t_max.z += t_delta.z;

                if distance_traveled > range {
                    return None;
                }

                hit_normal = -step.z * IVec3::unit_z();
            }

            let cell = Self::get_cell_at(cell_position, &world.sector_vec);

            if cell.solid {
                return Some((cell_position, hit_normal));
            }
        }

        None
    }
}
