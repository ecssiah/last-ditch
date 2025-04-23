#[derive(Clone, Debug)]
pub enum Kind {
    Lion,
    Eagle,
    Wolf,
    Horse,
}

impl Kind {
    pub fn all() -> [Kind; 4] {
        [Kind::Lion, Kind::Eagle, Kind::Wolf, Kind::Horse]
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            Kind::Lion => [0.70, 0.55, 0.85, 1.0],
            Kind::Eagle => [0.65, 0.70, 0.80, 1.0],
            Kind::Wolf => [0.85, 0.35, 0.35, 1.0],
            Kind::Horse => [0.988, 0.863, 0.592, 1.0],
        }
    }
}
