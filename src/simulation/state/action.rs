pub mod act;

pub use act::Act;
use ultraviolet::Vec3;

use crate::simulation::{
    constants::PITCH_LIMIT,
    state::{
        action::act::{move_data::MoveData, rotate_data::RotateData},
        population::{judge::Judge, kinematic::Kinematic},
        State,
    },
};
use std::collections::VecDeque;

pub struct Action {
    pub active: bool,
    pub act_deque: VecDeque<Act>,
}

impl Action {
    pub fn new() -> Self {
        let active = false;
        let act_deque = VecDeque::new();

        Self { active, act_deque }
    }

    pub fn tick(state: &mut State) {
        let _ = tracing::info_span!("action_tick").entered();

        if !state.action.active {
            return;
        }

        let act_deque = std::mem::take(&mut state.action.act_deque);

        for act in act_deque {
            match act {
                Act::Rotate(rotate_data) => {
                    Self::apply_rotate(&rotate_data, &mut state.population.judge)
                }
                Act::Move(move_data) => Self::apply_move(&move_data, &mut state.population.judge),
                Act::Jump => Self::apply_jump(&mut state.population.judge.kinematic),
                Act::PlaceBlock => State::place_block(state),
                Act::RemoveBlock => State::remove_block(state),
            }
        }
    }

    pub fn apply_rotate(rotate_data: &RotateData, judge: &mut Judge) {
        const MOVEMENT_EPSILON: f32 = 1e-6;

        if rotate_data.rotate_xy.abs() > MOVEMENT_EPSILON
            || rotate_data.rotate_yz.abs() > MOVEMENT_EPSILON
        {
            judge.sight.rotation_xy = judge.sight.rotation_xy + rotate_data.rotate_xy;

            judge.sight.rotation_yz =
                (judge.sight.rotation_yz + rotate_data.rotate_yz).clamp(-PITCH_LIMIT, PITCH_LIMIT);

            Judge::set_rotation(judge.sight.rotation_xy, judge.sight.rotation_yz, judge);
        }
    }

    pub fn apply_move(move_data: &MoveData, judge: &mut Judge) {
        const MOVEMENT_EPSILON: f32 = 1e-6;

        if move_data.move_direction.mag_sq() > MOVEMENT_EPSILON {
            if judge.kinematic.flying {
                let local_horizontal_move_direction =
                    Vec3::new(move_data.move_direction.x, move_data.move_direction.y, 0.0);

                let horizontal_move_direction = judge.sight.rotor * local_horizontal_move_direction;
                let vertical_move_direction = Vec3::new(0.0, 0.0, move_data.move_direction.z);

                let move_direction = (horizontal_move_direction + vertical_move_direction).normalized();

                let velocity = judge.kinematic.speed * move_direction;

                judge.kinematic.velocity = velocity;
            } else {
                let local_velocity = judge.kinematic.speed * move_data.move_direction;

                let velocity = judge.spatial.rotor * local_velocity;

                judge.kinematic.velocity.x = velocity.x;
                judge.kinematic.velocity.y = velocity.y;
            };
        } else {
            if judge.kinematic.flying {
                judge.kinematic.velocity = Vec3::zero();
            } else {
                judge.kinematic.velocity.x = 0.0;
                judge.kinematic.velocity.y = 0.0;
            }
        }
    }

    pub fn apply_jump(kinematic: &mut Kinematic) {
        if !kinematic.flying {
            kinematic.velocity.z = kinematic.jump_speed;
        }
    }
}
