#[derive(Clone, Debug, Default)]
pub struct StatePair<T> {
    pub current: T,
    pub next: T,
}

impl<T> StatePair<T> {
    pub fn new(current: T, next: T) -> Self {
        Self { current, next }
    }
}
