#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Load,
    Simulate,
    Shutdown,
    Exit,
}
