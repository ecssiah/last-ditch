use crate::simulation::state::population::person::person_id::PersonID;

#[derive(Clone, Debug)]
pub struct LeadershipView {
    pub judge_id: PersonID,
}

impl LeadershipView {
    pub fn new() -> Self {
        let judge_id = PersonID::JUDGE_ID_1;

        Self { judge_id }
    }
}

impl Default for LeadershipView {
    fn default() -> Self {
        Self::new()
    }
}
