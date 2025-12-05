pub mod role;
pub mod sex;

pub use role::Role;
pub use sex::Sex;

use crate::simulation::state::population::nation;

#[derive(Clone, Copy, Debug)]
pub struct Identity {
    pub age: u32,
    pub sex: sex::Sex,
    pub role: role::Role,
    pub nation_kind: nation::Kind,
}

impl Identity {
    pub fn new() -> Self {
        let identity = Identity {
            age: 28,
            sex: Sex::Male,
            role: Role::None,
            nation_kind: nation::Kind::Eagle,
        };

        identity
    }
}
