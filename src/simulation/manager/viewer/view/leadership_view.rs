use crate::simulation::constants::JUDGE_ID_0;

#[derive(Clone, Debug)]
pub struct LeadershipView {
    pub judge_id: u64,
}

impl LeadershipView {
    pub fn new() -> Self {
        let judge_id = JUDGE_ID_0;

        Self { judge_id }
    }
}
