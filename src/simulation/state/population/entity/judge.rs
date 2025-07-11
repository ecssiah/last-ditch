use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
        population::entity::{self, Detection, Kinematic, Nation, Spatial},
        receiver::action::{JudgeAction, JumpAction, MovementData},
        world::{chunk, World},
    },
};
use glam::{Quat, Vec3};

pub struct Judge {
    pub id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub action_vec: Vec<JudgeAction>,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
    pub kind: entity::Kind,
    pub nation: Nation,
}

impl Judge {
    pub fn new() -> Self {
        Self {
            id: entity::ID::allocate(),
            chunk_id: chunk::ID(0),
            chunk_updated: false,
            action_vec: Vec::new(),
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
            kind: entity::Kind::Eagle,
            nation: Nation {
                kind: entity::Kind::Eagle,
            },
        }
    }

    pub fn tick(judge: &mut Judge, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(judge.spatial.world_position);

        if chunk_id != judge.chunk_id {
            judge.chunk_updated = true;
            judge.chunk_id = chunk_id;
        }

        let action_vec = std::mem::take(&mut judge.action_vec);

        for action in action_vec {
            match action {
                JudgeAction::Movement(movement_data) => {
                    Judge::apply_movement_data(judge, &movement_data)
                }
                JudgeAction::Jump(jump_action) => Judge::apply_jump_action(judge, &jump_action),
            }
        }
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

    pub fn receive_action(&mut self, judge_action: &JudgeAction) {
        self.action_vec.push(*judge_action);
    }

    pub fn apply_movement_data(judge: &mut Judge, movement_data: &MovementData) {
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

    pub fn apply_jump_action(judge: &mut Judge, jump_action: &JumpAction) {
        if let JumpAction::Start = jump_action {
            judge.kinematic.velocity.y = JUDGE_SPEED_Y;
        }
    }
}
