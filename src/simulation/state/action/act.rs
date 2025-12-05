pub mod jump_data;
pub mod move_data;
pub mod place_block_data;
pub mod remove_block_data;
pub mod rotate_data;

pub use jump_data::JumpData;
pub use move_data::MoveData;
pub use place_block_data::PlaceBlockData;
pub use remove_block_data::RemoveBlockData;
pub use rotate_data::RotateData;

#[derive(Clone, Copy, Debug)]
pub enum Act {
    Rotate(RotateData),
    Move(MoveData),
    Jump(JumpData),
    PlaceBlock(PlaceBlockData),
    RemoveBlock(RemoveBlockData),
}
