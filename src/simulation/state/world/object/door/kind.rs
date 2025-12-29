use std::fmt;

#[derive(Clone, Debug)]
pub enum Kind {
    Door1,
}

impl Kind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            self::Kind::Door1 => "door 1",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
