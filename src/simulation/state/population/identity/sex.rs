use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sex::Male => write!(f, "man"),
            Sex::Female => write!(f, "woman"),
        }
    }
}
