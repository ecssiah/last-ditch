#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Solid,
    Trigger,
    StairsNorth,
    StairsWest,
    StairsSouth,
    StairsEast,
    Ladder,
}
