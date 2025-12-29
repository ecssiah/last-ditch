use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contact {
    Ground = 1 << 0,
    Ladder = 1 << 1,
    Stairs = 1 << 2,
}

impl Contact {
    pub const ALL: &'static [Self] = &[Self::Ground, Self::Ladder, Self::Stairs];

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ground => "ground",
            Self::Ladder => "ladder",
            Self::Stairs => "stairs",
        }
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
