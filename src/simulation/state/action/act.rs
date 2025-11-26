pub mod move_data;
pub mod place_block_data;

pub use move_data::MoveData;
pub use place_block_data::PlaceBlockData;

#[derive(Clone, Copy, Debug)]
pub enum Act {
    Start,
    Quit,
    Exit,
    Debug,
    Jump,
    Move(MoveData),
    PlaceBlock(PlaceBlockData),
    RemoveBlock,
    Test1,
    Test2,
    Test3,
    Test4,
}
