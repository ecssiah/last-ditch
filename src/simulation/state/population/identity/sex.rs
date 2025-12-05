#[derive(Clone, Copy, Debug)]
pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn to_string(sex: Self) -> String {
        match sex {
            Self::Male => String::from("man"),
            Self::Female => String::from("woman"),
        }
    }
}
