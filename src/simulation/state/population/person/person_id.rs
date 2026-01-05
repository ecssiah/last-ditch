use std::fmt;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersonID(u32);

impl PersonID {
    pub const JUDGE_ID_1: PersonID = PersonID::new(1);
    pub const JUDGE_ID_2: PersonID = PersonID::new(2);
    pub const JUDGE_ID_3: PersonID = PersonID::new(3);
    pub const JUDGE_ID_4: PersonID = PersonID::new(4);

    pub const fn new(id_value: u32) -> Self {
        Self(id_value)
    }
}

impl fmt::Debug for PersonID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PersonID").field(&self.0).finish()
    }
}
