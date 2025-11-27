pub mod move_data;

pub use move_data::MoveData;

#[derive(Clone, Copy, Debug)]
pub enum Act {
    Move(MoveData),
    PlaceBlock,
    Jump,
    RemoveBlock,
}
