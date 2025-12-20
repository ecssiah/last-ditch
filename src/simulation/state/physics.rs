//! Forces affecting Population

pub mod body;
pub mod collider;

use crate::{
    simulation::{
        constants::*,
        state::{
            physics::{body::Body, collider::Collider},
            population::{
                kinematic::Kinematic, person::Person, sight::Sight, transform::Transform,
                Population,
            },
            world::grid::{self, axis::Axis},
            World,
        },
    },
    utils::ldmath::FloatBox,
};
use ultraviolet::Vec3;

#[derive(Default)]
pub struct Physics {
    pub active: bool,
    pub gravity_active: bool,
    pub gravity: Vec3,
}

impl Physics {
    pub fn new() -> Self {
        let active = false;
        let gravity_active = true;
        let gravity = Vec3::new(0.0, 0.0, -GRAVITY_ACCELERATION);

        Self {
            active,
            gravity_active,
            gravity,
        }
    }

    pub fn tick(world: &World, population: &mut Population, physics: &mut Self) {
        let _ = tracing::info_span!("physics_tick").entered();

        if !physics.active {
            return;
        }

        if let Some(mut judge) = population.person_map.get_mut(&ID_JUDGE_1) {
            let (velocity, delta) = Self::integrate(physics, &mut judge.kinematic);

            Self::resolve_person(world, &velocity, &delta, &mut judge);
        }
    }

    pub fn set_gravity_active(gravity_active: bool, physics: &mut Self) {
        physics.gravity_active = gravity_active;
    }

    pub fn toggle_gravity_active(physics: &mut Self) {
        Self::set_gravity_active(!physics.gravity_active, physics);
    }

    fn integrate(physics: &Self, kinematic: &mut Kinematic) -> (Vec3, Vec3) {
        let initial_velocity = kinematic.velocity;

        let acceleration = if physics.gravity_active {
            physics.gravity
        } else {
            Vec3::zero()
        };

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        let delta = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta)
    }

    fn resolve_person(world: &World, velocity: &Vec3, delta: &Vec3, person: &mut Person) {
        let core_collider =
            Body::get_collider(collider::Label::Core, &person.body).expect("Body is missing core");

        let mut core_world_position = Collider::get_world_position(core_collider);
        let mut core_float_box = core_collider.float_box.clone();

        let mut velocity = *velocity;

        for axis in [Axis::Z, Axis::Y, Axis::X] {
            let delta_along_axis = match axis {
                Axis::X => delta.x,
                Axis::Y => delta.y,
                Axis::Z => delta.z,
            };

            let resolution_delta =
                Self::compute_resolution_along_axis(&core_float_box, world, axis, delta_along_axis);

            let displacement = Axis::unit(axis) * resolution_delta;

            core_world_position += displacement;

            core_float_box = FloatBox::translated(displacement, &core_float_box);

            let movement_blocked = (resolution_delta - delta_along_axis).abs() > EPSILON_COLLISION;

            if movement_blocked {
                match axis {
                    Axis::X => velocity.x = 0.0,
                    Axis::Y => velocity.y = 0.0,
                    Axis::Z => velocity.z = 0.0,
                }
            }
        }

        person.kinematic.velocity = velocity;

        let core_collider = Body::get_collider_mut(collider::Label::Core, &mut person.body)
            .expect("Body has no core");

        Collider::set_world_position(core_world_position, core_collider);

        let core_collider_radius = Collider::get_size(&core_collider) / 2.0;

        let person_world_position = Vec3::new(
            core_world_position.x,
            core_world_position.y,
            core_world_position.z - core_collider_radius.z + CELL_RADIUS_IN_METERS,
        );

        Transform::set_world_position(person_world_position, &mut person.transform);
        Sight::apply_world_position(person_world_position, &mut person.sight);
    }

    fn compute_resolution_along_axis(
        float_box: &FloatBox,
        world: &World,
        axis: Axis,
        delta: f32,
    ) -> f32 {
        if delta.abs() < EPSILON_COLLISION {
            return delta;
        }

        let sign = if delta >= 0.0 { 1.0 } else { -1.0 };
        let delta_abs = delta.abs();

        let mut min = 0.0;
        let mut max = delta_abs;
        let mut final_abs = 0.0;

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let mid = (min + max) * 0.5;

            let displacement = Axis::unit(axis) * (sign * mid);
            let float_box_test = FloatBox::translated(displacement, float_box);

            if Self::is_float_box_colliding(&float_box_test, world) {
                max = mid;
            } else {
                final_abs = mid;
                min = mid;
            }
        }

        sign * final_abs
    }

    fn is_float_box_colliding(float_box: &FloatBox, world: &World) -> bool {
        grid::get_cell_overlap_vec(float_box)
            .into_iter()
            .any(|cell_grid_position| World::is_block_solid_at(cell_grid_position, world))
    }
}
