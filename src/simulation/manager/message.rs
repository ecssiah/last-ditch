pub mod movement_input_data;
pub mod rotation_input_data;
pub mod seed_data;

pub use movement_input_data::MovementInputData;
pub use rotation_input_data::RotationInputData;
pub use seed_data::SeedData;

#[derive(Debug)]
pub enum Message {
    Interact1,
    Interact2,
    MovementInput(MovementInputData),
    RotatationInput(RotationInputData),
    JumpInput,
    SetSeed(SeedData),
    GenerateWorld,
    GeneratePopulation,
    Quit,
    Debug,
    Option1,
    Option2,
    Option3,
    Option4,
}
