use ultraviolet::Vec3;

use crate::simulation::{
    constants::{
        JUDGE_DEFAULT_JUMP_SPEED, JUDGE_DEFAULT_SIZE_X, JUDGE_DEFAULT_SIZE_Y, JUDGE_DEFAULT_SIZE_Z,
        JUDGE_DEFAULT_SPEED,
    },
    state::{
        population::{
            self, identity::Identity, kinematic::Kinematic, nation, sight::Sight, spatial::Spatial,
        },
        world::block,
        World,
    },
};

pub struct Judge {
    pub entity_id: u64,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub sight: Sight,
    pub selected_block_kind: block::Kind,
}

impl Judge {
    pub fn new(entity_id: u64) -> Self {
        let identity = Identity {
            role: population::Role::Judge,
            nation_kind: nation::Kind::Eagle,
        };

        let mut spatial = Spatial::new();

        Spatial::set_size(
            Vec3::new(
                JUDGE_DEFAULT_SIZE_X,
                JUDGE_DEFAULT_SIZE_Y,
                JUDGE_DEFAULT_SIZE_Z,
            ),
            &mut spatial,
        );

        let kinematic = Kinematic {
            speed: JUDGE_DEFAULT_SPEED,
            jump_speed: JUDGE_DEFAULT_JUMP_SPEED,
            velocity: Vec3::zero(),
            flying: false,
        };

        let mut sight = Sight::new();
        sight.relative_position = Vec3::new(0.0, 0.0, 0.9 * spatial.size.z);

        let selected_block_kind = block::Kind::Engraved1;

        Self {
            entity_id,
            identity,
            spatial,
            kinematic,
            sight,
            selected_block_kind,
        }
    }

    pub fn set_world_position(world_position: Vec3, judge: &mut Self) {
        Spatial::set_world_position(world_position, &mut judge.spatial);

        Sight::set_world_position(
            world_position + judge.sight.relative_position,
            &mut judge.sight,
        );
    }

    pub fn set_rotation(rotation_xy: f32, rotation_yz: f32, judge: &mut Self) {
        Spatial::set_rotation(rotation_xy, &mut judge.spatial);

        Sight::set_rotation(rotation_xy, rotation_yz, &mut judge.sight);
    }

    pub fn tick(_world: &World, judge: &mut Self) {
        Spatial::update_sector_id(&mut judge.spatial);
    }
}
