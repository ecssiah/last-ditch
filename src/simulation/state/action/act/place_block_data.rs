use crate::simulation::state::population::person::person_id::PersonID;

#[derive(Clone, Copy, Debug)]
pub struct PlaceBlockData {
    pub person_id: PersonID,
}
