#[derive(Default)]
pub struct RunContent {
    pub main_window_active: bool,
    pub info_message_vec: Vec<String>,
}

impl RunContent {
    pub fn new() -> Self {
        let main_window_active = false;
        let info_message_vec = Vec::new();

        Self {
            main_window_active,
            info_message_vec,
        }
    }
}
