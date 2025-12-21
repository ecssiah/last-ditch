//! Forces affecting Population

pub mod body;
pub mod collider;

use std::f32;

use crate::{
    simulation::{
        constants::*,
        state::{
            physics::body::Body,
            population::{kinematic::Kinematic, person::Person, Population},
            world::grid::{self, axis::Axis},
            World,
        },
    },
    utils::ldmath::{vec3_ext, FloatBounds, FloatBox},
};
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

        if !physics.active {
            return;
        }

        if let Some(mut judge) = population.person_map.get_mut(&ID_JUDGE_1) {
            let (velocity, delta) = Self::integrate(physics, &mut judge.kinematic);

            let (resolved_delta, axis_mask) = Self::resolve_person(world, &delta, &mut judge);

            Person::set_world_position(judge.transform.world_position + resolved_delta, judge);

            judge.kinematic.velocity = axis_mask * velocity;
        }
    }

    pub fn set_gravity_active(active: bool, physics: &mut Self) {
        physics.gravity = if active {
            Vec3::new(0.0, 0.0, -GRAVITY_ACCELERATION)
        } else {
            Vec3::zero()
        }
    }

    pub fn toggle_gravity_active(physics: &mut Self) {
        if vec3_ext::approx_eq(physics.gravity, Vec3::zero(), f32::EPSILON) {
            Self::set_gravity_active(true, physics);
        } else {
            Self::set_gravity_active(false, physics);
        }
    }

    fn integrate(physics: &Self, kinematic: &mut Kinematic) -> (Vec3, Vec3) {
        let initial_velocity = kinematic.velocity;
        let acceleration = physics.gravity;

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        let delta = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta)
    }

    fn resolve_person(world: &World, delta: &Vec3, person: &mut Person) -> (Vec3, Vec3) {
        let core_collider =
            Body::get_collider(collider::Label::Core, &person.body).expect("Body is missing core");

        let mut core_float_box = core_collider.float_box.clone();

        let mut resolution_delta_vec3 = Vec3::zero();
        let mut axis_mask_vec3 = Vec3::one();

        for axis in [Axis::Z, Axis::Y, Axis::X] {
            let axis_index = Axis::index(axis);

            let (resolution_delta_axis, axis_mask) = Self::compute_resolution_along_axis(
                &core_float_box,
                world,
                delta[axis_index],
                axis,
            );

            resolution_delta_vec3[axis_index] = resolution_delta_axis;
            axis_mask_vec3[axis_index] = axis_mask;

            core_float_box =
                FloatBox::translated(Axis::unit(axis) * resolution_delta_vec3, &core_float_box);
        }

        (resolution_delta_vec3, axis_mask_vec3)
    }

    fn compute_resolution_along_axis(
        float_box: &FloatBox,
        world: &World,
        delta: f32,
        axis: Axis,
    ) -> (f32, f32) {
        if delta.abs() < EPSILON_COLLISION {
            return (delta, 1.0);
        }

        let mut delta_resolved = 0.0;

        let mut float_bounds = FloatBounds::new(0.0, delta);

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let midpoint = FloatBounds::get_midpoint(&float_bounds);

            let float_box_test = FloatBox::translated(Axis::unit(axis) * midpoint, float_box);

            if Self::is_float_box_colliding(&float_box_test, world) {
                float_bounds.max = midpoint;
            } else {
                float_bounds.min = midpoint;
                delta_resolved = midpoint;
            }
        }

        let axis_mask = if (delta_resolved - delta).abs() > EPSILON_COLLISION {
            0.0
        } else {
            1.0
        };

        (delta_resolved, axis_mask)
    }

    fn is_float_box_colliding(float_box: &FloatBox, world: &World) -> bool {
        grid::get_cell_overlap_vec(float_box)
            .into_iter()
            .any(|cell_grid_position| World::is_block_solid_at(cell_grid_position, world))
    }
}
