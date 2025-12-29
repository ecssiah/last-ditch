use ultraviolet::IVec3;

use crate::simulation::{
    constants::SECTOR_VOLUME_IN_CELLS,
    state::{
        world::{
            grid::{self, Direction},
            object::{door, ladder, stairs, Door, Ladder, Stairs},
        },
        World,
    },
};

#[derive(Clone, Debug)]
pub struct ObjectManager {
    pub door_vec: Vec<Option<Door>>,
    pub stairs_vec: Vec<Option<Stairs>>,
    pub ladder_vec: Vec<Option<Ladder>>,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            door_vec: vec![None; SECTOR_VOLUME_IN_CELLS],
            stairs_vec: vec![None; SECTOR_VOLUME_IN_CELLS],
            ladder_vec: vec![None; SECTOR_VOLUME_IN_CELLS],
        }
    }

    pub fn get_door(grid_position: IVec3, world: &World) -> Option<&Door> {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &world.sector_vec[sector_index];

            sector.object_manager.door_vec[cell_index].as_ref()
        } else {
            None
        }
    }

    pub fn set_door(
        grid_position: IVec3,
        door_kind: &door::Kind,
        direction: &Direction,
        world: &mut World,
    ) {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let door = Door::new(door_kind, direction);

            let sector = &mut world.sector_vec[sector_index];
            sector.object_manager.door_vec[cell_index] = Some(door);
            sector.version += 1;
        }
    }

    pub fn get_stairs(grid_position: IVec3, world: &World) -> Option<&Stairs> {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &world.sector_vec[sector_index];

            sector.object_manager.stairs_vec[cell_index].as_ref()
        } else {
            None
        }
    }

    pub fn set_stairs(
        grid_position: IVec3,
        stairs_kind: &stairs::Kind,
        direction: &Direction,
        world: &mut World,
    ) {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let stairs = Stairs::new(stairs_kind, direction);

            let sector = &mut world.sector_vec[sector_index];
            sector.object_manager.stairs_vec[cell_index] = Some(stairs);
            sector.version += 1;
        }
    }

    pub fn set_stairs_cube(
        min: IVec3,
        max: IVec3,
        stairs_kind: &stairs::Kind,
        direction: &Direction,
        world: &mut World,
    ) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_stairs(position, stairs_kind, direction, world);
                }
            }
        }
    }

    pub fn get_ladder(grid_position: IVec3, world: &World) -> Option<&Ladder> {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let sector = &world.sector_vec[sector_index];

            sector.object_manager.ladder_vec[cell_index].as_ref()
        } else {
            None
        }
    }

    pub fn set_ladder(
        grid_position: IVec3,
        ladder_kind: &ladder::Kind,
        direction: &Direction,
        world: &mut World,
    ) {
        if grid::grid_position_is_valid(grid_position) {
            let (sector_index, cell_index) = grid::grid_position_to_indices(grid_position);

            let ladder = Ladder::new(ladder_kind, direction);

            let sector = &mut world.sector_vec[sector_index];
            sector.object_manager.ladder_vec[cell_index] = Some(ladder);
            sector.version += 1;
        }
    }

    pub fn set_ladder_cube(
        min: IVec3,
        max: IVec3,
        ladder_kind: &ladder::Kind,
        direction: &Direction,
        world: &mut World,
    ) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let position = IVec3::new(x, y, z);

                    Self::set_ladder(position, ladder_kind, direction, world);
                }
            }
        }
    }
}
