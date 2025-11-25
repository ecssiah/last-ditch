pub mod id;

pub use id::ID;

use crate::simulation::{
    constants::*,
    state::{World, population::{
        self, identity::Identity, kinematic::Kinematic, nation, sight::Sight, spatial::Spatial,
    }, receiver::action::{JumpAction, MovementData}},
};
use ultraviolet::{Rotor3, Vec3};

pub struct Judge {
    pub id: ID,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub sight: Sight,
}

impl Judge {
    pub fn new() -> Self {
        let id = ID::allocate();

        let identity = Identity {
            role: population::Role::Judge,
            nation_kind: nation::Kind::Eagle,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();
        let sight = Sight::new();

        Self {
            id,
            identity,
            spatial,
            kinematic,
            sight,
        }
    }

    pub fn tick(world: &World, judge: &mut Judge) {
        Spatial::update_sector_id(&world.grid, &mut judge.spatial);
    }

    pub fn apply_movement_data(movement_data: &MovementData, judge: &mut Judge) {
        if movement_data.rotation.x.abs() > 1e-6 || movement_data.rotation.y.abs() > 1e-6 {
            judge.spatial.yaw += movement_data.rotation.x;
            judge.spatial.pitch += movement_data.rotation.y;

            judge.spatial.pitch = judge.spatial.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

            let yaw_rotor = Rotor3::from_rotation_xy(judge.spatial.yaw);
            let pitch_rotor = Rotor3::from_rotation_yz(-judge.spatial.pitch);

            judge.spatial.rotor = yaw_rotor * pitch_rotor;
        }

        if movement_data.direction.mag_sq() > 1e-6 {
            let yaw_rotor = Rotor3::from_rotation_xy(judge.spatial.yaw);

            let local_velocity =
                Vec3::new(movement_data.direction.x, movement_data.direction.y, 0.0)
                    * Vec3::new(JUDGE_DEFAULT_SPEED_X, JUDGE_DEFAULT_SPEED_Y, 0.0);

            let velocity = yaw_rotor * local_velocity;

            judge.kinematic.velocity.x = velocity.x;
            judge.kinematic.velocity.y = velocity.y;
        } else {
            judge.kinematic.velocity.x = 0.0;
            judge.kinematic.velocity.y = 0.0;
        }
    }

    pub fn apply_jump_action(jump_action: &JumpAction, judge: &mut Judge) {
        if let JumpAction::Start = jump_action {
            judge.kinematic.velocity.z = JUDGE_DEFAULT_SPEED_Z;
        }
    }
}
