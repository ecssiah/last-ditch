//! Defines data structure for each HUD Mode

#[derive(Clone)]
pub struct MenuData {
    pub message: String,
}

#[derive(Clone)]
pub struct LoadData {
    pub message: String,
}

#[derive(Clone)]
pub struct SimulateData {
    pub message: String,
}

#[derive(Clone)]
pub struct ShutdownData {
    pub message: String,
}

#[derive(Clone)]
pub enum Mode {
    Menu(MenuData),
    Load(LoadData),
    Simulate(SimulateData),
    Shutdown(ShutdownData),
}
