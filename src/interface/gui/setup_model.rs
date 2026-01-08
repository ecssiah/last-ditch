#[derive(Default)]
pub struct SetupModel {
    pub progress: f32,
}

impl SetupModel {
    pub fn new() -> Self {
        let progress = 0.0;

        Self {
            progress,
        }
    }
}
