pub mod move_data;
pub mod rotate_data;

pub use move_data::MoveData;
pub use rotate_data::RotateData;

#[derive(Debug)]
pub enum Message {
    Interact1,
    Interact2,
    Move(MoveData),
    Rotate(RotateData),
    Jump,
    Debug,
    Generate,
    Quit,
    Option1,
    Option2,
    Option3,
    Option4,
}
