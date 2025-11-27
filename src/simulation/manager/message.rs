use crate::simulation::manager::message::{move_data::MoveData, rotate_data::RotateData};

pub mod move_data;
pub mod rotate_data;

#[derive(Debug)]
pub enum Message {
    Interact1,
    Interact2,
    Move(MoveData),
    Rotate(RotateData),
    Jump,
    Debug,
    Start,
    Quit,
    Option1,
    Option2,
    Option3,
    Option4,
}
