use crate::simulation::state::population::person::person_id::PersonID;
use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct RotateData {
    pub person_id: PersonID,
    pub rotation_angles: Vec3,
}
