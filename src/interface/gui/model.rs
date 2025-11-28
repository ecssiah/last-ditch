#[derive(Default)]
pub struct Model {
    pub seed_input_string: String,
    pub info_message_vec: Vec<String>,
}

impl Model {
    pub fn new() -> Self {
        let seed_input_string = String::from("813");
        let info_message_vec = Vec::new();

        Self {
            seed_input_string,
            info_message_vec,
        }
    }
}
