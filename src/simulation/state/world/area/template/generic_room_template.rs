use crate::simulation::state::{World, world::{Area, area::template::Template, block}};

pub struct GenericRoomTemplate {

}

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