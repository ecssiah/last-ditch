#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Lion,
    Eagle,
    Horse,
    Wolf,
}

impl Kind {
    pub const ALL: [Self; 4] = [Self::Lion, Self::Eagle, Self::Horse, Self::Wolf];
}
