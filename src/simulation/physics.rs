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
        let acceleration = 0.0; // self.gravity;

        let displacement = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;
        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        Self::resolve_dynamic_object(judge, world, &velocity, &displacement);
        Self::sync_dynamic_object(judge);
    }

    fn resolve_dynamic_object<T: DynamicObject>(
        dynamic_object: &mut T,
        _world: &World,
        velocity_target: &Vec3,
        displacement: &Vec3,
    ) {
        let mut aabb = dynamic_object.aabb();
        aabb.set_bottom_center(aabb.bottom_center() + *displacement);

        dynamic_object.set_aabb(aabb);
        dynamic_object.set_velocity(*velocity_target);
    }

    fn sync_dynamic_object<T: DynamicObject>(dynamic_object: &mut T) {
        let position = dynamic_object.aabb().bottom_center();

        if let Some(chunk_id) = grid::get_chunk_id_at_world_position(position) {
            let chunk_update = chunk_id != dynamic_object.chunk_id();

            dynamic_object.set_chunk_update(chunk_update);
            dynamic_object.set_position(position);
        } else {
            let chunk_update = true;

            dynamic_object.set_chunk_update(chunk_update);
            dynamic_object.set_position(Vec3::new(0.0, 10.0, 0.0));
        }
    }

    fn _get_solid_collisions(target: &AABB, world: &World) -> Vec<AABB> {
        grid::get_overlapping_aabb_list(target)
            .into_iter()
            .filter(|block_aabb| {
                let block_position = block_aabb.center().as_ivec3();

                world.get_block(block_position).map_or(false, |b| b.solid)
            })
            .collect()
    }
}
