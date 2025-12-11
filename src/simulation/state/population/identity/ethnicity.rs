use crate::simulation::state::population::nation;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Ethnicity {
    pub ethnicity_map: HashMap<nation::Kind, f32>,
}

impl Ethnicity {
    pub fn new() -> Self {
        let ethnicity_map = HashMap::from([
            (nation::Kind::Eagle, 1.0),
            (nation::Kind::Lion, 0.0),
            (nation::Kind::Wolf, 0.0),
            (nation::Kind::Horse, 0.0),
        ]);

        Self { ethnicity_map }
    }
}
