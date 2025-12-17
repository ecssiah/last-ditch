use crate::simulation::state::physics::collider::Collider;

pub struct StairBody {
    pub core: Collider,
}

impl StairBody {
    pub fn new() -> Self {
        Self {
            core: Collider::default(),
        }
    }
}
