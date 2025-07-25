use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{
            entity::{self, Detection, Info, Kinematic, Spatial},
            nation,
        },
        receiver::action::{JumpAction, MovementData},
        world::{chunk, World},
    },
};
use glam::{Quat, Vec3};

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
            chunk_id: chunk::ID(0),
            chunk_updated: false,
            entity_kind: entity::Kind::Judge,
            nation_kind: nation::Kind::Eagle,
        };

        Self {
            info,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
        }
    }

    pub fn tick(judge: &mut Judge, world: &World) {
        Info::update_chunk_id(&judge.spatial, &world.grid, &mut judge.info);
    }

    pub fn set_world_position(
        world_position: Vec3,
        spatial: &mut Spatial,
        detection: &mut Detection,
    ) {
        spatial.world_position = world_position;

        Detection::set_world_position(world_position, &mut detection.body);
    }

    pub fn size(&self) -> Vec3 {
        self.detection.body.size()
    }

    pub fn set_size(size: Vec3, detection: &mut Detection) {
        detection.body = AABB::new(detection.body.center(), size);
    }

    pub fn set_rotation(yaw: f32, pitch: f32, spatial: &mut Spatial, kinematic: &mut Kinematic) {
        spatial.yaw = yaw.to_radians();
        spatial.pitch = pitch.to_radians();

        spatial.pitch = spatial.pitch.clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

        spatial.quaternion =
            Quat::from_rotation_y(spatial.yaw) * Quat::from_rotation_x(spatial.pitch);

        let velocity_xz = Vec3::new(kinematic.velocity.x, 0.0, kinematic.velocity.z);
        let speed = velocity_xz.length_squared();

        if speed > 1e-12 {
            let forward = spatial.quaternion * Vec3::Z;
            let new_velocity_xz = forward.normalize() * speed;

            kinematic.velocity.x = new_velocity_xz.x;
            kinematic.velocity.z = new_velocity_xz.z;
        }
    }

    pub fn eye(&self) -> Vec3 {
        self.spatial.world_position + self.spatial.up() * 0.9 * self.size().y
    }

    pub fn apply_movement_data(movement_data: &MovementData, judge: &mut Judge) {
        if movement_data.rotation.y.abs() > 1e-6 || movement_data.rotation.z.abs() > 1e-6 {
            judge.spatial.yaw += movement_data.rotation.y;
            judge.spatial.pitch += movement_data.rotation.z;

            judge.spatial.pitch = judge
                .spatial
                .pitch
                .clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

            judge.spatial.quaternion = Quat::from_rotation_y(judge.spatial.yaw)
                * Quat::from_rotation_x(judge.spatial.pitch);
        }

        let input_direction = movement_data.direction.normalize_or_zero();

        if input_direction.length_squared() > 1e-6 {
            let yaw_quat = Quat::from_rotation_y(judge.spatial.yaw);

            let local_velocity = input_direction * Vec3::new(JUDGE_SPEED_X, 0.0, JUDGE_SPEED_Z);
            let velocity = yaw_quat * local_velocity;

            judge.kinematic.velocity.x = velocity.x;
            judge.kinematic.velocity.z = velocity.z;
        } else {
            judge.kinematic.velocity.x = 0.0;
            judge.kinematic.velocity.z = 0.0;
        }
    }

    pub fn apply_jump_action(jump_action: &JumpAction, judge: &mut Judge) {
        if let JumpAction::Start = jump_action {
            judge.kinematic.velocity.y = JUDGE_SPEED_Y;
        }
    }
}
