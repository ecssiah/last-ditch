use crate::simulation::{
    observation::view::entity_view::EntityView, population::entity, time::Tick,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PopulationView {
    pub tick: Tick,
    pub entity_views: HashMap<entity::ID, EntityView>,
}
