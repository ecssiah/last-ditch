#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Init,
    Load,
    Run,
    Done,
}
