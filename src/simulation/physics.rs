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
        self.resolve(world, population);
    }

    fn integrate(&mut self, _world: &World, population: &mut Population) {
        let judge = &mut population.judge;

        let initial_velocity = judge.velocity;
        let acceleration = 0.0;
        let delta_time = SIMULATION_TICK_IN_SECONDS;

        let displacement = initial_velocity * delta_time + 0.5 * acceleration * delta_time.powi(2);
        judge.velocity = initial_velocity + acceleration * delta_time;

        let aabb_center = judge.aabb.center() + displacement;
        judge.aabb = AABB::new(aabb_center, judge.size);
    }

    fn resolve(&self, world: &World, population: &mut Population) {
        let synced_aabb = Self::resolve_dynamic(&population.judge, world);

        population.judge.set_aabb(
            synced_aabb.center().x,
            synced_aabb.center().y,
            synced_aabb.center().z,
        );

        Self::sync_judge(&mut population.judge);
    }

    fn resolve_dynamic<T: Dynamic>(dynamic_entity: &T, world: &World) -> AABB {
        let aabb = dynamic_entity.aabb();

        aabb
    }

    fn sync_judge(judge: &mut Judge) {
        if let Some(chunk_id) = grid::get_chunk_id_at(judge.aabb.center()) {
            judge.chunk_update = chunk_id != judge.chunk_id;
            judge.position = judge.aabb.center() - Vec3::Y * (judge.size.y * 0.5);
        } else {
            judge.chunk_update = true;
            judge.set_position(0.0, 10.0, 0.0);
        }
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
