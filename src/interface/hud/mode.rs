//! Defines data structure for each HUD Mode

#[derive(Clone, Default)]
pub struct MenuData {
    pub message: String,
}

#[derive(Clone, Default)]
pub struct LoadData {
    pub message: String,
}

#[derive(Clone, Default)]
pub struct SimulateData {
    pub message: String,
}

#[derive(Clone, Default)]
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

impl Default for Mode {
    fn default() -> Self {
        Self::Menu(MenuData::default())
    }
}
