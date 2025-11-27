#[derive(PartialEq, Eq)]
pub enum Status {
    Init,
    Load,
    Run,
    Pause,
    Done,
}