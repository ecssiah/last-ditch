#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Load,
    Simulate,
    Shutdown,
    Exit,
}
