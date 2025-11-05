//! The simulated environment

pub mod block;
pub mod grid;
pub mod sector;

use crate::simulation::{
    self, constructor,
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{
            entity::{self, Judge},
            nation,
        },
        world::{block::Block, grid::Grid, sector::Sector},
    },
};
use glam::{IVec3, Vec3};
use std::collections::HashMap;

pub struct World {
    pub kind: simulation::Kind,
    pub grid: Grid,
    pub block_info_map: HashMap<block::Kind, block::Info>,
    pub sector_vec: Vec<sector::Sector>,
    pub flag_position_map: HashMap<nation::Kind, IVec3>,
}

impl World {
    pub fn new(kind: simulation::Kind) -> Self {
        let grid = Grid::new(kind);
        let block_info_map = block::Info::setup();
        let sector_vec = Self::setup_sector_vec(&grid);

        let flag_position_map = HashMap::from([
            (nation::Kind::Lion, IVec3::ZERO),
            (nation::Kind::Eagle, IVec3::ZERO),
            (nation::Kind::Horse, IVec3::ZERO),
            (nation::Kind::Wolf, IVec3::ZERO),
        ]);

        Self {
            kind,
            grid,
            block_info_map,
            sector_vec,
            flag_position_map,
        }
    }

    pub fn placeholder() -> Self {
        let kind = simulation::Kind::Placeholder;

        let grid = Grid::new(kind);
        let block_info_map = HashMap::default();
        let sector_vec = Vec::default();

        let flag_position_map = HashMap::default();

        Self {
            kind,
            grid,
            block_info_map,
            sector_vec,
            flag_position_map,
        }
    }

    pub fn get_flag(
        kind: entity::Kind,
        flag_position_map: HashMap<entity::Kind, IVec3>,
    ) -> Option<IVec3> {
        flag_position_map.get(&kind).cloned()
    }

    pub fn setup(kind: simulation::Kind, world: &mut World) {
        match kind {
            simulation::Kind::Main => {
                constructor::world::main::construct(world);
            }
            simulation::Kind::Empty => {
                constructor::world::empty::construct(world);
            }
            simulation::Kind::WorldTest => {
                constructor::world::world_test::construct(world);
            }
            simulation::Kind::GraphTest => {
                constructor::world::graph_test::construct(world);
            }
            simulation::Kind::Placeholder => (),
        }
    }

    fn setup_sector_vec(grid: &Grid) -> Vec<sector::Sector> {
        Grid::sector_ids(grid)
            .into_iter()
            .map(|sector_id| {
                let position = Grid::sector_id_to_position(grid, sector_id);

                sector::Sector {
                    id: sector_id,
                    modified: sector::Modified {
                        block: false,
                        boundary: false,
                    },
                    position,
                    aabb: AABB::new(
                        position.as_vec3(),
                        Vec3::splat(grid.sector_size_in_cells as f32),
                    ),
                    block_vec: Self::setup_block_vec(grid, sector_id),
                }
            })
            .collect()
    }

    fn setup_block_vec(grid: &Grid, sector_id: sector::ID) -> Vec<block::Block> {
        Grid::block_ids(grid)
            .into_iter()
            .map(|block_id| {
                let position = Grid::ids_to_position(grid, sector_id, block_id);

                block::Block {
                    id: block_id,
                    sector_id,
                    position,
                    kind: block::Kind::Empty,
                    solid: false,
                    face_array: Block::face_array(),
                }
            })
            .collect()
    }

    pub fn get_sector<'a>(
        sector_id: sector::ID,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a sector::Sector> {
        sector_vec_slice.get(usize::from(sector_id))
    }

    pub fn get_sector_at<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a sector::Sector> {
        let sector_id = Grid::position_to_sector_id(grid, position);

        if sector_id != sector::ID::MAX {
            sector_vec_slice.get(usize::from(sector_id))
        } else {
            None
        }
    }

    pub fn get_sector_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut sector::Sector> {
        let sector_id = Grid::position_to_sector_id(grid, position);

        if sector_id != sector::ID::MAX {
            sector_vec_slice.get_mut(usize::from(sector_id))
        } else {
            None
        }
    }

    pub fn get_block(
        sector_id: sector::ID,
        block_id: block::ID,
        sector_vec_slice: &[Sector],
    ) -> Option<&block::Block> {
        let sector = sector_vec_slice.get(usize::from(sector_id))?;

        sector.block_vec.get(usize::from(block_id))
    }

    pub fn get_block_mut(
        sector_id: sector::ID,
        block_id: block::ID,
        sector_vec_slice: &mut [Sector],
    ) -> Option<&mut block::Block> {
        let sector = sector_vec_slice.get_mut(usize::from(sector_id))?;

        sector.block_vec.get_mut(usize::from(block_id))
    }

