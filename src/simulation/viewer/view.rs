pub mod agent_view;
pub mod block_view;
pub mod face_view;
pub mod judge_view;
pub mod population_view;
pub mod sector_view;
pub mod world_view;

pub use agent_view::AgentView;
pub use block_view::BlockView;
pub use face_view::FaceView;
pub use judge_view::JudgeView;
pub use population_view::PopulationView;
pub use sector_view::SectorView;
pub use world_view::WorldView;

#[derive(Clone)]
pub struct View {
    pub population_view: PopulationView,
    pub world_view: WorldView,
}

impl View {
    pub fn new() -> Self {
        Self {
            population_view: PopulationView::new(),
            world_view: WorldView::new(),
        }
    }
}
