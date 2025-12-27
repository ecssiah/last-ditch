use crate::simulation::state::physics::collider;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Hit {
    pub collider_kind: collider::Kind,
    pub contact_point: Vec3,
    pub normal: Vec3,
}
