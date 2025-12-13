#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Kind {
    Center,
    CenterHall,
    OuterHall,
    CornerHall,
    LowerRoom,
    UpperRoom,
    UpperArea,
}
