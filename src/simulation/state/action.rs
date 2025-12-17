pub mod act;

pub use act::Act;

use crate::simulation::{
    constants::PITCH_LIMIT,
    state::{
        action::act::{move_data::MoveData, JumpData, PlaceBlockData, RemoveBlockData, RotateData},
        population::person::Person,
        Population, State, World,
    },
};
use std::collections::VecDeque;
use ultraviolet::Vec3;

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
                Act::Rotate(rotate_data) => Self::apply_rotate(&rotate_data, &mut state.population),
                Act::Move(move_data) => Self::apply_move(&move_data, &mut state.population),
                Act::Jump(jump_data) => Self::apply_jump(&jump_data, &mut state.population),
                Act::PlaceBlock(place_block_data) => Self::apply_place_block(
                    &place_block_data,
                    &mut state.world,
                    &mut state.population,
                ),
                Act::RemoveBlock(remove_block_data) => Self::apply_remove_block(
                    &remove_block_data,
                    &mut state.world,
                    &mut state.population,
                ),
            }
        }
    }

    pub fn apply_rotate(rotate_data: &RotateData, population: &mut Population) {
        if let Some(person) = population.person_map.get_mut(&rotate_data.person_id) {
            const ROTATION_EPSILON: f32 = 1e-6;

            if rotate_data.rotation_angles.mag_sq() > ROTATION_EPSILON {
                person.sight.rotation_xy = person.sight.rotation_xy + rotate_data.rotation_angles.z;

                person.sight.rotation_yz = (person.sight.rotation_yz
                    + rotate_data.rotation_angles.x)
                    .clamp(-PITCH_LIMIT, PITCH_LIMIT);

                Person::set_rotation(person.sight.rotation_xy, person.sight.rotation_yz, person);
            }
        }
    }

    pub fn apply_move(move_data: &MoveData, population: &mut Population) {
        if let Some(person) = population.person_map.get_mut(&move_data.person_id) {
            const MOVEMENT_EPSILON: f32 = 1e-6;

            if move_data.move_direction.mag_sq() > MOVEMENT_EPSILON {
                if person.kinematic.flying {
                    let local_horizontal_move_direction =
                        Vec3::new(move_data.move_direction.x, move_data.move_direction.y, 0.0);

                    let horizontal_move_direction =
                        person.sight.rotor * local_horizontal_move_direction;
                    let vertical_move_direction = Vec3::new(0.0, 0.0, move_data.move_direction.z);

                    let move_direction =
                        (horizontal_move_direction + vertical_move_direction).normalized();

                    let velocity = person.kinematic.speed * move_direction;

                    person.kinematic.velocity = velocity;
                } else {
                    let local_velocity = person.kinematic.speed * move_data.move_direction;

                    let velocity = person.transform.rotor * local_velocity;

                    person.kinematic.velocity.x = velocity.x;
                    person.kinematic.velocity.y = velocity.y;
                };
            } else {
                if person.kinematic.flying {
                    person.kinematic.velocity = Vec3::zero();
                } else {
                    person.kinematic.velocity.x = 0.0;
                    person.kinematic.velocity.y = 0.0;
                }
            }
        }
    }

    pub fn apply_jump(jump_data: &JumpData, population: &mut Population) {
        if let Some(person) = population.person_map.get_mut(&jump_data.person_id) {
            if !person.kinematic.flying {
                person.kinematic.velocity.z = person.kinematic.jump_speed;
            }
        }
    }

    fn apply_place_block(
        place_block_data: &PlaceBlockData,
        world: &mut World,
        population: &mut Population,
    ) {
        if let Some(person) = population.person_map.get_mut(&place_block_data.person_id) {
            State::place_block(person, world);
        }
    }

    fn apply_remove_block(
        remove_block_data: &RemoveBlockData,
        world: &mut World,
        population: &mut Population,
    ) {
        if let Some(person) = population.person_map.get_mut(&remove_block_data.person_id) {
            State::remove_block(person, world);
        }
    }
}
