pub mod admin_view;
pub mod chunk_view;
pub mod entity_view;
pub mod population_view;
pub mod time_view;
pub mod world_view;

pub use admin_view::AdminView;
pub use chunk_view::ChunkView;
pub use entity_view::EntityView;
pub use population_view::PopulationView;
pub use time_view::TimeView;
pub use world_view::WorldView;

use crate::simulation::population::entity;

#[derive(Clone)]
pub struct View {
    pub entity_id: entity::ID,
    pub admin_view: AdminView,
    pub time_view: TimeView,
    pub population_view: PopulationView,
    pub world_view: WorldView,
}
