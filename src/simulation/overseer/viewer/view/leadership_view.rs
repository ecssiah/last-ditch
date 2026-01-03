use crate::simulation::constants::ID_NULL;

#[derive(Clone, Debug)]
pub struct LeadershipView {
    pub judge_id: u64,
}

impl LeadershipView {
    pub fn new() -> Self {
        let judge_id = ID_NULL;

        Self { judge_id }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
