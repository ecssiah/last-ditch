#[derive(Default)]
pub struct GUIModel {
    pub seed_input_string: String,
    pub info_message_vec: Vec<String>,
}

impl GUIModel {
    pub fn new() -> Self {
        let seed_input_string = String::from("813");
        let info_message_vec = Vec::new();

        Self {
            seed_input_string,
            info_message_vec,
        }
    }
}
