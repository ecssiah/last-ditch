use crate::simulation::state::{
    population::{identity::Identity, kinematic::Kinematic, sight::Sight, spatial::Spatial},
    world::block,
};
use ultraviolet::Vec3;

pub struct Person {
    pub person_id: u64,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub sight: Sight,
    pub selected_block_kind: block::Kind,
}

impl Person {
    pub fn new(person_id: u64) -> Self {
        let identity = Identity::new();
        let spatial = Spatial::new();
        let kinematic = Kinematic::new();
        let sight = Sight::new();
        let selected_block_kind = block::Kind::Engraved1;

        Self {
            person_id,
            identity,
            spatial,
            kinematic,
            sight,
            selected_block_kind,
        }
    }

    pub fn set_world_position(world_position: Vec3, person: &mut Self) {
        Spatial::set_world_position(world_position, &mut person.spatial);

        Sight::set_world_position(
            world_position + person.sight.relative_position,
            &mut person.sight,
        );
    }

    pub fn set_rotation(rotation_xy: f32, rotation_yz: f32, person: &mut Self) {
        Spatial::set_rotation(rotation_xy, &mut person.spatial);

        Sight::set_rotation(rotation_xy, rotation_yz, &mut person.sight);
    }
}
