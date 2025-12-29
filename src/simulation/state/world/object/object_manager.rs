use ultraviolet::IVec3;

use crate::simulation::{
    constants::SECTOR_VOLUME_IN_CELLS,
    state::{
        world::{
            grid::{self, Direction},
            object::{door, Door, Ladder, Stairs},
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
}
