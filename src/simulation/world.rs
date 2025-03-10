#[derive(Debug)]
pub struct World {
    pub active: bool,
    pub update_window: u32,
    pub seed: u64,
    pub time: f32,
}
