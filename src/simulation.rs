pub mod action;

pub struct Simulation {
    time: u64,
}

impl Simulation {
    pub async fn new() -> Simulation {
        Simulation { time: 0 }
    }
}
