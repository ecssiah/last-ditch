#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    None,
    Eagle,
    Lion,
    Horse,
    Wolf,
}

impl Kind {
    pub const ALL: [Self; 5] = [Self::None, Self::Eagle, Self::Lion, Self::Horse, Self::Wolf];
}
