use crate::simulation::state::population::nation::nation_kind::NationKind;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Ethnicity {
    pub ethnicity_map: HashMap<NationKind, f32>,
}

impl Ethnicity {
    pub fn new() -> Self {
        let ethnicity_map = HashMap::from([
            (NationKind::Eagle, 0.0),
            (NationKind::Lion, 0.0),
            (NationKind::Wolf, 0.0),
            (NationKind::Horse, 0.0),
        ]);

        Self { ethnicity_map }
    }

    pub fn from_nation_kind(nation_kind: &NationKind) -> Self {
        let mut ethnicity_map = HashMap::from([
            (NationKind::Eagle, 0.0),
            (NationKind::Lion, 0.0),
            (NationKind::Wolf, 0.0),
            (NationKind::Horse, 0.0),
        ]);

        ethnicity_map.insert(nation_kind.clone(), 1.0);

        Self { ethnicity_map }
    }
}

impl Default for Ethnicity {
    fn default() -> Self {
        Self::new()
    }
}
