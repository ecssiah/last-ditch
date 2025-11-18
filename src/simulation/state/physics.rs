//! Forces affecting Population

pub mod aabb;

use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{entity::Judge, Population},
        world::grid::{self, Grid},
        World,
    },
};
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
        let (velocity, delta) = Physics::integrate_judge(physics.gravity, &mut population.judge);

        Self::resolve_judge(&mut population.judge, world, &velocity, &delta);
        Self::sync_judge(&mut population.judge);
    }

    fn integrate_judge(gravity: Vec3, judge: &mut Judge) -> (Vec3, Vec3) {
        let initial_velocity = judge.kinematic.velocity;
        let acceleration = judge.kinematic.acceleration + gravity;

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        let delta = 
            initial_velocity * SIMULATION_TICK_IN_SECONDS + 
            0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta)
    }

    fn resolve_judge(judge: &mut Judge, world: &World, velocity: &Vec3, delta: &Vec3) {
        let mut aabb = judge.detection.body;
        let mut velocity = *velocity;

        for axis in [grid::Axis::Y, grid::Axis::X, grid::Axis::Z] {
            let axis_index = axis as usize;
            let axis_delta = delta[axis_index];

            let (resolved_aabb, step) = Self::resolve_axis(aabb, world, axis, axis_delta);

            aabb = resolved_aabb;
            velocity[axis_index] = step / SIMULATION_TICK_IN_SECONDS;
        }

        judge.detection.body = aabb;
        judge.kinematic.velocity = velocity;
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

    fn get_solid_collisions(aabb: AABB, world: &World) -> Vec<AABB> {
        Grid::cells_overlapping(&world.grid, aabb)
            .into_iter()
            .filter(|cell_aabb| {
                let aabb_center = cell_aabb.center();
                
                let cell_position = IVec3::new(
                    aabb_center.x.round() as i32,
                    aabb_center.y.round() as i32,
                    aabb_center.z.round() as i32,
                );

                if let Some(cell) =
                    World::get_cell_at(cell_position, &world.grid, &world.sector_vec)
                {
                    cell.solid
                } else {
                    true
                }
            })
            .collect()
    }

    fn sync_judge(judge: &mut Judge) {
        Judge::set_world_position(
            judge.detection.body.bottom_center(),
            &mut judge.spatial,
            &mut judge.detection,
        );
    }
}
