#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Eagle,
    Lion,
    Horse,
    Wolf,
}

impl Kind {
    pub const ALL: [Kind; 4] = [Kind::Eagle, Kind::Lion, Kind::Horse, Kind::Wolf];
}
