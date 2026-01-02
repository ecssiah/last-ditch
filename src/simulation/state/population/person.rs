use crate::simulation::state::{
    physics::body::Body,
    population::{identity::Identity, motion::Motion, sight::Sight, transform::Transform},
    world::block::BlockKind,
};
use ultraviolet::Vec3;

pub struct Person {
    pub person_id: u64,
    pub identity: Identity,
    pub transform: Transform,
    pub motion: Motion,
    pub body: Body,
    pub sight: Sight,
    pub selected_block_kind: BlockKind,
}

impl Person {
    pub fn new(person_id: u64) -> Self {
        let identity = Identity::default();
        let transform = Transform::default();
        let motion = Motion::default();
        let body = Body::default();
        let sight = Sight::default();

        let selected_block_kind = BlockKind::Carved1;

        Self {
            person_id,
            identity,
            transform,
            motion,
            body,
            sight,
            selected_block_kind,
        }
    }

    pub fn set_world_position(world_position: Vec3, person: &mut Self) {
        Transform::set_world_position(world_position, &mut person.transform);

        Body::set_world_position(world_position, &mut person.body);

        Sight::set_world_position(
            world_position + person.sight.local_position,
            &mut person.sight,
        );
    }

    pub fn set_rotation(rotation_xy: f32, rotation_yz: f32, person: &mut Self) {
        Transform::set_rotation(rotation_xy, &mut person.transform);
        Sight::set_rotation(rotation_xy, rotation_yz, &mut person.sight);
    }

    pub fn set_velocity(velocity: Vec3, person: &mut Self) {
        person.motion.velocity = velocity;
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Person ID: {}", self.person_id)
    }
}
