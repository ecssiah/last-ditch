#[derive(Default)]
pub struct MenuModel {
    pub seed_input_string: String,
}

impl MenuModel {
    pub fn new() -> Self {
        let seed_input_string = String::from("813");

        Self { seed_input_string }
    }
}
