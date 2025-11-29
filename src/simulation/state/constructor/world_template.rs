use crate::simulation::state::{self, World};

pub mod empty_template;
pub mod main_template;
pub mod test_template;

pub fn construct(template: state::Template, world: &mut World) {
    match template {
        state::Template::Empty => empty_template::construct(world),
        state::Template::Main => main_template::construct(world),
        state::Template::Test => test_template::construct(world),
    }
}
