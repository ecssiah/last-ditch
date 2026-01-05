use crate::simulation::state::population::person::person_id::PersonID;
use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MoveData {
    pub person_id: PersonID,
    pub move_direction: Vec3,
}
