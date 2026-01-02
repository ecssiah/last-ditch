use crate::simulation::state::physics::collider::ColliderKind;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Hit {
    pub collider_kind: ColliderKind,
    pub contact_point: Vec3,
}
