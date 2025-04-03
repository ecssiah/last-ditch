use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Light {
    pub received: u8,
    pub emitted: u8,
}
