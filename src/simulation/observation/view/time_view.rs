use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub instant: Instant,
}

impl Default for TimeView {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}
