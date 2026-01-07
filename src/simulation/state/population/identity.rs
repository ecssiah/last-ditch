pub mod ethnicity;
pub mod role;
pub mod sex;

pub use ethnicity::Ethnicity;
pub use role::Role;
pub use sex::Sex;

use crate::simulation::state::population::{
    identity::ethnicity::skin_tone::SkinTone, nation::nation_kind::NationKind,
};

#[derive(Clone)]
pub struct Identity {
    pub age: u32,
    pub sex: Sex,
    pub role: Role,
    pub ethnicity: Ethnicity,
    pub skin_tone: SkinTone,
    pub nation_kind: NationKind,
}

impl Identity {
    pub fn new() -> Self {
        let identity = Identity {
            age: 28,
            sex: Sex::Male,
            role: Role::None,
            ethnicity: Ethnicity::default(),
            skin_tone: SkinTone::Color1,
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
