use crate::simulation::constants::ID_NULL;

#[derive(Clone, Debug)]
pub struct Leadership {
    pub judge_id: u64,
}

impl Leadership {
    pub fn new() -> Self {
        Self { judge_id: ID_NULL }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
