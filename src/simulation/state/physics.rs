//! Forces affecting Population

pub mod aabb;
pub mod dynamic_object;

use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{entity::Judge, Population},
        world::grid,
        World,
    },
};
use glam::Vec3;

pub struct Physics {
    pub gravity: Vec3,
}

impl Physics {
    pub fn new() -> Self {
        let gravity = Vec3::new(0.0, -GRAVITY_ACCELERATION, 0.0);

        Self { gravity }
    }

    pub fn tick(&self, world: &World, population: &mut Population) {
        let judge = population.get_judge_mut();

        let (velocity, delta) = Self::integrate_judge(judge);

        Self::resolve_judge(judge, world, &velocity, &delta);
        Self::sync_judge(judge);
    }

    fn integrate_judge(judge: &mut Judge) -> (Vec3, Vec3) {
        let initial_velocity = judge.kinematics.velocity;
        let acceleration = judge.kinematics.acceleration;

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;
        let delta = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta)
    }

    fn resolve_judge(judge: &mut Judge, world: &World, velocity: &Vec3, delta: &Vec3) {
        let mut aabb = judge.spatial.aabb;
        let mut velocity = *velocity;

        for axis in [grid::Axis::Y, grid::Axis::X, grid::Axis::Z] {
            let axis_index = axis as usize;
            let axis_delta = delta[axis_index];

            let (resolved_aabb, step) = Self::resolve_axis(aabb, world, axis, axis_delta);

            aabb = resolved_aabb;
            velocity[axis_index] = step / SIMULATION_TICK_IN_SECONDS;
        }

        judge.spatial.aabb = aabb;
        judge.kinematics.velocity = velocity;
    }

    fn resolve_axis(aabb: AABB, world: &World, axis: grid::Axis, delta: f32) -> (AABB, f32) {
        if delta.abs() < EPSILON_COLLISION {
            return (aabb, 0.0);
        }

        let mut min = 0.0;
        let mut max = delta;
        let mut final_delta = 0.0;

        for _ in 0..MAX_RESOLVE_ITERATIONS {
            let mid = (min + max) * 0.5;
            let test_aabb = aabb.translate(axis.unit() * mid);

            if Self::get_solid_collisions(test_aabb, world).is_empty() {
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

    fn get_solid_collisions(target: AABB, world: &World) -> Vec<AABB> {
        world
            .grid
            .blocks_overlapping(target)
            .into_iter()
            .filter(|block_aabb| {
                let block_position = block_aabb.center().as_ivec3();

                world
                    .get_block_at(block_position)
                    .map_or(false, |b| b.solid)
            })
            .collect()
    }

    fn sync_judge(judge_data: &mut Judge) {
        let world_position = judge_data.spatial.aabb.bottom_center();

        judge_data.spatial.world_position = world_position;
    }
}
