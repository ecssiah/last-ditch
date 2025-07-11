pub mod admin_view;
pub mod agent_view;
pub mod chunk_view;
pub mod judge_view;
pub mod population_view;
pub mod time_view;
pub mod world_view;

pub use admin_view::AdminView;
pub use agent_view::AgentView;
pub use chunk_view::ChunkView;
pub use judge_view::JudgeView;
pub use population_view::PopulationView;
pub use time_view::TimeView;
pub use world_view::WorldView;

use crate::simulation::state::population::entity;

#[derive(Clone)]
pub struct View {
    pub entity_id: entity::ID,
    pub admin_view: AdminView,
    pub time_view: TimeView,
    pub population_view: PopulationView,
    pub world_view: WorldView,
}

impl View {
    pub fn new() -> Self {
        Self {
            entity_id: entity::ID::MAX,
            admin_view: AdminView::new(),
            time_view: TimeView::new(),
            population_view: PopulationView::new(),
            world_view: WorldView::new(),
        }
    }
}
