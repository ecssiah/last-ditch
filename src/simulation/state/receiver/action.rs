pub mod move_data;

pub use move_data::MoveData;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Start,
    Quit,
    Exit,
    Debug,
    Jump,
    Move(MoveData),
    Test1,
    Test2,
    Test3,
    Test4,
}
