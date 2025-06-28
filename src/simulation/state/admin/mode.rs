#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Menu,
    Load,
    Simulate,
    Shutdown,
    Exit,
}
