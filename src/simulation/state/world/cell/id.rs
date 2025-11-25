#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID(pub u32);

impl ID {
    pub const MAX: Self = Self(u32::MAX);

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn to_u32(&self) -> u32 {
        self.0 as u32
    }

    pub fn to_i32(&self) -> i32 {
        self.0 as i32
    }
}
