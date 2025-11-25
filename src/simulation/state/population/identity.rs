use crate::simulation::state::population::{nation, role};

#[derive(Debug)]
pub struct Identity {
    pub role: role::Role,
    pub nation_kind: nation::Kind,
}
