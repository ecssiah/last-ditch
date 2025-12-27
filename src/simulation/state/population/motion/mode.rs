use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Ground,
    Climb,
    Air,
}

impl Mode {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Mode::Ground => "ground",
            Mode::Climb => "climb",
            Mode::Air => "air",
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
