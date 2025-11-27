use crate::simulation::state::{self, Population, world::World};

pub mod empty_template;
pub mod main_template;
pub mod test_template;

pub fn construct(template: state::Template, world: &World, entity: &mut Population) {
    match template {
        state::Template::Placeholder => {}
        state::Template::Empty => empty_template::construct(entity),
        state::Template::Main => main_template::construct(world, entity),
        state::Template::Test => test_template::construct(entity),
    }
}
