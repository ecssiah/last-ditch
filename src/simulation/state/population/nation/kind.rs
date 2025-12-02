#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Eagle,
    Lion,
    Horse,
    Wolf,
}

impl Kind {
    pub const ALL: [Kind; 4] = [Kind::Eagle, Kind::Lion, Kind::Horse, Kind::Wolf];

    pub fn from_string(string: &str) -> Option<Self> {
        if Self::matches_kind(string, "eagle") {
            Some(Kind::Eagle)
        } else if Self::matches_kind(string, "horse") {
            Some(Kind::Horse)
        } else if Self::matches_kind(string, "lion") {
            Some(Kind::Lion)
        } else if Self::matches_kind(string, "wolf") {
            Some(Kind::Wolf)
        } else {
            None
        }
    }

    fn matches_kind(string: &str, kind: &str) -> bool {
        string.to_ascii_lowercase().contains(kind)
    }
}
