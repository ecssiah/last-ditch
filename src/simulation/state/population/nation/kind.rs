#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Eagle,
    Lion,
    Horse,
    Wolf,
}

impl Kind {
    pub const ALL: [Self; 4] = [Self::Eagle, Self::Lion, Self::Horse, Self::Wolf];
}
