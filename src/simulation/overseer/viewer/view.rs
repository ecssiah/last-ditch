pub mod leadership_view;
pub mod overseer_view;
pub mod person_view;
pub mod population_view;
pub mod sector_view;
pub mod time_view;
pub mod world_view;

pub use leadership_view::LeadershipView;
pub use overseer_view::OverseerView;
pub use person_view::PersonView;
pub use population_view::PopulationView;
pub use sector_view::SectorView;
pub use time_view::TimeView;
pub use world_view::WorldView;

#[derive(Clone, Default)]
pub struct View {
    pub overseer_view: OverseerView,
    pub population_view: PopulationView,
    pub world_view: WorldView,
}