    pub fn get_block_at<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a [Sector],
    ) -> Option<&'a block::Block> {
        let (sector_id, block_id) = Grid::position_to_ids(grid, position);

        if sector_id != sector::ID::MAX && block_id != block::ID::MAX {
            Self::get_block(sector_id, block_id, sector_vec_slice)
        } else {
            None
        }
    }

    pub fn get_block_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &'a mut [Sector],
    ) -> Option<&'a mut block::Block> {
        let (sector_id, block_id) = Grid::position_to_ids(grid, position);

        if sector_id != sector::ID::MAX && block_id != block::ID::MAX {
            Self::get_block_mut(sector_id, block_id, sector_vec_slice)
        } else {
            None
        }
    }

    pub fn get_clearance(position: IVec3, grid: &Grid, sector_vec_slice: &[Sector]) -> u32 {
        let ground_is_solid = Self::get_block_at(position + IVec3::NEG_Y, grid, sector_vec_slice)
            .is_some_and(|block| block.solid);

        let mut clearance = 0;

        if ground_is_solid {
            for level in 0..MAXIMUM_CLEARANCE {
                let level_position = position + IVec3::new(0, level as i32, 0);

                if let Some(block) = Self::get_block_at(level_position, grid, sector_vec_slice) {
                    if !block.solid {
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

    fn mark_updates(position1: IVec3, grid: &Grid, sector_vec_slice: &mut [Sector]) {
        let mut sector1_id = None;

        if let Some(sector1) = Self::get_sector_at_mut(position1, grid, sector_vec_slice) {
            sector1_id = Some(sector1.id);
            sector1.modified.block = true;

            if Grid::on_sector_boundary(grid, position1) {
                sector1.modified.boundary = true;
            }
        }

        if let Some(sector1_id) = sector1_id {
            for direction_offset in grid::Direction::face_offset_array() {
                let position2 = position1 + direction_offset;
                let sector_id2 = Grid::position_to_sector_id(grid, position2);

                if sector_id2 != sector::ID::MAX && sector1_id != sector_id2 {
                    if let Some(sector2) = sector_vec_slice.get_mut(usize::from(sector_id2)) {
                        sector2.modified.boundary = true;
                    }
                }
            }
        }
    }

    pub fn set_block_kind(
        position: IVec3,
        kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
        sector_vec_slice: &mut [Sector],
    ) -> bool {
        let (sector_id, block_id) = Grid::position_to_ids(grid, position);

        if sector_id != sector::ID::MAX && block_id != block::ID::MAX {
            let block_info = block_info_map.get(&kind).cloned().unwrap();

            if let Some(block) = Self::get_block_mut(sector_id, block_id, sector_vec_slice) {
                block.kind = kind;
                block.solid = block_info.solid;
            }

            Self::mark_updates(position, grid, sector_vec_slice);

            true
        } else {
            log::info!(
                "{:?} block cannot be set at invalid location: {:?}",
                kind,
                position
            );

            false
        }
    }

    pub fn set_box(
        position1: IVec3,
        position2: IVec3,
        kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
        sector_vec_slice: &mut [Sector],
    ) {
        let min = position1.min(position2);
        let max = position1.max(position2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
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
                        Self::set_block_kind(
                            position,
                            kind,
                            grid,
                            block_info_map,
                            sector_vec_slice,
                        );
                    } else {
                        Self::set_block_kind(
                            position,
                            block::Kind::Empty,
                            grid,
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
        kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
        sector_vec_slice: &mut [Sector],
    ) {
        let min = position1.min(position2);
        let max = position1.max(position2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let position = IVec3::new(x, y, z);

                    Self::set_block_kind(position, kind, grid, block_info_map, sector_vec_slice);
                }
            }
        }
    }

    pub fn update_sectors(grid: &Grid, sector_vec_slice: &mut [Sector]) {
        for sector_id in Grid::sector_ids(grid) {
            Self::update_sector(grid, sector_id, sector_vec_slice);
        }
    }

    fn update_sector(grid: &Grid, sector_id: sector::ID, sector_vec_slice: &mut [Sector]) {
        if Self::get_sector(sector_id, sector_vec_slice)
            .map_or(false, |sector| sector.modified.block)
        {
            for block_id in Grid::block_ids(grid) {
                let block = Self::get_block(sector_id, block_id, sector_vec_slice).unwrap();
                let face_exposure =
                    Self::compute_face_exposure(block.position, grid, sector_vec_slice);

                if let Some(block) = Self::get_block_mut(sector_id, block_id, sector_vec_slice) {
                    Self::update_block_faces(block, face_exposure);
                }
            }
        }
    }

    fn compute_face_exposure(
        position: IVec3,
        grid: &Grid,
        sector_vec_slice: &[Sector],
    ) -> [bool; 6] {
        let mut face_exposure = [false; 6];

        for (index, direction) in grid::Direction::face_array().iter().enumerate() {
            let neighbor_pos = position + direction.offset();

            if let Some(neighbor_block) = World::get_block_at(neighbor_pos, grid, sector_vec_slice)
            {
                face_exposure[index] = neighbor_block.kind == block::Kind::Empty;
            } else {
                face_exposure[index] = true;
            }
        }

        face_exposure
    }

    fn update_block_faces(block: &mut Block, face_exposure: [bool; 6]) {
        for (face, &exposed) in block.face_array.iter_mut().zip(face_exposure.iter()) {
            face.exposed = exposed;
        }
    }

    pub fn get_visible_sector_id_vec(
        judge: &Judge,
        grid: &Grid,
        sector_vec_slice: &[Sector],
    ) -> Vec<sector::ID> {
        let mut visible_sector_id_vec = Vec::new();

        let judge_sector_coordinates =
            Grid::world_to_sector_coordinates(grid, judge.spatial.world_position);

        let view_radius = 6;
        let view_direction = judge.spatial.forward();
        let view_origin = judge.spatial.eye() + judge.spatial.forward() * -8.0;

        for x in -view_radius..=view_radius {
            for y in -view_radius + 1..=view_radius - 1 {
                for z in -view_radius..=view_radius {
                    let sector_coordinates = judge_sector_coordinates + IVec3::new(x, y, z);
                    let sector_id = Grid::sector_coordinates_to_sector_id(grid, sector_coordinates);

                    if sector_id != sector::ID::MAX {
                        if let Some(sector) = sector_vec_slice.get(usize::from(sector_id)) {
                            let origin_to_center = sector.aabb.center() - view_origin;

                            if view_direction.dot(origin_to_center) >= 0.0 {
                                visible_sector_id_vec.push(sector_id);
                            }
                        }
                    }
                }
            }
        }

        visible_sector_id_vec
    }
}
