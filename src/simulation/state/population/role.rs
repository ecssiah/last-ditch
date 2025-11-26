#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Role {
    Agent,
    Judge,
}

impl Role {
    pub fn from_string(string: &str) -> Option<Self> {
        if Self::matches_kind(string, "agent") {
            Some(Role::Agent)
        } else if Self::matches_kind(string, "judge") {
            Some(Role::Judge)
        } else {
            None
        }
    }

    fn matches_kind(string: &str, kind: &str) -> bool {
        string.to_ascii_lowercase().contains(kind)
    }
}
