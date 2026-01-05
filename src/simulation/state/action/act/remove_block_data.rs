use crate::simulation::state::population::person::person_id::PersonID;

#[derive(Clone, Copy, Debug)]
pub struct RemoveBlockData {
    pub person_id: PersonID,
}
