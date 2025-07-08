#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Menu,
    Load,
    Simulate,
    Shutdown,
}
