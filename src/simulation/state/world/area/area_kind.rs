#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum AreaKind {
    Center,
    CenterHall,
    OuterHall,
    CornerHall,
    LowerRoom,
    UpperRoom,
    UpperArea,
}
