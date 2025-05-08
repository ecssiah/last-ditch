use crate::simulation::{
    consts::*,
    physics::aabb::AABB,
    population::{Judge, Population},
    world::{grid, World},
};
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
        let judge_aabb = population.judge.aabb.clone();
        let synced_aabb = Self::resolve_aabb(&judge_aabb, world);

        population.judge.set_aabb(
            synced_aabb.center().x,
            synced_aabb.center().y,
            synced_aabb.center().z,
        );

        Self::sync_judge(&mut population.judge);
    }

    fn resolve_aabb(aabb: &AABB, world: &World) -> AABB {
        let mut synced_aabb = aabb.clone();
        let collisions = Self::get_solid_collisions(&aabb, world);

        for block_aabb in collisions {
            let dx1 = block_aabb.max.x - synced_aabb.min.x;
            let dx2 = block_aabb.min.x - synced_aabb.max.x;
            let dy1 = block_aabb.max.y - synced_aabb.min.y;
            let dy2 = block_aabb.min.y - synced_aabb.max.y;
            let dz1 = block_aabb.max.z - synced_aabb.min.z;
            let dz2 = block_aabb.min.z - synced_aabb.max.z;

            let overlap_x = if dx1.abs() < dx2.abs() { dx1 } else { dx2 };
            let overlap_y = if dy1.abs() < dy2.abs() { dy1 } else { dy2 };
            let overlap_z = if dz1.abs() < dz2.abs() { dz1 } else { dz2 };

            let min_overlap = [overlap_x, overlap_y, overlap_z]
                .into_iter()
                .min_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())
                .unwrap();

            if min_overlap == overlap_x {
                synced_aabb.min.x += overlap_x;
                synced_aabb.max.x += overlap_x;
            } else if min_overlap == overlap_y {
                synced_aabb.min.y += overlap_y;
                synced_aabb.max.y += overlap_y;
            } else {
                synced_aabb.min.z += overlap_z;
                synced_aabb.max.z += overlap_z;
            }
        }

        synced_aabb
    }

    fn sync_judge(judge: &mut Judge) {
        if let Some(grid_position) = grid::world_to_grid(judge.aabb.center()) {
            if let Some(chunk_id) = grid::get_chunk_id(grid_position) {
                if chunk_id != judge.chunk_id {
                    judge.chunk_update = true;
                }

                judge.position = judge.aabb.center() - Vec3::Y * (judge.size.y * 0.5);
            }
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
