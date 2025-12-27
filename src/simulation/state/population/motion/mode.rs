use std::fmt;

#[derive(Clone, Debug)]
pub enum Mode {
    Ground,
    Climb,
    Fly,
}

impl Mode {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Mode::Ground => "ground",
            Mode::Climb => "climb",
            Mode::Fly => "fly",
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
