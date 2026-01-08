#[derive(Default)]
pub struct RunModel {
    pub main_window_active: bool,
    pub info_message_vec: Vec<String>,
}

impl RunModel {
    pub fn new() -> Self {
        let main_window_active = false;
        let info_message_vec = Vec::new();

        Self {
            main_window_active,
            info_message_vec,
        }
    }
}
