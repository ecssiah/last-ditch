use crate::simulation::{consts::*, physics::aabb::AABB, population::Population, world::World};
use glam::Vec3;

pub mod aabb;
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
        let acceleration = self.gravity;
        let delta_time = SIMULATION_TICK_IN_SECONDS;

        let displacement = initial_velocity * delta_time + 0.5 * acceleration * delta_time.powi(2);
        judge.velocity = initial_velocity + acceleration * delta_time;

        let aabb_center = judge.aabb.center() + displacement;
        judge.aabb = AABB::new(aabb_center, judge.size);
    }

    fn resolve(&self, world: &World, population: &mut Population) {
        let judge = &mut population.judge;

        {
            let collisions = Self::get_solid_collisions(&mut judge.aabb, world);

            for block_aabb in collisions {
                if judge.aabb.min.y < block_aabb.max.y && judge.aabb.max.y > block_aabb.min.y {
                    if judge.velocity.y < 0.0
                        && judge.aabb.min.y < block_aabb.max.y
                        && judge.aabb.max.y > block_aabb.max.y
                    {
                        judge.velocity.y = 0.0;

                        judge.aabb.min.y = block_aabb.max.y + COLLISION_EPSILON;
                        judge.aabb.max.y = judge.aabb.min.y + judge.size.y;
                    } else if judge.velocity.y > 0.0
                        && judge.aabb.max.y > block_aabb.min.y
                        && judge.aabb.min.y < block_aabb.min.y
                    {
                        judge.velocity.y = 0.0;

                        judge.aabb.max.y = block_aabb.min.y - COLLISION_EPSILON;
                        judge.aabb.min.y = judge.aabb.max.y - judge.size.y;
                    }
                }
            }
        }

        {
            let collisions = Self::get_solid_collisions(&mut judge.aabb, world);

            for block_aabb in collisions {
                if judge.aabb.min.x < block_aabb.max.x && judge.aabb.max.x > block_aabb.min.x {
                    if judge.velocity.x < 0.0
                        && judge.aabb.min.x < block_aabb.max.x
                        && judge.aabb.max.x > block_aabb.max.x
                    {
                        judge.velocity.x = 0.0;

                        judge.aabb.min.x = block_aabb.max.x + COLLISION_EPSILON;
                        judge.aabb.max.x = judge.aabb.min.x + judge.size.x;
                    } else if judge.velocity.x > 0.0
                        && judge.aabb.max.x > block_aabb.min.x
                        && judge.aabb.min.x < block_aabb.min.x
                    {
                        judge.velocity.x = 0.0;

                        judge.aabb.max.x = block_aabb.min.x - COLLISION_EPSILON;
                        judge.aabb.min.x = judge.aabb.max.x - judge.size.x;
                    }
                }
            }
        }

        {
            let collisions = Self::get_solid_collisions(&mut judge.aabb, world);

            for block_aabb in collisions {
                if judge.aabb.min.z < block_aabb.max.z && judge.aabb.max.z > block_aabb.min.z {
                    if judge.velocity.z < 0.0
                        && judge.aabb.min.z < block_aabb.max.z
                        && judge.aabb.max.z > block_aabb.max.z
                    {
                        judge.velocity.z = 0.0;

                        judge.aabb.min.z = block_aabb.max.z + COLLISION_EPSILON;
                        judge.aabb.max.z = judge.aabb.min.z + judge.size.z;
                    } else if judge.velocity.z > 0.0
                        && judge.aabb.max.z > block_aabb.min.z
                        && judge.aabb.min.z < block_aabb.min.z
                    {
                        judge.velocity.z = 0.0;

                        judge.aabb.max.z = block_aabb.min.z - COLLISION_EPSILON;
                        judge.aabb.min.z = judge.aabb.max.z - judge.size.z;
                    }
                }
            }
        }

        judge.position = judge.aabb.center() - Vec3::Y * (judge.size.y * 0.5);
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
