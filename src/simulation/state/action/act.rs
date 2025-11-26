pub mod move_data;
pub mod set_block_data;

pub use move_data::MoveData;
pub use set_block_data::SetBlockData;

#[derive(Clone, Copy, Debug)]
pub enum Act {
    Start,
    Quit,
    Exit,
    Debug,
    Jump,
    Move(MoveData),
    SetBlock(SetBlockData),
    Test1,
    Test2,
    Test3,
    Test4,
}
