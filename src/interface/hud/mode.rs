use crate::interface::hud::data::{LoadData, SimulateData};

pub enum Mode {
    Load(LoadData),
    Simulate(SimulateData),
}
