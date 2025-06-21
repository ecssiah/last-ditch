#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Load,
    Simulate,
    Shutdown,
    Exit,
}
