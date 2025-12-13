pub mod elevator_template;
pub mod generic_room_template;
pub mod wireframe_template;

pub use elevator_template::ElevatorTemplate;
pub use generic_room_template::GenericRoomTemplate;
pub use wireframe_template::WireframeTemplate;

use crate::simulation::state::{World, world::Area};

pub trait Template {
    fn construct(area: &Area, world: &mut World);
}