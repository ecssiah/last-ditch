#[derive(Default)]
pub struct MenuContent {
    pub seed_input_string: String,
}

impl MenuContent {
    pub fn new() -> Self {
        let seed_input_string = String::from("813");

        Self { seed_input_string }
    }
}
