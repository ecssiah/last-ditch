use std::fmt;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AreaID(u32);

impl AreaID {
    pub const fn new(id_value: u32) -> Self {
        Self(id_value)
    }
}

impl fmt::Debug for AreaID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AreaID").field(&self.0).finish()
    }
}
