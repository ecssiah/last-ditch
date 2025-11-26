//! Action processor

pub mod action;

pub use action::Action;

use crate::simulation::{
    constants::PITCH_LIMIT,
    state::{
        population::{kinematic::Kinematic, spatial::Spatial},
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
                Action::ToggleDebug => state.admin.debug_active = !state.admin.debug_active,
                Action::Start => State::init_load(state),
                Action::Quit => State::init_shutdown(state),
                Action::Exit => receiver.is_off = true,
                Action::Test1 => tracing::info!("Test Action 1"),
                Action::Test2 => tracing::info!("Test Action 2"),
                Action::Test3 => tracing::info!("Test Action 3"),
                Action::Test4 => tracing::info!("Test Action 4"),
                Action::Move(move_data) => Self::apply_move(
                    &move_data,
                    &mut state.population.judge.spatial,
                    &mut state.population.judge.kinematic,
                ),
                Action::Jump => Self::apply_jump(&mut state.population.judge.kinematic),
            }
        }
    }

    pub fn apply_move(
        move_data: &action::MoveData,
        spatial: &mut Spatial,
        kinematic: &mut Kinematic,
    ) {
        if move_data.rotation.x.abs() > 1e-6 || move_data.rotation.y.abs() > 1e-6 {
            spatial.yaw += move_data.rotation.x;
            spatial.pitch += move_data.rotation.y;

            spatial.pitch = spatial.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

            let yaw_rotor = Rotor3::from_rotation_xy(spatial.yaw);
            let pitch_rotor = Rotor3::from_rotation_yz(-spatial.pitch);

            spatial.rotor = yaw_rotor * pitch_rotor;
        }

        if move_data.direction.mag_sq() > 1e-6 {
            let yaw_rotor = Rotor3::from_rotation_xy(spatial.yaw);

            let local_velocity = kinematic.speed
                * Vec3::new(move_data.direction.x, move_data.direction.y, 0.0).normalized();

            let velocity = yaw_rotor * local_velocity;

            kinematic.velocity.x = velocity.x;
            kinematic.velocity.y = velocity.y;
        } else {
            kinematic.velocity.x = 0.0;
            kinematic.velocity.y = 0.0;
        }
    }

    pub fn apply_jump(kinematic: &mut Kinematic) {
        kinematic.velocity.z = kinematic.jump_speed;
    }
}
