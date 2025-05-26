use crate::simulation::{
    consts::*,
    physics::{aabb::AABB, dynamic_object::DynamicObject},
    population::Population,
    world::{
        grid::{self, Axis},
        World,
    },
};
use glam::Vec3;

pub mod aabb;
pub mod dynamic_object;
pub mod judge_controller;

pub struct Physics {
    pub gravity: Vec3,
}

impl Physics {
    pub fn new() -> Physics {
        let gravity = Vec3::new(0.0, -GRAVITY_ACCELERATION, 0.0);

        let physics = Self { gravity };

        physics
    }

    pub fn tick(&self, world: &World, population: &mut Population) {
        let judge = population.get_judge_mut();

        let (velocity, delta) = Self::integrate_dynamic_object(judge);

        Self::resolve_dynamic_object(judge, world, &velocity, &delta);
        Self::sync_dynamic_object(judge);
    }

    fn integrate_dynamic_object<T: DynamicObject>(dynamic_object: &mut T) -> (Vec3, Vec3) {
        let initial_velocity = dynamic_object.velocity();
        let acceleration = dynamic_object.acceleration();

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;
        let delta = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta)
    }

    fn resolve_dynamic_object<T: DynamicObject>(
        dynamic_object: &mut T,
        world: &World,
        velocity: &Vec3,
        delta: &Vec3,
    ) {
        let mut aabb = dynamic_object.aabb();
        let mut velocity = *velocity;

        for axis in [Axis::Y, Axis::X, Axis::Z] {
            let axis_index = axis as usize;
            let axis_delta = delta[axis_index];

            let (resolved_aabb, step) = Self::resolve_axis(aabb, world, axis, axis_delta);

            aabb = resolved_aabb;
            velocity[axis_index] = step / SIMULATION_TICK_IN_SECONDS;
        }

        dynamic_object.set_aabb(aabb);
        dynamic_object.set_velocity(velocity);
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
            let test_aabb = aabb.translate(axis.unit() * mid);

            if Self::get_solid_collisions(&test_aabb, world).is_empty() {
                final_delta = mid;
                min = mid;
            } else {
                max = mid;
            }
        }

        let adjusted_aabb = aabb.translate(axis.unit() * final_delta);

        let adjusted_velocity = if (final_delta - delta).abs() > 0.0001 {
            0.0
        } else {
            final_delta
        };

        (adjusted_aabb, adjusted_velocity)
    }

    fn get_solid_collisions(target: &AABB, world: &World) -> Vec<AABB> {
        grid::overlapping_aabb_list(target)
            .into_iter()
            .filter(|block_aabb| {
                let block_position = block_aabb.center().as_ivec3();

                world.get_block(block_position).map_or(false, |b| b.solid)
            })
            .collect()
    }

    fn sync_dynamic_object<T: DynamicObject>(dynamic_object: &mut T) {
        let position = dynamic_object.aabb().bottom_center();

        if let Some(chunk_id) = grid::world_to_chunk_id(position) {
            let chunk_update = chunk_id != dynamic_object.chunk_id();

            dynamic_object.set_chunk_update(chunk_update);
            dynamic_object.set_position(position);
        } else {
            let chunk_update = true;

            dynamic_object.set_chunk_update(chunk_update);
            dynamic_object.set_position(Vec3::new(0.0, 10.0, 0.0));
        }
    }
}
