use crate::simulation::state::physics::collider::Collider;

pub struct LadderBody {
    pub core: Collider,
}

impl LadderBody {
    pub fn new() -> Self {
        Self {
            core: Collider::default(),
        }
    }
}
