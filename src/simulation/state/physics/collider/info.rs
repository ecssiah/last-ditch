use crate::simulation::state::physics::collider::{self, Collider};

pub struct Info {
    pub collider: Collider,
    pub collider_kind: collider::Kind,
    pub collider_owner: collider::Owner,
}