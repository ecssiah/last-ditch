#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Init,
    Load,
    Run,
    Done,
}
