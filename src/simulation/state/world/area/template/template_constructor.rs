use crate::simulation::state::{World, world::Area};

pub trait TemplateConstructor {
    fn construct(area: &Area, world: &mut World);
}