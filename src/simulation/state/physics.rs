//! Forces affecting Population

pub mod body;
pub mod collider;

use crate::{
    simulation::{
        constants::*,
        state::{
            physics::body::Body,
            population::{person::Person, Population},
            world::grid::{self, axis::Axis},
            World,
        },
    },
    utils::ldmath::FloatBox,
};
use std::f32;
use ultraviolet::Vec3;

#[derive(Default)]
pub struct Physics {
    pub active: bool,
    pub gravity: Vec3,
}

impl Physics {
    pub fn new() -> Self {
        let active = false;
        let gravity = Vec3::new(0.0, 0.0, -GRAVITY_ACCELERATION);

        Self { active, gravity }
    }

    pub fn tick(world: &World, population: &mut Population, physics: &mut Self) {
        let _ = tracing::info_span!("physics_tick").entered();

        if let Some(judge) = population.person_map.get_mut(&ID_JUDGE_1) {
            let (velocity, delta_position_intent) = Self::integrate_person(physics, judge);

            let (delta_position_resolved, velocity_mask) =
                Self::compute_resolution_person(&delta_position_intent, world, judge);

            let world_position = judge.transform.world_position + delta_position_resolved;
            let velocity = velocity_mask * velocity;

            Person::set_world_position(world_position, judge);
            Person::set_velocity(velocity, judge);
        }
    }

    fn integrate_person(physics: &Self, judge: &mut Person) -> (Vec3, Vec3) {
        let initial_velocity = judge.motion.velocity;

        let acceleration = if judge.body.is_massive && !judge.body.is_grounded {
            physics.gravity
        } else {
            Vec3::zero()
        };

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        let delta_position_intent = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta_position_intent)
    }

    fn compute_resolution_person(
        delta_position_intent: &Vec3,
        world: &World,
        person: &mut Person,
    ) -> (Vec3, Vec3) {
        let core_collider =
            Body::get_collider(collider::Label::Core, &person.body).expect("Body is missing core");

        let ground_collider = Body::get_collider(collider::Label::Ground, &person.body)
            .expect("Body is missing ground");

        let mut core_float_box = core_collider.float_box.clone();

        let mut delta_position_resolved = Vec3::zero();
        let mut velocity_mask = Vec3::one();

        for delta_axis in [Axis::Z, Axis::X, Axis::Y] {
            let axis_index = Axis::index(delta_axis);

            let core_float_box_query = if delta_axis != Axis::Z && person.body.is_grounded {
                let vertical_offset = Vec3::new(0.0, 0.0, COLLISION_EPSILON);

                FloatBox::translated(vertical_offset, &core_float_box)
            } else {
                core_float_box.clone()
            };

            let (delta_position_resolved_axis, velocity_mask_axis) =
                Self::compute_resolution_along_axis(
                    core_float_box_query,
                    delta_axis,
                    delta_position_intent[axis_index],
                    world,
                );

            delta_position_resolved[axis_index] = delta_position_resolved_axis;
            velocity_mask[axis_index] = velocity_mask_axis;

            core_float_box = FloatBox::translated(
                Axis::unit(delta_axis) * delta_position_resolved,
                &core_float_box,
            );
        }

        person.body.is_grounded = Self::is_float_box_colliding(&ground_collider.float_box, world);

        (delta_position_resolved, velocity_mask)
    }

    fn compute_resolution_along_axis(
        float_box: FloatBox,
        delta_axis: Axis,
        delta_position_intent: f32,
        world: &World,
    ) -> (f32, f32) {
        if delta_axis != Axis::Z && delta_position_intent.abs() < COLLISION_EPSILON {
            return (0.0, 1.0);
        }

        let mut delta_position_resolved = 0.0;

        let mut t_min = 0.0;
        let mut t_max = 1.0;

        for _ in 0..COLLISION_RESOLVE_ITERATIONS {
            let t_mid = (t_min + t_max) * 0.5;
            let delta = delta_position_intent * t_mid;

            let float_box_test = FloatBox::translated(Axis::unit(delta_axis) * delta, &float_box);

            if Self::is_float_box_colliding(&float_box_test, world) {
                t_max = t_mid;
            } else {
                t_min = t_mid;
                delta_position_resolved = delta;
            }
        }

        let collision_occurred =
            (delta_position_resolved - delta_position_intent).abs() > COLLISION_EPSILON;

        let velocity_mask = if collision_occurred { 0.0 } else { 1.0 };

        (delta_position_resolved, velocity_mask)
    }

    fn is_float_box_colliding(float_box: &FloatBox, world: &World) -> bool {
        grid::get_grid_overlap_vec(float_box)
            .into_iter()
            .any(|cell_grid_position| World::is_block_solid_at(cell_grid_position, world))
    }
}
