#[derive(Default)]
pub struct Model {
    pub info_message_vec: Vec<String>,
}

impl Model {
    pub fn new() -> Self {
        let info_message_vec = Vec::new();

        Self { info_message_vec }
    }
}
