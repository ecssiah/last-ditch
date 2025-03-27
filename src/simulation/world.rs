pub type Tick = u64;

pub struct World {
    pub active: bool,
    pub seed: u64,
    pub time: f64,
    pub ticks: Tick,
    pub last_update: Tick,
}
