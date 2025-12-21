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
    utils::ldmath::{FloatBounds, FloatBox},
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

        let acceleration = if judge.body.is_massive {
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

        let mut core_float_box = core_collider.float_box.clone();

        let mut delta_position_resolved = Vec3::zero();
        let mut velocity_mask = Vec3::one();

        for delta_axis in [Axis::Z, Axis::Y, Axis::X] {
            let axis_index = Axis::index(delta_axis);

            let (delta_position_resolved_axis, velocity_mask_axis) = Self::compute_resolution_axis(
                &core_float_box,
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

        (delta_position_resolved, velocity_mask)
    }

    fn compute_resolution_axis(
        float_box: &FloatBox,
        delta_axis: Axis,
        delta_position_intent: f32,
        world: &World,
    ) -> (f32, f32) {
        if delta_position_intent.abs() < EPSILON_COLLISION {
            return (delta_position_intent, 1.0);
        }

        let mut delta_position_resolved = 0.0;

        let mut float_bounds = FloatBounds::new(0.0, delta_position_intent);

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let midpoint = FloatBounds::get_midpoint(&float_bounds);

            let float_box_test = FloatBox::translated(Axis::unit(delta_axis) * midpoint, float_box);

            if Self::is_float_box_colliding(&float_box_test, world) {
                float_bounds.max = midpoint;
            } else {
                float_bounds.min = midpoint;

                delta_position_resolved = midpoint;
            }
        }

        let collision_occurred =
            (delta_position_resolved - delta_position_intent).abs() > EPSILON_COLLISION;

        let velocity_mask = if collision_occurred { 0.0 } else { 1.0 };

        (delta_position_resolved, velocity_mask)
    }

    fn is_float_box_colliding(float_box: &FloatBox, world: &World) -> bool {
        grid::get_grid_overlap_vec(float_box)
            .into_iter()
            .any(|cell_grid_position| World::is_block_solid_at(cell_grid_position, world))
    }
}
