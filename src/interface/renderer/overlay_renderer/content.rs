use crate::interface::renderer::overlay_renderer::content::{
    menu_content::MenuContent, run_content::RunContent, setup_content::SetupContent,
};

pub mod menu_content;
pub mod run_content;
pub mod setup_content;

#[derive(Default)]
pub struct Content {
    pub setup_content: SetupContent,
    pub menu_content: MenuContent,
    pub run_content: RunContent,
}

impl Content {
    pub fn new() -> Self {
        let setup_content = SetupContent::new();
        let menu_content = MenuContent::new();
        let run_content = RunContent::new();

        Self {
            setup_content,
            menu_content,
            run_content,
        }
    }
}
