pub mod act;

use crate::{
    simulation::{
        constants::{MOVEMENT_EPSILON, PITCH_LIMIT},
        state::{
            action::act::{
                move_data::MoveData, Act, JumpData, PlaceBlockData, RemoveBlockData, RotateData,
            },
            physics::body::{self, ContactSet},
            population::{motion, person::Person},
            Population, State, World,
        },
    },
    utils::ldmath::float_ext,
};
use std::{collections::VecDeque, f32::EPSILON};
use tracing::instrument;
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

    #[instrument(skip_all)]
    pub fn tick(state: &mut State) {
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
            match person.motion.mode {
                motion::Mode::Ground => Self::apply_ground_move(move_data, person),
                motion::Mode::Climb => Self::apply_climb_move(move_data, person),
                motion::Mode::Air => Self::apply_air_move(move_data, person),
            }
        }
    }

    fn apply_ground_move(move_data: &MoveData, person: &mut Person) {
        if move_data.move_direction.mag_sq() > MOVEMENT_EPSILON {
            if ContactSet::has(body::Contact::Ladder, &person.body.contact_set) {
                if float_ext::not_equal(move_data.move_direction.z, 0.0, EPSILON) {
                    person.motion.mode = motion::Mode::Climb;

                    return;
                }
            }

            let horizontal_movement_direction =
                Vec3::new(move_data.move_direction.x, move_data.move_direction.y, 0.0);

            let horizontal_movement_direction =
                if horizontal_movement_direction.mag_sq() > MOVEMENT_EPSILON {
                    horizontal_movement_direction.normalized()
                } else {
                    Vec3::zero()
                };

            let local_velocity = person.motion.ground_speed * horizontal_movement_direction;
            let velocity = person.transform.rotor * local_velocity;

            person.motion.velocity.x = velocity.x;
            person.motion.velocity.y = velocity.y;
        } else {
            person.motion.velocity.x = 0.0;
            person.motion.velocity.y = 0.0;
        }
    }

    fn apply_climb_move(move_data: &MoveData, person: &mut Person) {
        if move_data.move_direction.mag_sq() > MOVEMENT_EPSILON {
            let local_horizontal_movement_direction =
                Vec3::new(move_data.move_direction.x, move_data.move_direction.y, 0.0);

            let horizontal_move_direction =
                person.sight.rotor * local_horizontal_movement_direction;

            let horizontal_ground_velocity =
                horizontal_move_direction * 0.6 * person.motion.climb_speed;

            let vertical_movement_direction = Vec3::new(0.0, 0.0, move_data.move_direction.z);
            let vertical_climb_velocity = vertical_movement_direction * person.motion.climb_speed;

            person.motion.velocity = horizontal_ground_velocity + vertical_climb_velocity;
        } else {
            person.motion.velocity = Vec3::zero();
        }
    }

    fn apply_air_move(move_data: &MoveData, person: &mut Person) {
        if move_data.move_direction.mag_sq() > MOVEMENT_EPSILON {
            let local_horizontal_move_direction =
                Vec3::new(move_data.move_direction.x, move_data.move_direction.y, 0.0);

            let horizontal_move_direction = person.sight.rotor * local_horizontal_move_direction;
            let vertical_move_direction = Vec3::new(0.0, 0.0, move_data.move_direction.z);

            let move_direction = (horizontal_move_direction + vertical_move_direction).normalized();

            let velocity = move_direction * person.motion.air_speed;

            person.motion.velocity = velocity;
        } else {
            person.motion.velocity = Vec3::zero();
        }
    }

    pub fn apply_jump(jump_data: &JumpData, population: &mut Population) {
        if let Some(person) = population.person_map.get_mut(&jump_data.person_id) {
            match person.motion.mode {
                motion::Mode::Ground => {
                    if ContactSet::has(body::Contact::Ground, &person.body.contact_set) {
                        person.motion.velocity.z = person.motion.jump_speed
                    }
                }
                motion::Mode::Climb => (),
                motion::Mode::Air => (),
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
