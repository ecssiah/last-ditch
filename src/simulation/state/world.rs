//! The simulated environment

pub mod block;
pub mod cell;
pub mod grid;
pub mod object;
pub mod sector;
pub mod structure;

pub use cell::Cell;
pub use object::Object;
pub use sector::Sector;

use crate::simulation::{
    constants::*,
    state::{
        physics::box_collider::BoxCollider,
        population::nation,
        world::{
            self,
            grid::{Area, Axis},
        },
        Time,
    },
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct World {
    pub active: bool,
    pub random_number_generator: ChaCha8Rng,
    pub time: Time,
    pub sector_vec: Vec<world::Sector>,
    pub object_map: HashMap<usize, Vec<Object>>,
    pub area_map: HashMap<u64, Area>,
    pub next_area_id: u64,
    pub next_object_id: u64,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let random_number_generator = ChaCha8Rng::seed_from_u64(seed);
        let time = Time::new();
        let sector_vec = Self::setup_sector_vec();
        let area_map = HashMap::new();
        let object_map = Self::setup_object_map();

        let next_area_id = 0;
        let next_object_id = 0;

        Self {
            active,
            random_number_generator,
            time,
            sector_vec,
            object_map,
            area_map,
            next_area_id,
            next_object_id,
        }
    }

    pub fn tick(world: &mut Self) {
        let _ = tracing::info_span!("world_tick").entered();

        if !world.active {
            return;
        }

        Time::tick(&mut world.time);
    }

    pub fn reset(world: &mut Self) {
        world.sector_vec = Self::setup_sector_vec();
        world.area_map.clear();

        for object_vec in world.object_map.values_mut() {
            object_vec.clear();
        }
    }

    pub fn get_next_area_id(world: &mut Self) -> u64 {
        let area_id = world.next_area_id;

        world.next_area_id += 1;

        area_id
    }

    pub fn get_next_object_id(world: &mut Self) -> u64 {
        let object_id = world.next_object_id;

        world.next_object_id += 1;

        object_id
    }

    pub fn get_flag(
        nation_kind: nation::Kind,
        home_position_map: HashMap<nation::Kind, IVec3>,
    ) -> Option<IVec3> {
        home_position_map.get(&nation_kind).cloned()
    }

    pub fn get_floor_position(floor_number: i32) -> i32 {
        let floor_position = floor_number * FLOOR_HEIGHT as i32 - 1;

        floor_position
    }

    fn setup_sector_vec() -> Vec<world::Sector> {
        grid::sector_id_vec()
            .into_iter()
            .map(|sector_id| {
                let grid_position = grid::sector_id_to_grid_position(sector_id);
                let version = 0;

                let box_collider = BoxCollider::new(
                    Vec3::from(grid_position),
                    Vec3::broadcast(SECTOR_SIZE_IN_CELLS as f32),
                );

                let cell_vec = Self::setup_cell_vec(sector_id);

                Sector {
                    sector_id,
                    version,
                    grid_position,
                    box_collider,
                    cell_vec,
                }
            })
            .collect()
    }

    fn setup_object_map() -> HashMap<usize, Vec<Object>> {
        grid::sector_id_vec()
            .into_iter()
            .map(|sector_id| (sector_id, Vec::new()))
            .collect()
    }

    pub fn set_object(
        grid_position: IVec3,
        direction: grid::Direction,
        object_kind: object::Kind,
        world: &mut Self,
    ) {
        let object = Object {
            object_id: World::get_next_object_id(world),
            kind: object_kind,
            grid_position,
            direction,
        };

        let sector_id = grid::grid_position_to_sector_id(grid_position);

        if let Some(object_vec) = world.object_map.get_mut(&sector_id) {
            object_vec.push(object);
        }
    }

    fn setup_cell_vec(sector_id: usize) -> Vec<world::Cell> {
        grid::cell_id_vec()
            .into_iter()
            .map(|cell_id| {
                let grid_position = grid::ids_to_grid_position(sector_id, cell_id);

                world::Cell {
                    cell_id,
                    sector_id,
                    grid_position,
                    block_kind: block::Kind::None,
                    solid: false,
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

    pub fn get_cell(sector_id: usize, cell_id: usize, sector_vec_slice: &[Sector]) -> &world::Cell {
        let sector = &sector_vec_slice[sector_id];
        let cell = &sector.cell_vec[cell_id];

        cell
    }

    pub fn get_cell_mut(
        sector_id: usize,
        cell_id: usize,
        sector_vec_slice: &mut [Sector],
    ) -> &mut world::Cell {
        let sector = &mut sector_vec_slice[sector_id];

        let cell = &mut sector.cell_vec[cell_id];

        cell
    }

    pub fn get_cell_at<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a [Sector],
    ) -> &'a world::Cell {
        let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

        let cell = Self::get_cell(sector_id, cell_id, sector_vec_slice);

        cell
    }

    pub fn get_cell_at_mut<'a>(
        grid_position: IVec3,
        sector_vec_slice: &'a mut [Sector],
    ) -> &'a mut world::Cell {
        let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

        let cell = Self::get_cell_mut(sector_id, cell_id, sector_vec_slice);

        cell
    }

    pub fn get_clearance(grid_position: IVec3, sector_vec_slice: &[Sector]) -> u32 {
        let ground_grid_position = grid_position + -1 * IVec3::unit_z();

        let is_ground_solid = if grid::is_grid_position_valid(ground_grid_position) {
            Self::get_cell_at(ground_grid_position, sector_vec_slice).solid
        } else {
            false
        };

        let mut clearance = 0;

        if is_ground_solid {
            for level in 0..MAXIMUM_CLEARANCE {
                let level_grid_position = grid_position + IVec3::new(0, 0, level as i32);

                if grid::is_grid_position_valid(level_grid_position) {
                    if !Self::get_cell_at(level_grid_position, sector_vec_slice).solid {
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
        grid_position: IVec3,
        block_kind: block::Kind,
        sector_vec_slice: &mut [Sector],
    ) {
        let (sector_id, cell_id) = grid::grid_position_to_ids(grid_position);

        if grid::is_sector_id_valid(sector_id) && grid::is_cell_id_valid(cell_id) {
            let block_info = block::get_info(block_kind);

            let cell = Self::get_cell_mut(sector_id, cell_id, sector_vec_slice);
            cell.block_kind = block_kind;
            cell.solid = block_info.solid;

            let sector = Self::get_sector_mut(sector_id, sector_vec_slice);
            sector.version += 1;
        }
    }

    pub fn set_frame(
        grid_position: IVec3,
        size: (usize, usize),
        normal_axis: Axis,
        block_kind: block::Kind,
        world: &mut Self,
    ) {
        let (u, v) = match normal_axis {
            Axis::X => (1, 2),
            Axis::Y => (0, 2),
            Axis::Z => (0, 1),
        };

        let min_u = grid_position[u];
        let min_v = grid_position[v];
        let max_u = min_u + size.0 as i32 - 1;
        let max_v = min_v + size.1 as i32 - 1;

        for vy in 0..size.1 {
            let v_val = min_v + vy as i32;

            let mut block_grid_position = grid_position;
            block_grid_position[v] = v_val;
            block_grid_position[u] = min_u;
            World::set_block(block_grid_position, block_kind, &mut world.sector_vec);

            let mut block_grid_position = grid_position;
            block_grid_position[v] = v_val;
            block_grid_position[u] = max_u;
            World::set_block(block_grid_position, block_kind, &mut world.sector_vec);
        }

        for ux in 0..size.0 {
            let u_val = min_u + ux as i32;

            let mut block_grid_position = grid_position;
            block_grid_position[u] = u_val;
            block_grid_position[v] = min_v;
            World::set_block(block_grid_position, block_kind, &mut world.sector_vec);

            let mut block_grid_position = grid_position;
            block_grid_position[u] = u_val;
            block_grid_position[v] = max_v;
            World::set_block(block_grid_position, block_kind, &mut world.sector_vec);
        }

        if size.0 > 2 && size.1 > 2 {
            let interior_min_u = min_u + 1;
            let interior_max_u = max_u - 1;
            let interior_min_v = min_v + 1;
            let interior_max_v = max_v - 1;

            for vy in interior_min_v..=interior_max_v {
                for ux in interior_min_u..=interior_max_u {
                    let mut block_grid_position = grid_position;
                    block_grid_position[u] = ux;
                    block_grid_position[v] = vy;

                    World::set_block(
                        block_grid_position,
                        block::Kind::None,
                        &mut world.sector_vec,
                    );
                }
            }
        }
    }

    pub fn set_plane(
        grid_position: IVec3,
        size: (usize, usize),
        normal_axis: Axis,
        block_kind: block::Kind,
        world: &mut Self,
    ) {
        let (u, v) = match normal_axis {
            Axis::X => (1, 2),
            Axis::Y => (0, 2),
            Axis::Z => (0, 1),
        };

        for vy in 0..size.1 {
            for ux in 0..size.0 {
                let mut block_grid_position = grid_position;
                block_grid_position[u] += ux as i32;
                block_grid_position[v] += vy as i32;

                World::set_block(block_grid_position, block_kind, &mut world.sector_vec);
            }
        }
    }

    pub fn set_wireframe_box(
        grid_position1: IVec3,
        grid_position2: IVec3,
        block_kind: block::Kind,
        sector_vec_slice: &mut [Sector],
    ) {
        let min = IVec3::new(
            grid_position1.x.min(grid_position2.x),
            grid_position1.y.min(grid_position2.y),
            grid_position1.z.min(grid_position2.z),
        );

        let max = IVec3::new(
            grid_position1.x.max(grid_position2.x),
            grid_position1.y.max(grid_position2.y),
            grid_position1.z.max(grid_position2.z),
        );

        let set_block = |block_grid_position: IVec3, sector_vec_slice: &mut [Sector]| {
            Self::set_block(block_grid_position, block_kind, sector_vec_slice);
        };

        // 12 edges of the BoxCollider
        //
        // 4 edges along X at min.y/min.z and max.y/min.z
        // 4 edges along Y at min.x/min.z and max.x/min.z
        // 4 vertical edges at min.x/min.y and max.x/max.y

        // --- X edges ---
        for x in min.x..=max.x {
            // bottom rectangle (z = min.z)
            set_block(IVec3::new(x, min.y, min.z), sector_vec_slice);
            set_block(IVec3::new(x, max.y, min.z), sector_vec_slice);

            // top rectangle (z = max.z)
            set_block(IVec3::new(x, min.y, max.z), sector_vec_slice);
            set_block(IVec3::new(x, max.y, max.z), sector_vec_slice);
        }

        // --- Y edges ---
        for y in min.y..=max.y {
            // bottom rectangle (z = min.z)
            set_block(IVec3::new(min.x, y, min.z), sector_vec_slice);
            set_block(IVec3::new(max.x, y, min.z), sector_vec_slice);

            // top rectangle (z = max.z)
            set_block(IVec3::new(min.x, y, max.z), sector_vec_slice);
            set_block(IVec3::new(max.x, y, max.z), sector_vec_slice);
        }

        // --- Vertical Z edges ---
        for z in min.z..=max.z {
            set_block(IVec3::new(min.x, min.y, z), sector_vec_slice);
            set_block(IVec3::new(min.x, max.y, z), sector_vec_slice);
            set_block(IVec3::new(max.x, min.y, z), sector_vec_slice);
            set_block(IVec3::new(max.x, max.y, z), sector_vec_slice);
        }
    }

    pub fn set_box(
        grid_position1: IVec3,
        grid_position2: IVec3,
        block_kind: block::Kind,
        sector_vec_slice: &mut [Sector],
    ) {
        let min = IVec3::new(
            grid_position1.x.min(grid_position2.x),
            grid_position1.y.min(grid_position2.y),
            grid_position1.z.min(grid_position2.z),
        );

        let max = IVec3::new(
            grid_position1.x.max(grid_position2.x),
            grid_position1.y.max(grid_position2.y),
            grid_position1.z.max(grid_position2.z),
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

                    let grid_position = IVec3::new(x, y, z);

                    if on_boundary {
                        Self::set_block(grid_position, block_kind, sector_vec_slice);
                    } else {
                        Self::set_block(grid_position, block::Kind::None, sector_vec_slice);
                    }
                }
            }
        }
    }

    pub fn set_shell(
        grid_position1: IVec3,
        grid_position2: IVec3,
        block_kind: block::Kind,
        sector_vec_slice: &mut [Sector],
    ) {
        let min = IVec3::new(
            grid_position1.x.min(grid_position2.x),
            grid_position1.y.min(grid_position2.y),
            grid_position1.z.min(grid_position2.z),
        );

        let max = IVec3::new(
            grid_position1.x.max(grid_position2.x),
            grid_position1.y.max(grid_position2.y),
            grid_position1.z.max(grid_position2.z),
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

                    let grid_position = IVec3::new(x, y, z);

                    if on_boundary {
                        Self::set_block(grid_position, block_kind, sector_vec_slice);
                    }
                }
            }
        }
    }

    pub fn set_cube(
        grid_position1: IVec3,
        grid_position2: IVec3,
        block_kind: block::Kind,
        sector_vec_slice: &mut [Sector],
    ) {
        let grid_position_min = IVec3::new(
            grid_position1.x.min(grid_position2.x),
            grid_position1.y.min(grid_position2.y),
            grid_position1.z.min(grid_position2.z),
        );

        let grid_position_max = IVec3::new(
            grid_position1.x.max(grid_position2.x),
            grid_position1.y.max(grid_position2.y),
            grid_position1.z.max(grid_position2.z),
        );

        for z in grid_position_min.z..=grid_position_max.z {
            for y in grid_position_min.y..=grid_position_max.y {
                for x in grid_position_min.x..=grid_position_max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_block(position, block_kind, sector_vec_slice);
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

        let mut cell_position = grid::world_position_to_grid_position(origin);

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
