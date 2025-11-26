pub mod act;

pub use act::Act;

use crate::simulation::{
    constants::PITCH_LIMIT,
    state::{
        action::act::MoveData,
        population::{kinematic::Kinematic, spatial::Spatial},
        state_loading, state_shutdown, Admin, State,
    },
};
use std::collections::VecDeque;
use ultraviolet::{Rotor3, Vec3};

pub struct Action {
    pub act_deque: VecDeque<Act>,
}

impl Action {
    pub fn new() -> Self {
        let act_deque = VecDeque::new();

        Self { act_deque }
    }

    pub fn tick(state: &mut State) {
        let act_deque: VecDeque<Act> = state.action.act_deque.drain(..).collect();

        for act in act_deque {
            match act {
                Act::Start => state_loading::init(state),
                Act::Quit => state_shutdown::init(state),
                Act::Exit => state.active = false,
                Act::Debug => Admin::toggle_debug(&mut state.admin),
                Act::Move(move_data) => Self::apply_move(
                    &move_data,
                    &mut state.population.judge.spatial,
                    &mut state.population.judge.kinematic,
                ),
                Act::Jump => Self::apply_jump(&mut state.population.judge.kinematic),
                Act::Test1 => tracing::info!("Test Action 1"),
                Act::Test2 => tracing::info!("Test Action 2"),
                Act::Test3 => tracing::info!("Test Action 3"),
                Act::Test4 => tracing::info!("Test Action 4"),
            }
        }
    }

    pub fn apply_move(move_data: &MoveData, spatial: &mut Spatial, kinematic: &mut Kinematic) {
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
