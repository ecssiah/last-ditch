pub mod act;

pub use act::Act;

use crate::simulation::{
    constants::PITCH_LIMIT,
    state::{
        action::act::MoveData,
        population::{judge::Judge, kinematic::Kinematic},
        State,
    },
};
use std::collections::VecDeque;

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
                Act::Move(move_data) => Self::apply_move(&move_data, &mut state.population.judge),
                Act::Jump => Self::apply_jump(&mut state.population.judge.kinematic),
                Act::PlaceBlock => State::place_block(state),
                Act::RemoveBlock => State::remove_block(state),
            }
        }
    }

    pub fn apply_move(move_data: &MoveData, judge: &mut Judge) {
        const MOVEMENT_EPSILON: f32 = 1e-6;

        if move_data.rotation_xy.abs() > MOVEMENT_EPSILON
            || move_data.rotation_yz.abs() > MOVEMENT_EPSILON
        {
            judge.sight.rotation_xy = judge.sight.rotation_xy + move_data.rotation_xy;

            judge.sight.rotation_yz =
                (judge.sight.rotation_yz + move_data.rotation_yz).clamp(-PITCH_LIMIT, PITCH_LIMIT);

            Judge::set_rotation(judge.sight.rotation_xy, judge.sight.rotation_yz, judge);
        }

        if move_data.direction.mag_sq() > MOVEMENT_EPSILON {
            let local_velocity = judge.kinematic.speed * move_data.direction.normalized();
            let velocity = judge.spatial.rotor * local_velocity;

            judge.kinematic.velocity.x = velocity.x;
            judge.kinematic.velocity.y = velocity.y;
        } else {
            judge.kinematic.velocity.x = 0.0;
            judge.kinematic.velocity.y = 0.0;
        }
    }

    pub fn apply_jump(kinematic: &mut Kinematic) {
        kinematic.velocity.z = kinematic.jump_speed;
    }
}
