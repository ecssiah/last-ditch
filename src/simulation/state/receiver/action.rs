use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MoveData {
    pub direction: Vec3,
    pub rotation: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Start,
    Quit,
    Exit,
    ToggleDebug,
    Jump,
    Move(MoveData),
    Test1,
    Test2,
    Test3,
    Test4,
}
