#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    None,
    Lion,
    Eagle,
    Horse,
    Wolf,
}

impl Kind {
    pub const ALL: [Self; 5] = [Self::None, Self::Lion, Self::Eagle, Self::Horse, Self::Wolf];
}
