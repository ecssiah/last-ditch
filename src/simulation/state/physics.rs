//! Forces affecting Population

pub mod aabb;

use crate::simulation::{
    constants::*,
    state::{
        physics::aabb::AABB,
        population::{kinematic::Kinematic, sight::Sight, spatial::Spatial, Population},
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

    pub fn tick(world: &World, population: &mut Population, physics: &mut Physics) {
        let _ = tracing::info_span!("physics_tick").entered();

        if !physics.active {
            return;
        }

        let (velocity, delta) = Self::integrate(physics, &mut population.judge.kinematic);

        Self::resolve(
            world,
            &velocity,
            &delta,
            &mut population.judge.spatial,
            &mut population.judge.kinematic,
        );

        Self::sync(&mut population.judge.spatial, &mut population.judge.sight);
    }

    pub fn set_gravity_active(gravity_active: bool, physics: &mut Physics) {
        physics.gravity_active = gravity_active;
    }

    pub fn toggle_gravity_active(physics: &mut Physics) {
        Self::set_gravity_active(!physics.gravity_active, physics);
    }

    fn integrate(physics: &Physics, kinematic: &mut Kinematic) -> (Vec3, Vec3) {
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
        spatial: &mut Spatial,
        kinematic: &mut Kinematic,
    ) {
        let mut aabb = spatial.body;
        let mut velocity = *velocity;

        for axis in [Axis::Y, Axis::X, Axis::Z] {
            let delta_axis = match axis {
                Axis::X => delta.x,
                Axis::Y => delta.y,
                Axis::Z => delta.z,
            };

            let (resolved_aabb, step) = Self::resolve_axis(aabb, world, axis, delta_axis);

            aabb = resolved_aabb;

            match axis {
                Axis::X => velocity.x = step / SIMULATION_TICK_IN_SECONDS,
                Axis::Y => velocity.y = step / SIMULATION_TICK_IN_SECONDS,
                Axis::Z => velocity.z = step / SIMULATION_TICK_IN_SECONDS,
            }
        }

        spatial.body = aabb;
        kinematic.velocity = velocity;
    }

    fn resolve_axis(aabb: AABB, world: &World, axis: Axis, delta: f32) -> (AABB, f32) {
        if delta.abs() < EPSILON_COLLISION {
            return (aabb, 0.0);
        }

        let mut min = 0.0;
        let mut max = delta;
        let mut final_delta = 0.0;

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let mid = (min + max) * 0.5;
            let test_aabb = aabb.translate(Axis::unit(axis) * mid);

            if Self::get_solid_collisions(test_aabb, world).is_empty() {
                final_delta = mid;
                min = mid;
            } else {
                max = mid;
            }
        }

        let adjusted_aabb = aabb.translate(Axis::unit(axis) * final_delta);

        let adjusted_velocity = if (final_delta - delta).abs() > 0.0001 {
            0.0
        } else {
            final_delta
        };

        (adjusted_aabb, adjusted_velocity)
    }

    fn get_solid_collisions(aabb: AABB, world: &World) -> Vec<AABB> {
        grid::cells_overlapping(aabb)
            .into_iter()
            .filter(|cell_aabb| {
                let aabb_center = cell_aabb.center();

                let cell_position = IVec3::new(
                    aabb_center.x.round() as i32,
                    aabb_center.y.round() as i32,
                    aabb_center.z.round() as i32,
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

    fn sync(spatial: &mut Spatial, sight: &mut Sight) {
        Spatial::set_world_position(spatial.body.bottom_center(), spatial);
        
        Sight::set_world_position(
            spatial.body.bottom_center() + sight.relative_position,
            sight,
        );
    }
}
