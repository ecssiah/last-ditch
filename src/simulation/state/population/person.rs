use crate::simulation::state::{
    body::SimpleBody,
    population::{identity::Identity, kinematic::Kinematic, sight::Sight, transform::Transform},
    world::block,
};
use ultraviolet::Vec3;

pub struct Person {
    pub person_id: u64,
    pub identity: Identity,
    pub transform: Transform,
    pub kinematic: Kinematic,
    pub body: SimpleBody,
    pub sight: Sight,
    pub selected_block_kind: block::Kind,
}

impl Person {
    pub fn new(person_id: u64) -> Self {
        let identity = Identity::default();
        let transform = Transform::default();
        let kinematic = Kinematic::default();
        let body = SimpleBody::default();
        let sight = Sight::default();

        let selected_block_kind = block::Kind::Engraved1;

        Self {
            person_id,
            identity,
            transform,
            kinematic,
            body,
            sight,
            selected_block_kind,
        }
    }

    pub fn set_world_position(world_position: Vec3, person: &mut Self) {
        Transform::set_world_position(world_position, &mut person.transform);
        SimpleBody::set_world_position(world_position, &mut person.body);
        Sight::set_world_position(world_position, &mut person.sight);
    }

    pub fn set_size(size: Vec3, person: &mut Self) {
        Transform::set_size(size, &mut person.transform);
        SimpleBody::set_size(size, &mut person.body);
    }

    pub fn set_rotation(rotation_xy: f32, rotation_yz: f32, person: &mut Self) {
        Transform::set_rotation(rotation_xy, &mut person.transform);
        Sight::set_rotation(rotation_xy, rotation_yz, &mut person.sight);
    }
}
