use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contact {
    Ground = 1 << 0,
    Ladder = 1 << 1,
    StairsNorth = 1 << 2,
    StairsWest = 1 << 3,
    StairsSouth = 1 << 4,
    StairsEast = 1 << 5,
}

impl Contact {
    pub const ALL: &'static [Self] = &[
        Self::Ground,
        Self::Ladder,
        Self::StairsNorth,
        Self::StairsWest,
        Self::StairsSouth,
        Self::StairsEast,
    ];

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ground => "ground",
            Self::Ladder => "ladder",
            Self::StairsNorth => "stairs north",
            Self::StairsWest => "stairs west",
            Self::StairsSouth => "stairs south",
            Self::StairsEast => "stairs east",
        }
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
