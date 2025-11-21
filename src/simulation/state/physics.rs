//! Forces affecting Population

pub mod aabb;

use crate::simulation::{
    constants::*,
    state::{
        physics::aabb::AABB,
        population::{entity::Entity, Population},
        world::grid::{self, Grid},
        World,
    },
};
use tracing::info_span;
use ultraviolet::{IVec3, Vec3};

#[derive(Default)]
pub struct Physics {
    pub gravity: Vec3,
}

impl Physics {
    pub fn new() -> Self {
        let gravity = Vec3::new(0.0, 0.0, -GRAVITY_ACCELERATION);

        Self { gravity }
    }

    pub fn tick(physics: &Physics, world: &World, population: &mut Population) {
        let _physics_span = info_span!("physics_tick").entered();

        let (velocity, delta) =
            Physics::integrate_entity(physics.gravity, &mut population.judge.entity);

        Self::resolve_entity(world, &velocity, &delta, &mut population.judge.entity);
        Self::sync_entity(&mut population.judge.entity);
    }

    fn integrate_entity(gravity: Vec3, entity: &mut Entity) -> (Vec3, Vec3) {
        let initial_velocity = entity.kinematic.velocity;
        let acceleration = entity.kinematic.acceleration + gravity;

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        let delta = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta)
    }

    fn resolve_entity(world: &World, velocity: &Vec3, delta: &Vec3, entity: &mut Entity) {
        let mut aabb = entity.sense.touch.body;
        let mut velocity = *velocity;

        for axis in [
            grid::Direction::North,
            grid::Direction::East,
            grid::Direction::Up,
        ] {
            let delta_axis = if axis == grid::Direction::North {
                delta.y
            } else if axis == grid::Direction::East {
                delta.x
            } else {
                delta.z
            };

            let (resolved_aabb, step) = Self::resolve_axis(aabb, world, axis, delta_axis);

            aabb = resolved_aabb;

            if axis == grid::Direction::North {
                velocity.y = step / SIMULATION_TICK_IN_SECONDS;
            } else if axis == grid::Direction::East {
                velocity.x = step / SIMULATION_TICK_IN_SECONDS;
            } else {
                velocity.z = step / SIMULATION_TICK_IN_SECONDS;
            }
        }

        entity.sense.touch.body = aabb;
        entity.kinematic.velocity = velocity;
    }

    fn resolve_axis(aabb: AABB, world: &World, axis: grid::Direction, delta: f32) -> (AABB, f32) {
        if delta.abs() < EPSILON_COLLISION {
            return (aabb, 0.0);
        }

        let mut min = 0.0;
        let mut max = delta;
        let mut final_delta = 0.0;

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let mid = (min + max) * 0.5;
            let test_aabb = aabb.translate(axis.to_vec3() * mid);

            if Self::get_solid_collisions(test_aabb, world).is_empty() {
                final_delta = mid;
                min = mid;
            } else {
                max = mid;
            }
        }

        let adjusted_aabb = aabb.translate(axis.to_vec3() * final_delta);

        let adjusted_velocity = if (final_delta - delta).abs() > 0.0001 {
            0.0
        } else {
            final_delta
        };

        (adjusted_aabb, adjusted_velocity)
    }

    fn get_solid_collisions(aabb: AABB, world: &World) -> Vec<AABB> {
        Grid::cells_overlapping(aabb, &world.grid)
            .into_iter()
            .filter(|cell_aabb| {
                let aabb_center = cell_aabb.center();

                let cell_position = IVec3::new(
                    aabb_center.x.round() as i32,
                    aabb_center.y.round() as i32,
                    aabb_center.z.round() as i32,
                );

                if Grid::position_valid(cell_position, &world.grid) {
                    let cell = World::get_cell_at(cell_position, &world.grid, &world.sector_vec);

                    cell.solid
                } else {
                    true
                }
            })
            .collect()
    }

    fn sync_entity(entity: &mut Entity) {
        Entity::set_world_position(entity.sense.touch.body.bottom_center(), entity);
    }
}
