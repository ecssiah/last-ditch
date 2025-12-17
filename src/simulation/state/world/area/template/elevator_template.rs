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

        World::set_box(
            area_ibox.min,
            area_ibox.max,
            block::Kind::Metal2,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 0, area_ibox.min.z + 1),
            IVec3::new(area_ibox.max.x - 2, area_ibox.max.y + 0, area_ibox.max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_ibox.min.x + 0, area_ibox.min.y + 2, area_ibox.min.z + 1),
            IVec3::new(area_ibox.max.x + 0, area_ibox.max.y - 2, area_ibox.max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 2, area_ibox.min.z + 0),
            IVec3::new(area_ibox.max.x - 2, area_ibox.max.y - 2, area_ibox.max.z + 0),
            block::Kind::None,
            &mut world.sector_vec,
        );

        // Stairs

        World::set_object(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 2, area_ibox.min.z + 0),
            Direction::South,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 3, area_ibox.min.z + 1),
            Direction::South,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 4, area_ibox.min.z + 1),
            Direction::South,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 3, area_ibox.min.y + 4, area_ibox.min.z + 2),
            Direction::West,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 4, area_ibox.min.y + 4, area_ibox.min.z + 2),
            Direction::West,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 4, area_ibox.min.y + 3, area_ibox.min.z + 3),
            Direction::North,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 4, area_ibox.min.y + 2, area_ibox.min.z + 3),
            Direction::North,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 3, area_ibox.min.y + 2, area_ibox.min.z + 4),
            Direction::East,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 2, area_ibox.min.z + 4),
            Direction::East,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 3, area_ibox.min.z + 5),
            Direction::South,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 4, area_ibox.min.z + 5),
            Direction::South,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 3, area_ibox.min.y + 4, area_ibox.min.z + 6),
            Direction::West,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 4, area_ibox.min.y + 4, area_ibox.min.z + 6),
            Direction::West,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 4, area_ibox.min.y + 3, area_ibox.min.z + 7),
            Direction::North,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 4, area_ibox.min.y + 2, area_ibox.min.z + 7),
            Direction::North,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_ibox.min.x + 3, area_ibox.min.y + 2, area_ibox.min.z + 8),
            Direction::East,
            object::Kind::Stairs,
            world,
        );
    }
}
