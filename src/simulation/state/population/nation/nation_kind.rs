#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NationKind {
    Lion,
    Eagle,
    Horse,
    Wolf,
}

impl NationKind {
    pub const ALL: [Self; 4] = [Self::Lion, Self::Eagle, Self::Horse, Self::Wolf];
}
