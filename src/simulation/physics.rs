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
        self.integrate(world, population);

        Self::sync_dynamic_object(population.get_judge_mut());
    }

    fn integrate(&self, world: &World, population: &mut Population) {
        let judge = &mut population.judge;

        let initial_velocity = judge.velocity;
        let acceleration = self.gravity;

        let displacement = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;
        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        Self::resolve_dynamic_object(judge, world, &velocity, &displacement);
    }

    fn resolve_dynamic_object<T: DynamicObject>(
        dynamic_object: &mut T,
        world: &World,
        velocity_target: &Vec3,
        displacement: &Vec3,
    ) {
        let mut velocity = *velocity_target;
        let mut aabb = dynamic_object.aabb();

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let axis_index = axis as usize;

            let (resolved_aabb, step) =
                Self::resolve_axis(aabb, displacement[axis_index], axis, world);
            velocity[axis_index] = step / SIMULATION_TICK_IN_SECONDS;

            aabb = resolved_aabb;
        }

        dynamic_object.set_velocity(velocity);
        dynamic_object.set_aabb(aabb);
    }

    fn resolve_axis(aabb: AABB, delta: f32, axis: Axis, world: &World) -> (AABB, f32) {
        if delta.abs() < EPSILON_COLLISION {
            return (aabb, 0.0);
        }

        let mut min = 0.0;
        let mut max = delta;
        let mut final_delta = 0.0;

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let mid = (min + max) * 0.5;

            let test_aabb = aabb.translate(axis.unit() * mid);

            if Physics::get_solid_collisions(&test_aabb, world).is_empty() {
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
