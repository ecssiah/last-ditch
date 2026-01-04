use std::time::Instant;

pub struct Timestep {
    pub start_instant: Instant,
    pub next_instant: Instant,
    pub ticks_total: u32,
    pub ticks_frame: u32,
}

impl Timestep {
    pub fn new() -> Self {
        let start_instant = Instant::now();
        let next_instant = Instant::now();
        let ticks_total = 0;
        let ticks_frame = 0;

        Self {
            start_instant,
            next_instant,
            ticks_total,
            ticks_frame,
        }
    }
}
