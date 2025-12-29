use std::fmt;

#[derive(Clone, Debug)]
pub enum Kind {
    Stairs1,
}

impl Kind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            self::Kind::Stairs1 => "stairs 1",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
