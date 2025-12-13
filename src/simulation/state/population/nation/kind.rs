#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    None,
    Wolf,
    Eagle,
    Lion,
    Horse,
}

impl Kind {
    pub const ALL: [Self; 5] = [Self::None, Self::Wolf, Self::Eagle, Self::Lion, Self::Horse];
}
