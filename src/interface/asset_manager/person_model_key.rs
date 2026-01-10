use crate::simulation::state::population::identity::sex::Sex;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PersonModelKey {
    pub sex: Sex,
    pub age_period: u32,
}

impl PersonModelKey {
    pub fn from_sex_and_age(sex: &Sex, age_period: u32) -> Self {
        Self {
            sex: sex.clone(),
            age_period,
        }
    }
}
