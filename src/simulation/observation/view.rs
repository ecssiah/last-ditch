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

#[derive(Clone, Default)]
pub struct View {
    pub judge_id: entity::ID,
    pub admin_view: AdminView,
    pub time_view: TimeView,
    pub population_view: PopulationView,
    pub world_view: WorldView,
}
