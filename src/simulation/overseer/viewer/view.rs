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

#[derive(Clone)]
pub struct View {
    pub overseer_view: OverseerView,
    pub population_view: PopulationView,
    pub world_view: WorldView,
}

impl View {
    pub fn new() -> Self {
        Self {
            overseer_view: OverseerView::default(),
            population_view: PopulationView::default(),
            world_view: WorldView::default(),
        }
    }
}
