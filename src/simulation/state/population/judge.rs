pub mod id;

pub use id::ID;

use crate::simulation::{
    consts::*,
    state::{
        population::entity::{self, nation, Entity, Info, Kinematic, Sense, Spatial},
        receiver::action::{JumpAction, MovementData},
        world::{sector, World},
    },
};
use ultraviolet::{Rotor3, Vec3};

pub struct Judge {
    pub judge_id: ID,
    pub entity: Entity,
}

impl Judge {
    pub fn new() -> Self {
        let judge_id = ID::allocate();

        let info = Info {
            sector_id: sector::ID(0),
            sector_updated: false,
            entity_kind: entity::Kind::Judge,
            nation_kind: nation::Kind::Eagle,
        };

        let entity = Entity {
            info,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            sense: Sense::new(),
        };

        Self { judge_id, entity }
    }

    pub fn tick(world: &World, judge: &mut Judge) {
        Entity::tick(world, &mut judge.entity);
    }

    pub fn apply_movement_data(movement_data: &MovementData, judge: &mut Judge) {
        if movement_data.rotation.x.abs() > 1e-6 || movement_data.rotation.y.abs() > 1e-6 {
            judge.entity.spatial.yaw += movement_data.rotation.x;
            judge.entity.spatial.pitch += movement_data.rotation.y;

            judge.entity.spatial.pitch = judge
                .entity
                .spatial
                .pitch
                .clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

            let yaw_rotor = Rotor3::from_rotation_xy(judge.entity.spatial.yaw);
            let pitch_rotor = Rotor3::from_rotation_yz(-judge.entity.spatial.pitch);

            judge.entity.spatial.rotor = yaw_rotor * pitch_rotor;
        }

        if movement_data.direction.mag_sq() > 1e-6 {
            let yaw_rotor = Rotor3::from_rotation_xy(judge.entity.spatial.yaw);

            let local_velocity =
                Vec3::new(movement_data.direction.x, movement_data.direction.y, 0.0)
                    * Vec3::new(JUDGE_SPEED_X, JUDGE_SPEED_Y, 0.0);

            let velocity = yaw_rotor * local_velocity;

            judge.entity.kinematic.velocity.x = velocity.x;
            judge.entity.kinematic.velocity.y = velocity.y;
        } else {
            judge.entity.kinematic.velocity.x = 0.0;
            judge.entity.kinematic.velocity.y = 0.0;
        }
    }

    pub fn apply_jump_action(jump_action: &JumpAction, judge: &mut Judge) {
        if let JumpAction::Start = jump_action {
            judge.entity.kinematic.velocity.z = JUDGE_SPEED_Z;
        }
    }
}
