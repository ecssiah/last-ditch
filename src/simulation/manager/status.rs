#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Init,
    Load,
    Run,
    Pause,
    Done,
}
