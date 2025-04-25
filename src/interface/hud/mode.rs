use crate::interface::hud::{load_data::LoadData, simulate_data::SimulateData};

pub enum Mode {
    Load(LoadData),
    Simulate(SimulateData),
}
