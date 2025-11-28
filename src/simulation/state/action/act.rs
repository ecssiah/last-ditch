pub mod move_data;
pub mod rotate_data;

pub use move_data::MoveData;
pub use rotate_data::RotateData;

#[derive(Clone, Copy, Debug)]
pub enum Act {
    Rotate(RotateData),
    Move(MoveData),
    Jump,
    PlaceBlock,
    RemoveBlock,
}
