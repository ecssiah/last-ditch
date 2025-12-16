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
        let (area_min, area_max) = grid::get_bounds(area.grid_position, area.size);

        World::set_box(
            area_min,
            area_max,
            block::Kind::Metal2,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_min.x + 2, area_min.y + 0, area_min.z + 1),
            IVec3::new(area_max.x - 2, area_max.y + 0, area_max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_min.x + 0, area_min.y + 2, area_min.z + 1),
            IVec3::new(area_max.x + 0, area_max.y - 2, area_max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_min.x + 2, area_min.y + 2, area_min.z + 0),
            IVec3::new(area_max.x - 2, area_max.y - 2, area_max.z + 0),
            block::Kind::None,
            &mut world.sector_vec,
        );

        // Stairs

        World::set_object(
            IVec3::new(area_min.x + 2, area_min.y + 2, area_min.z + 0),
            Direction::South,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 2, area_min.y + 3, area_min.z + 1),
            Direction::South,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 2, area_min.y + 4, area_min.z + 1),
            Direction::South,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 3, area_min.y + 4, area_min.z + 2),
            Direction::West,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 4, area_min.y + 4, area_min.z + 2),
            Direction::West,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 4, area_min.y + 3, area_min.z + 3),
            Direction::North,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 4, area_min.y + 2, area_min.z + 3),
            Direction::North,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 3, area_min.y + 2, area_min.z + 4),
            Direction::East,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 2, area_min.y + 2, area_min.z + 4),
            Direction::East,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 2, area_min.y + 3, area_min.z + 5),
            Direction::South,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 2, area_min.y + 4, area_min.z + 5),
            Direction::South,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 3, area_min.y + 4, area_min.z + 6),
            Direction::West,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 4, area_min.y + 4, area_min.z + 6),
            Direction::West,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 4, area_min.y + 3, area_min.z + 7),
            Direction::North,
            object::Kind::Stairs,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 4, area_min.y + 2, area_min.z + 7),
            Direction::North,
            object::Kind::Platform,
            world,
        );

        World::set_object(
            IVec3::new(area_min.x + 3, area_min.y + 2, area_min.z + 8),
            Direction::East,
            object::Kind::Stairs,
            world,
        );
    }
}
