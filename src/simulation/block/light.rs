use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Light {
    pub received: u8,
    pub emitted: u8,
}

impl Light {
    pub fn new() -> Light {
        let light = Light {
            received: 0,
            emitted: 0,
        };

        light
    }
}
