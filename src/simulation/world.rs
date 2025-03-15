#[derive(Debug)]
pub struct World {
    pub active: bool,
    pub seed: u64,
    pub time: f64,
    pub ticks: u64,
    pub last_update: u64,
}