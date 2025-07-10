use crate::simulation::state::time::Tick;
use glam::IVec3;

pub enum Kind {
    Idle { duration: Tick },
    Move { target_position: IVec3 },
}
