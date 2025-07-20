pub mod data;
pub mod id;
pub mod kind;
pub mod priority;
pub mod stage;
pub mod store;

pub use id::ID;
pub use kind::Kind;
pub use priority::Priority;
pub use stage::Stage;
pub use store::Store;

use crate::simulation::state::population::entity::decision::plan;
use glam::IVec3;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Plan {
    pub id: ID,
    pub kind: Kind,
    pub priority: Priority,
}

impl Plan {
    pub fn new(kind: Kind, priority: Priority) -> Self {
        Self {
            id: ID::allocate(),
            kind,
            priority,
        }
    }

    pub fn create_idle_plan(tick_duration: u32) -> (Self, plan::data::IdleData) {
        let idle_plan = Self {
            id: ID::allocate(),
            kind: Kind::Idle,
            priority: Priority::Medium,
        };

        let idle_data = plan::data::IdleData::new(tick_duration);

        (idle_plan, idle_data)
    }

    pub fn create_travel_plan(target_position: IVec3) -> (Self, plan::data::TravelData) {
        let travel_plan = Self {
            id: ID::allocate(),
            kind: Kind::Travel,
            priority: Priority::Medium,
        };

        let travel_data = plan::data::TravelData::new(target_position);

        (travel_plan, travel_data)
    }
}

impl PartialEq for Plan {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Plan {}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Plan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
