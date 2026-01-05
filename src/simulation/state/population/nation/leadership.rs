use crate::simulation::state::population::person::person_id::PersonID;

#[derive(Clone, Debug)]
pub struct Leadership {
    pub judge_id: PersonID,
}

impl Leadership {
    pub fn new() -> Self {
        Self {
            judge_id: PersonID::JUDGE_ID_1,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
