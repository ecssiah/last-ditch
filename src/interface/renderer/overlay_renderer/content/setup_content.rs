#[derive(Default)]
pub struct SetupContent {
    pub progress: f32,
    pub loading_string: String,
}

impl SetupContent {
    pub fn new() -> Self {
        let progress = 0.0;
        let loading_string = String::new();

        Self {
            progress,
            loading_string,
        }
    }
}
