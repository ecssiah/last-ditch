use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{
            entity::{self, Detection, Info, Kinematic, Spatial},
            nation,
        },
        receiver::action::{JumpAction, MovementData},
        world::{sector, World},
    },
};
use ultraviolet::{Rotor3, Vec3};

pub struct Judge {
    pub info: entity::Info,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
}

impl Judge {
    pub fn new() -> Self {
        let info = entity::Info {
            entity_id: entity::ID::allocate(),
            sector_id: sector::ID(0),
            sector_updated: false,
            entity_kind: entity::Kind::Judge,
            nation_kind: nation::Kind::Eagle,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();
        let detection = Detection::new();

        Self {
            info,
            spatial,
            kinematic,
            detection,
        }
    }

    pub fn tick(world: &World, judge: &mut Judge) {
        Info::tick(world, &judge.spatial, &mut judge.info);
    }

    pub fn set_world_position(
        world_position: Vec3,
        spatial: &mut Spatial,
        detection: &mut Detection,
    ) {
        spatial.world_position = world_position;

        Detection::set_world_position(world_position, detection);
    }

    pub fn set_size(size: Vec3, spatial: &mut Spatial, detection: &mut Detection) {
        spatial.size = size;

        detection.body = AABB::new(detection.body.center(), size);
    }

    pub fn set_rotation(yaw: f32, pitch: f32, spatial: &mut Spatial, kinematic: &mut Kinematic) {
        spatial.yaw = yaw.to_radians();
        spatial.pitch = pitch.to_radians();

        spatial.pitch = spatial.pitch.clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

        spatial.rotor = Rotor3::from_euler_angles(0.0, 0.0, -spatial.yaw)
            * Rotor3::from_euler_angles(0.0, spatial.pitch, 0.0);

        let velocity_xy = Vec3::new(kinematic.velocity.x, kinematic.velocity.z, 0.0);
        let speed = velocity_xy.mag_sq();

        if speed > 1e-12 {
            let new_velocity_xy = Spatial::forward(spatial) * speed;

            kinematic.velocity.x = new_velocity_xy.x;
            kinematic.velocity.y = new_velocity_xy.y;
        }
    }

    pub fn apply_movement_data(movement_data: &MovementData, judge: &mut Judge) {
        if movement_data.rotation.x.abs() > 1e-6 || movement_data.rotation.y.abs() > 1e-6 {
            judge.spatial.yaw += movement_data.rotation.x;
            judge.spatial.pitch += movement_data.rotation.y;

            judge.spatial.pitch = judge
                .spatial
                .pitch
                .clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

            let yaw_rotor = Rotor3::from_rotation_xy(judge.spatial.yaw);
            let pitch_rotor = Rotor3::from_rotation_yz(-judge.spatial.pitch);

            judge.spatial.rotor = yaw_rotor * pitch_rotor;
        }

        if movement_data.direction.mag_sq() > 1e-6 {
            let yaw_rotor = Rotor3::from_rotation_xy(judge.spatial.yaw);

            let local_velocity =
                Vec3::new(movement_data.direction.x, movement_data.direction.y, 0.0) * 
                Vec3::new(JUDGE_SPEED_X, JUDGE_SPEED_Y, 0.0);

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
            judge.kinematic.velocity.z = JUDGE_SPEED_Z;
        }
    }
}
