//! Action processor

pub mod action;

pub use action::Action;

use crate::simulation::{
    constants::PITCH_LIMIT,
    state::{
        population::{kinematic::Kinematic, spatial::Spatial},
        receiver::action::{AdminAction, JumpAction, MovementData},
        State,
    },
};
use tokio::sync::mpsc::UnboundedReceiver;
use ultraviolet::{Rotor3, Vec3};

pub struct Receiver {
    pub is_off: bool,
    pub action_rx: UnboundedReceiver<Action>,
}

impl Receiver {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        Self {
            is_off: false,
            action_rx,
        }
    }

    pub fn tick(receiver: &mut Receiver, state: &mut State) {
        while let Ok(action) = receiver.action_rx.try_recv() {
            match action {
                Action::Admin(admin_action) => match admin_action {
                    AdminAction::Debug => state.admin.debug_active = !state.admin.debug_active,
                    AdminAction::Start => State::init_load(state),
                    AdminAction::Quit => State::init_shutdown(state),
                    AdminAction::Exit => receiver.is_off = true,
                },
                Action::Test(test_action) => match test_action {
                    action::TestAction::Test1 => tracing::info!("Test Action 1"),
                    action::TestAction::Test2 => tracing::info!("Test Action 2"),
                    action::TestAction::Test3 => tracing::info!("Test Action 3"),
                    action::TestAction::Test4 => tracing::info!("Test Action 4"),
                },
                Action::Judge(judge_action) => match judge_action {
                    action::JudgeAction::Movement(movement_data) => Self::apply_movement_data(
                        &movement_data,
                        &mut state.population.judge.spatial,
                        &mut state.population.judge.kinematic,
                    ),
                    action::JudgeAction::Jump(jump_action) => {
                        Self::apply_jump_action(&jump_action, &mut state.population.judge.kinematic)
                    }
                },
            }
        }
    }

    pub fn apply_movement_data(
        movement_data: &MovementData,
        spatial: &mut Spatial,
        kinematic: &mut Kinematic,
    ) {
        if movement_data.rotation.x.abs() > 1e-6 || movement_data.rotation.y.abs() > 1e-6 {
            spatial.yaw += movement_data.rotation.x;
            spatial.pitch += movement_data.rotation.y;

            spatial.pitch = spatial.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

            let yaw_rotor = Rotor3::from_rotation_xy(spatial.yaw);
            let pitch_rotor = Rotor3::from_rotation_yz(-spatial.pitch);

            spatial.rotor = yaw_rotor * pitch_rotor;
        }

        if movement_data.direction.mag_sq() > 1e-6 {
            let yaw_rotor = Rotor3::from_rotation_xy(spatial.yaw);

            let local_velocity = Vec3::new(
                movement_data.direction.x * kinematic.speed,
                movement_data.direction.y * kinematic.speed,
                0.0,
            );

            let velocity = yaw_rotor * local_velocity;

            kinematic.velocity.x = velocity.x;
            kinematic.velocity.y = velocity.y;
        } else {
            kinematic.velocity.x = 0.0;
            kinematic.velocity.y = 0.0;
        }
    }

    pub fn apply_jump_action(jump_action: &JumpAction, kinematic: &mut Kinematic) {
        if let JumpAction::Start = jump_action {
            kinematic.velocity.z = kinematic.jump_speed;
        }
    }
}
