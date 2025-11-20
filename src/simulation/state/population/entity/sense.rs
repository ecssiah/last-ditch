pub mod hearing;
pub mod sight;
pub mod touch;

pub use hearing::Hearing;
pub use sight::Sight;
pub use touch::Touch;

#[derive(Clone, Copy, Debug, Default)]
pub struct Sense {
    pub hearing: Hearing,
    pub touch: Touch,
    pub sight: Sight,
}

impl Sense {
    pub fn new() -> Self {
        let hearing = Hearing::new();
        let touch = Touch::new();
        let sight = Sight::new();

        Self {
            hearing,
            touch,
            sight,
        }
    }
}
