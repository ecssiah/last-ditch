use crate::simulation::state::{
    world::{area::template::Template, block, Area},
    World,
};

pub struct GenericRoomTemplate {}

impl Template for GenericRoomTemplate {
    fn construct(area: &Area, world: &mut World) {
        World::set_wireframe_box(
            area.min,
            area.max,
            block::Kind::Metal1,
            &mut world.sector_vec,
        );
    }
}
