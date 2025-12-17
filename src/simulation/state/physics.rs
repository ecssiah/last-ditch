//! Forces affecting Population

pub mod collider;

use crate::simulation::{
    constants::*,
    state::{
        body::person_body::PersonBody,
        physics::collider::Collider,
        population::{kinematic::Kinematic, sight::Sight, transform::Transform, Population},
        world::grid::{self, axis::Axis},
        World,
    },
};
use ultraviolet::{IVec3, Vec3};

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

        if let Some(judge) = population
            .person_map
            .get_mut(&population.leadership.judge_id)
        {
            let (velocity, delta) = Self::integrate(physics, &mut judge.kinematic);

            Self::resolve(
                world,
                &velocity,
                &delta,
                &mut judge.person_body,
                &mut judge.kinematic,
            );

            Self::sync(&judge.person_body, &mut judge.transform, &mut judge.sight);
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

    fn resolve(
        world: &World,
        velocity: &Vec3,
        delta: &Vec3,
        person_body: &mut PersonBody,
        kinematic: &mut Kinematic,
    ) {
        let mut velocity = *velocity;
        let mut collider = person_body.core;

        for axis in [Axis::Y, Axis::X, Axis::Z] {
            let delta_axis = match axis {
                Axis::X => delta.x,
                Axis::Y => delta.y,
                Axis::Z => delta.z,
            };

            let (resolved_collider, resolved_delta) =
                Self::resolve_axis(collider, world, axis, delta_axis);

            collider = resolved_collider;

            let blocked = (resolved_delta - delta_axis).abs() > EPSILON_COLLISION;

            match axis {
                Axis::X => {
                    if blocked {
                        velocity.x = 0.0;
                    }
                }
                Axis::Y => {
                    if blocked {
                        velocity.y = 0.0;
                    }
                }
                Axis::Z => {
                    if blocked {
                        velocity.z = 0.0;
                    }
                }
            }
        }

        person_body.core = collider;

        kinematic.velocity = velocity;
    }

    fn resolve_axis(collider: Collider, world: &World, axis: Axis, delta: f32) -> (Collider, f32) {
        if delta.abs() < EPSILON_COLLISION {
            return (collider, 0.0);
        }

        let mut min = 0.0;
        let mut max = delta;
        let mut final_delta = 0.0;

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let mid = (min + max) * 0.5;
            let test_collider = collider.translate(Axis::unit(axis) * mid);

            if Self::get_solid_collisions(test_collider, world).is_empty() {
                final_delta = mid;
                min = mid;
            } else {
                max = mid;
            }
        }

        let collider_resolved = collider.translate(Axis::unit(axis) * final_delta);

        (collider_resolved, final_delta)
    }

    fn get_solid_collisions(collider: Collider, world: &World) -> Vec<Collider> {
        grid::cells_overlapping(collider)
            .into_iter()
            .filter(|cell_collider| {
                let collider_center_position = cell_collider.center();

                let cell_position = IVec3::new(
                    collider_center_position.x.round() as i32,
                    collider_center_position.y.round() as i32,
                    collider_center_position.z.round() as i32,
                );

                if grid::is_grid_position_valid(cell_position) {
                    let cell = World::get_cell_at(cell_position, &world.sector_vec);

                    cell.solid
                } else {
                    true
                }
            })
            .collect()
    }

    fn sync(person_body: &PersonBody, transform: &mut Transform, sight: &mut Sight) {
        Transform::set_world_position(person_body.core.bottom_center(), transform);

        Sight::set_world_position(
            person_body.core.bottom_center() + sight.relative_position,
            sight,
        );
    }
}
