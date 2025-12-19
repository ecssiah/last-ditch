use crate::simulation::state::{
    world::{
        area::template::Template,
        block,
        grid::{self, Direction},
        object,
    },
    World,
};
use ultraviolet::IVec3;

pub struct ElevatorTemplate {}

impl Template for ElevatorTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        let area_ibox = grid::get_grid_ibox(area.grid_position, area.size);

        World::set_block_box(area_ibox.min, area_ibox.max, block::Kind::Metal2, world);

        World::remove_block_cube(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 0,
                area_ibox.min.z + 1,
            ),
            IVec3::new(
                area_ibox.max.x - 2,
                area_ibox.max.y + 0,
                area_ibox.max.z - 3,
            ),
            world,
        );

        World::remove_block_cube(
            IVec3::new(
                area_ibox.min.x + 0,
                area_ibox.min.y + 2,
                area_ibox.min.z + 1,
            ),
            IVec3::new(
                area_ibox.max.x + 0,
                area_ibox.max.y - 2,
                area_ibox.max.z - 3,
            ),
            world,
        );

        World::remove_block_cube(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 2,
                area_ibox.min.z + 0,
            ),
            IVec3::new(
                area_ibox.max.x - 2,
                area_ibox.max.y - 2,
                area_ibox.max.z + 0,
            ),
            world,
        );

        // Stairs

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 2,
                area_ibox.min.z + 0,
            ),
            object::Kind::Platform,
            Direction::South,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 3,
                area_ibox.min.z + 1,
            ),
            object::Kind::Stairs,
            Direction::South,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 4,
                area_ibox.min.z + 1,
            ),
            object::Kind::Platform,
            Direction::South,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 3,
                area_ibox.min.y + 4,
                area_ibox.min.z + 2,
            ),
            object::Kind::Stairs,
            Direction::West,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 4,
                area_ibox.min.y + 4,
                area_ibox.min.z + 2,
            ),
            object::Kind::Platform,
            Direction::West,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 4,
                area_ibox.min.y + 3,
                area_ibox.min.z + 3,
            ),
            object::Kind::Stairs,
            Direction::North,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 4,
                area_ibox.min.y + 2,
                area_ibox.min.z + 3,
            ),
            object::Kind::Platform,
            Direction::North,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 3,
                area_ibox.min.y + 2,
                area_ibox.min.z + 4,
            ),
            object::Kind::Stairs,
            Direction::East,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 2,
                area_ibox.min.z + 4,
            ),
            object::Kind::Platform,
            Direction::East,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 3,
                area_ibox.min.z + 5,
            ),
            object::Kind::Stairs,
            Direction::South,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 2,
                area_ibox.min.y + 4,
                area_ibox.min.z + 5,
            ),
            object::Kind::Platform,
            Direction::South,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 3,
                area_ibox.min.y + 4,
                area_ibox.min.z + 6,
            ),
            object::Kind::Stairs,
            Direction::West,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 4,
                area_ibox.min.y + 4,
                area_ibox.min.z + 6,
            ),
            object::Kind::Platform,
            Direction::West,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 4,
                area_ibox.min.y + 3,
                area_ibox.min.z + 7,
            ),
            object::Kind::Stairs,
            Direction::North,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 4,
                area_ibox.min.y + 2,
                area_ibox.min.z + 7,
            ),
            object::Kind::Platform,
            Direction::North,
            world,
        );

        World::set_object(
            IVec3::new(
                area_ibox.min.x + 3,
                area_ibox.min.y + 2,
                area_ibox.min.z + 8,
            ),
            object::Kind::Stairs,
            Direction::East,
            world,
        );
    }
}
