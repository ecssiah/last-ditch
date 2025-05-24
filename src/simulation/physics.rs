use crate::simulation::{
    consts::*,
    physics::{aabb::AABB, dynamic_object::DynamicObject},
    population::Population,
    world::{grid, World},
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

    pub fn tick(&mut self, world: &World, population: &mut Population) {
        self.integrate(world, population);
    }

    fn integrate(&mut self, world: &World, population: &mut Population) {
        let judge = &mut population.judge;

        let initial_velocity = judge.velocity;
        let acceleration = self.gravity;

        let displacement = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;
        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        Self::resolve_dynamic_object(judge, world, &velocity, &displacement);
        Self::sync_dynamic_object(judge);
    }

    fn resolve_dynamic_object<T: DynamicObject>(
        dynamic_object: &mut T,
        world: &World,
        velocity_target: &Vec3,
        displacement: &Vec3,
    ) {
        let mut velocity = *velocity_target;
        let mut aabb = dynamic_object.aabb();

        {
            let dx = displacement.x;
            aabb.translate(Vec3::new(dx, 0.0, 0.0));

            let collisions = Physics::get_solid_collisions(&aabb, world);

            if !collisions.is_empty() {
                aabb.translate(Vec3::new(-dx, 0.0, 0.0));

                let step = dx.signum() * 0.01;
                let mut resolved = false;

                for i in 1..=MAX_RESOLVE_ITERATIONS {
                    let try_dx = step * (i as f32);

                    aabb.translate(Vec3::new(try_dx, 0.0, 0.0));

                    if Physics::get_solid_collisions(&aabb, world).is_empty() {
                        resolved = true;
                        break;
                    }

                    aabb.translate(Vec3::new(-try_dx, 0.0, 0.0));
                }

                if !resolved {
                    velocity.x = 0.0;
                }
            }
        }

        {
            let dy = displacement.y;
            aabb.translate(Vec3::new(0.0, dy, 0.0));

            let collisions = Physics::get_solid_collisions(&aabb, world);

            if !collisions.is_empty() {
                aabb.translate(Vec3::new(0.0, -dy, 0.0));

                let step = dy.signum() * 0.01;
                let mut resolved = false;

                for i in 1..=MAX_RESOLVE_ITERATIONS {
                    let try_dy = step * (i as f32);

                    aabb.translate(Vec3::new(0.0, try_dy, 0.0));

                    if Physics::get_solid_collisions(&aabb, world).is_empty() {
                        resolved = true;
                        break;
                    }

                    aabb.translate(Vec3::new(0.0, -try_dy, 0.0));
                }

                if !resolved {
                    velocity.y = 0.0;
                }
            }
        }

        {
            let dz = displacement.z;
            aabb.translate(Vec3::new(0.0, 0.0, dz));

            let collisions = Physics::get_solid_collisions(&aabb, world);

            if !collisions.is_empty() {
                aabb.translate(Vec3::new(0.0, 0.0, -dz));

                let step = dz.signum() * 0.01;
                let mut resolved = false;

                for i in 1..=MAX_RESOLVE_ITERATIONS {
                    let try_dz = step * (i as f32);

                    aabb.translate(Vec3::new(0.0, 0.0, try_dz));

                    if Physics::get_solid_collisions(&aabb, world).is_empty() {
                        resolved = true;
                        break;
                    }

                    aabb.translate(Vec3::new(0.0, 0.0, -try_dz));
                }

                if !resolved {
                    velocity.z = 0.0;
                }
            }
        }

        dynamic_object.set_velocity(velocity);
        dynamic_object.set_aabb(aabb);
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

    fn get_solid_collisions(target: &AABB, world: &World) -> Vec<AABB> {
        grid::overlapping_aabb_list(target)
            .into_iter()
            .filter(|block_aabb| {
                let block_position = block_aabb.center().as_ivec3();

                world.get_block(block_position).map_or(false, |b| b.solid)
            })
            .collect()
    }
}
