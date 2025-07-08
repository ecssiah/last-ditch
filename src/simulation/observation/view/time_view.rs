use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub instant: Instant,
}

impl TimeView {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}
