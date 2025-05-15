use crate::simulation::{
    consts::*,
    physics::{aabb::AABB, dynamic::Dynamic},
    population::{Judge, Population},
    world::{grid, World},
};
use glam::Vec3;

pub mod aabb;
pub mod dynamic;
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

    pub fn tick(&mut self, world: &World, population: &mut Population) {
        self.integrate(world, population);
    }

    fn integrate(&mut self, world: &World, population: &mut Population) {
        let judge = &mut population.judge;

        let initial_velocity = judge.velocity;
        let acceleration = 0.0;
        let delta_time = SIMULATION_TICK_IN_SECONDS;

        let displacement = initial_velocity * delta_time + 0.5 * acceleration * delta_time.powi(2);

        judge.velocity = initial_velocity + acceleration * delta_time;

        Self::resolve_dynamic(world, &mut population.judge, &displacement);
        Self::sync_dynamic(&mut population.judge);
    }

    fn resolve_dynamic<T: Dynamic>(world: &World, dynamic_entity: &mut T, displacement: &Vec3) {
        for axis in [grid::Axis::X, grid::Axis::Z, grid::Axis::Y] {
            let axis_displacement = displacement[axis as usize];

            
        }
    }

    fn sync_dynamic<T: Dynamic>(dynamic_entity: &mut T) {
        let (chunk_update, position) =
            if let Some(chunk_id) = grid::get_chunk_id_at(dynamic_entity.aabb().position()) {
                (
                    chunk_id != dynamic_entity.chunk_id(),
                    dynamic_entity.aabb().position() - Vec3::Y * (dynamic_entity.size().y * 0.5),
                )
            } else {
                (true, Vec3::new(0.0, 10.0, 0.0))
            };

        dynamic_entity.set_chunk_update(chunk_update);
        dynamic_entity.set_position(position);
    }

    fn get_solid_collisions(aabb: &AABB, world: &World) -> Vec<AABB> {
        let mut collisions = Vec::new();
        let block_size = Vec3::splat(BLOCK_SIZE);

        for block_aabb in Self::get_overlapping_aabb_list(aabb) {
            let block_position = block_aabb.center();

            if let Some(block) = world.get_block(block_position.as_ivec3()) {
                if block.solid {
                    let block_aabb = AABB::new(block_position, block_size);

                    collisions.push(block_aabb);
                }
            }
        }

        collisions
    }

    fn get_overlapping_aabb_list(target: &AABB) -> Vec<AABB> {
        let min = target.min.floor().as_ivec3();
        let max = target.max.floor().as_ivec3();

        let block_radius = Vec3::splat(BLOCK_RADIUS);
        let block_size = Vec3::splat(BLOCK_SIZE);

        let mut aabb_list = Vec::new();

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let center = Vec3::new(x as f32, y as f32, z as f32) + block_radius;
                    let block_aabb = AABB::new(center, block_size);

                    aabb_list.push(block_aabb);
                }
            }
        }

        aabb_list
    }
}
