use crate::simulation::{
    consts::*, physics::bounding_box::BoundingBox, population::Population, world::World,
};
use glam::{IVec3, Vec3};

pub mod bounding_box;
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

    fn integrate(&mut self, world: &World, population: &mut Population) {
        let dt = SIMULATION_TICK_DURATION.as_secs_f32();
        let judge = &mut population.judge;

        judge.velocity += dt * self.gravity;

        let offset = dt * judge.velocity;
        let center = judge.bounding_box.center() + offset;

        judge.bounding_box = BoundingBox::new(center, judge.size);
    }

    fn resolve(&mut self, world: &World, population: &mut Population) {
        let judge = &mut population.judge;

        let bounding_box = &mut judge.bounding_box;
        let mut velocity = judge.velocity;

        let collisions = self.collides_with_world(bounding_box, world);

        for block_pos in collisions {
            if bounding_box.min.y < (block_pos.y as f32 + 1.0)
                && bounding_box.max.y > (block_pos.y as f32)
            {
                if velocity.y < 0.0 {
                    bounding_box.min.y = block_pos.y as f32 + 1.0;
                    bounding_box.max.y = bounding_box.min.y + judge.size.y;
                    velocity.y = 0.0;
                } else if velocity.y > 0.0 {
                    bounding_box.max.y = block_pos.y as f32;
                    bounding_box.min.y = bounding_box.max.y - judge.size.y;
                    velocity.y = 0.0;
                }
            }

            if bounding_box.min.x < (block_pos.x as f32 + 1.0)
                && bounding_box.max.x > (block_pos.x as f32)
            {
                if velocity.x < 0.0 {
                    bounding_box.min.x = block_pos.x as f32 + 1.0;
                    bounding_box.max.x = bounding_box.min.x + judge.size.x;
                    velocity.x = 0.0;
                } else if velocity.x > 0.0 {
                    bounding_box.max.x = block_pos.x as f32;
                    bounding_box.min.x = bounding_box.max.x - judge.size.x;
                    velocity.x = 0.0;
                }
            }

            if bounding_box.min.z < (block_pos.z as f32 + 1.0)
                && bounding_box.max.z > (block_pos.z as f32)
            {
                if velocity.z < 0.0 {
                    bounding_box.min.z = block_pos.z as f32 + 1.0;
                    bounding_box.max.z = bounding_box.min.z + judge.size.z;
                    velocity.z = 0.0;
                } else if velocity.z > 0.0 {
                    bounding_box.max.z = block_pos.z as f32;
                    bounding_box.min.z = bounding_box.max.z - judge.size.z;
                    velocity.z = 0.0;
                }
            }
        }

        judge.velocity = velocity;
        judge.position = bounding_box.center() - Vec3::Y * (judge.size.y * 0.5);
    }

    fn collides_with_world(&self, aabb: &BoundingBox, world: &World) -> Vec<IVec3> {
        let mut collisions = Vec::new();

        for block_pos in aabb.blocks_overlapping() {
            if let Some(block) = world.get_block(block_pos) {
                if block.solid {
                    collisions.push(block_pos);
                }
            }
        }

        collisions
    }
}
