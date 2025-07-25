#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Agent,
    Judge,
}

impl Kind {
    pub fn from_string(string: &str) -> Option<Self> {
        if Self::matches_kind(string, "agent") {
            Some(Kind::Agent)
        } else if Self::matches_kind(string, "judge") {
            Some(Kind::Judge)
        } else {
            None
        }
    }

    fn matches_kind(string: &str, kind: &str) -> bool {
        string.to_ascii_lowercase().contains(kind)
    }
}
