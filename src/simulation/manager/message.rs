pub mod generate_data;
pub mod movement_input_data;
pub mod rotation_input_data;

pub use generate_data::GenerateData;
pub use movement_input_data::MovementInputData;
pub use rotation_input_data::RotationInputData;

#[derive(Debug)]
pub enum Message {
    Interact1,
    Interact2,
    MovementInput(MovementInputData),
    RotatationInput(RotationInputData),
    JumpInput,
    Generate(GenerateData),
    Quit,
    Debug,
    Option1,
    Option2,
    Option3,
    Option4,
}
