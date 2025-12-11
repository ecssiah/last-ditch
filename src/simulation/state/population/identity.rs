pub mod role;
pub mod sex;
pub mod ethnicity;

pub use role::Role;
pub use sex::Sex;
pub use ethnicity::Ethnicity;

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
