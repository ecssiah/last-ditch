pub mod age;
pub mod appearance;
pub mod ethnicity;
pub mod role;
pub mod sex;

use crate::simulation::state::population::{
    identity::{age::Age, ethnicity::Ethnicity, role::Role, sex::Sex},
    nation::nation_kind::NationKind,
};

#[derive(Clone)]
pub struct Identity {
    pub age: Age,
    pub sex: Sex,
    pub role: Role,
    pub ethnicity: Ethnicity,
    pub nation_kind: NationKind,
}

impl Identity {
    pub fn new() -> Self {
        let identity = Identity {
            age: Age::new(28),
            sex: Sex::Male,
            role: Role::None,
            ethnicity: Ethnicity::from_nation_kind(&NationKind::Eagle),
            nation_kind: NationKind::Eagle,
        };

        identity
    }
}

impl Default for Identity {
    fn default() -> Self {
        Self::new()
    }
}
