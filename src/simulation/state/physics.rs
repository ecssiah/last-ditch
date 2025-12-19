//! Forces affecting Population

pub mod collider;

use crate::simulation::{
    constants::*,
    state::{
        body::SimpleBody,
        population::{kinematic::Kinematic, sight::Sight, transform::Transform, Population},
        world::grid::{self, axis::Axis},
        World,
    },
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

        if let Some(judge) = population.person_map.get_mut(&ID_JUDGE_1) {
            let (velocity, delta) = Self::integrate(physics, &mut judge.kinematic);

            Self::resolve_simple_body(
                world,
                &velocity,
                &delta,
                &mut judge.body,
                &mut judge.kinematic,
            );

            Self::sync_simple_body(&judge.body, &mut judge.transform, &mut judge.sight);
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

    fn resolve_simple_body(
        world: &World,
        velocity: &Vec3,
        delta: &Vec3,
        body: &mut SimpleBody,
        kinematic: &mut Kinematic,
    ) {
        let mut velocity = *velocity;
        let mut body_working = body.clone();

        for axis in [Axis::Y, Axis::X, Axis::Z] {
            let delta_axis = match axis {
                Axis::X => delta.x,
                Axis::Y => delta.y,
                Axis::Z => delta.z,
            };

            let (body_resolved, delta_resolved) =
                Self::resolve_axis_simple_body(body_working, world, axis, delta_axis);

            body_working = body_resolved;

            let blocked = (delta_resolved - delta_axis).abs() > EPSILON_COLLISION;

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

        *body = body_working;
        kinematic.velocity = velocity;
    }

    fn resolve_axis_simple_body(
        body: SimpleBody,
        world: &World,
        axis: Axis,
        delta: f32,
    ) -> (SimpleBody, f32) {
        if delta.abs() < EPSILON_COLLISION {
            return (body, 0.0);
        }

        let mut min = 0.0;
        let mut max = delta;

        let mut final_delta = 0.0;

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let mid = (min + max) * 0.5;

            let test_body = SimpleBody {
                active: true,
                world_position: body.world_position + Axis::unit(axis) * mid,
                collider: body.collider,
            };

            if Self::is_simple_body_colliding(&test_body, world) {
                max = mid;
            } else {
                final_delta = mid;
                min = mid;
            }
        }

        let body_resolved = SimpleBody {
            active: true,
            world_position: body.world_position + Axis::unit(axis) * final_delta,
            collider: body.collider,
        };

        (body_resolved, final_delta)
    }

    fn is_simple_body_colliding(body: &SimpleBody, world: &World) -> bool {
        grid::get_cell_overlap_vec(&SimpleBody::get_fbox(body))
            .into_iter()
            .any(|cell_grid_position| {
                if grid::is_grid_position_valid(cell_grid_position) {
                    World::get_block_at(cell_grid_position, &world.sector_vec)
                        .map(|block| block.solid)
                        .unwrap_or(false)
                } else {
                    true
                }
            })
    }

    fn sync_simple_body(body: &SimpleBody, transform: &mut Transform, sight: &mut Sight) {
        Transform::set_world_position(body.world_position, transform);
        Sight::set_world_position(body.world_position + sight.relative_position, sight);
    }
}
