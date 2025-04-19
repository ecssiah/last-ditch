#[derive(Clone, Debug)]
pub struct StatePair<T> {
    pub current: T,
    pub next: T,
}

impl<T> StatePair<T> {
    pub fn new(current: T, next: T) -> StatePair<T> {
        let state_pair = StatePair { current, next };

        state_pair
    }
}
