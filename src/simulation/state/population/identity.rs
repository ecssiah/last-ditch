pub mod ethnicity;
pub mod role;
pub mod sex;

pub use ethnicity::Ethnicity;
pub use role::Role;
pub use sex::Sex;

use crate::simulation::state::population::nation;

#[derive(Clone, Debug)]
pub struct Identity {
    pub age: u32,
    pub sex: Sex,
    pub role: Role,
    pub ethnicity: Ethnicity,
    pub nation_kind: nation::Kind,
}

impl Identity {
    pub fn new() -> Self {
        let identity = Identity {
            age: 28,
            sex: Sex::Male,
            role: Role::None,
            ethnicity: Ethnicity::new(),
            nation_kind: nation::Kind::Eagle,
        };

        identity
    }
}
