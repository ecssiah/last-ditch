pub mod move_input_data;
pub mod rotate_input_data;
pub mod seed_data;

pub use move_input_data::MoveInputData;
pub use rotate_input_data::RotateInputData;
pub use seed_data::SeedData;

#[derive(Debug)]
pub enum Message {
    Interact1,
    Interact2,
    MoveInput(MoveInputData),
    RotateInput(RotateInputData),
    JumpInput,
    SetSeed(SeedData),
    Generate,
    Quit,
    Debug,
    Option1,
    Option2,
    Option3,
    Option4,
}
