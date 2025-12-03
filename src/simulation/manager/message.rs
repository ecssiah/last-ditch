pub mod generate_data;
pub mod move_data;
pub mod rotate_data;

pub use generate_data::GenerateData;
pub use move_data::MoveData;
pub use rotate_data::RotateData;

#[derive(Debug)]
pub enum Message {
    Interact1,
    Interact2,
    Move(MoveData),
    Rotate(RotateData),
    Jump,
    Generate(GenerateData),
    Quit,
    Debug,
    Option1,
    Option2,
    Option3,
    Option4,
}
