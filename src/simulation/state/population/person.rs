use crate::simulation::state::{
    body::person_body::PersonBody, population::{identity::Identity, kinematic::Kinematic, sight::Sight, transform::Transform}, world::block
};
use ultraviolet::Vec3;

pub struct Person {
    pub person_id: u64,
    pub identity: Identity,
    pub transform: Transform,
    pub kinematic: Kinematic,
    pub person_body: PersonBody,
    pub sight: Sight,
    pub selected_block_kind: block::Kind,
}

impl Person {
    pub fn new(person_id: u64) -> Self {
        let identity = Identity::new();
        let transform = Transform::new();
        let kinematic = Kinematic::new();
        let person_body = PersonBody::new();
        let sight = Sight::new();
        let selected_block_kind = block::Kind::Engraved1;

        Self {
            person_id,
            identity,
            transform,
            kinematic,
            person_body,
            sight,
            selected_block_kind,
        }
    }

    pub fn set_world_position(world_position: Vec3, person: &mut Self) {
        Transform::set_world_position(world_position, &mut person.transform);
        PersonBody::set_world_position(world_position, &mut person.person_body);

        Sight::set_world_position(
            world_position + person.sight.relative_position,
            &mut person.sight,
        );
    }

    pub fn set_size(size: Vec3, person: &mut Self) {
        Transform::set_size(size, &mut person.transform);
        PersonBody::set_size(size, &mut person.person_body);
    }

    pub fn set_rotation(rotation_xy: f32, rotation_yz: f32, person: &mut Self) {
        Transform::set_rotation(rotation_xy, &mut person.transform);

        Sight::set_rotation(rotation_xy, rotation_yz, &mut person.sight);
    }
}
