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
        let delta_time = SIMULATION_TICK_IN_SECONDS;

        let displacement = initial_velocity * delta_time + 0.5 * acceleration * delta_time.powi(2);
        let velocity = initial_velocity + acceleration * delta_time;

        Self::resolve_dynamic_object(judge, world, &velocity, &displacement);
        Self::sync_dynamic_object(judge);
    }

    fn resolve_dynamic_object<T: DynamicObject>(
        dynamic_object: &mut T,
        world: &World,
        velocity_target: &Vec3,
        displacement: &Vec3,
    ) {
        let mut aabb = dynamic_object.aabb();
        let mut velocity = velocity_target.clone();

        for axis in [grid::Axis::X, grid::Axis::Z, grid::Axis::Y] {
            let axis_index = axis as usize;
            let axis_displacement = displacement[axis_index];

            aabb.set_center(aabb.center() + axis_displacement * axis.unit());

            let solid_block_aabbs: Vec<AABB> = Self::get_solid_collisions(&aabb, world);

            for block_aabb in solid_block_aabbs {
                let block_overlap = aabb.get_overlap(axis_index, &block_aabb);

                if block_overlap.abs() > EPSILON_COLLISION {
                    aabb.set_center(aabb.center() + block_overlap * axis.unit());
                    
                    velocity[axis_index] = 0.0;
                }
            }
        }

        dynamic_object.set_aabb(aabb);
        dynamic_object.set_velocity(velocity);
    }

    fn sync_dynamic_object<T: DynamicObject>(dynamic_object: &mut T) {
        if let Some(chunk_id) = grid::get_chunk_id_at(dynamic_object.aabb().bottom_center()) {
            let chunk_update = chunk_id != dynamic_object.chunk_id();
            let position = dynamic_object.aabb().bottom_center();

            dynamic_object.set_chunk_update(chunk_update);
            dynamic_object.set_position(position);
        } else {
            let chunk_update = true;
            let position = Vec3::new(0.0, 10.0, 0.0);

            dynamic_object.set_chunk_update(chunk_update);
            dynamic_object.set_position(position);
        }
    }

    fn get_solid_collisions(target: &AABB, world: &World) -> Vec<AABB> {
        grid::get_overlapping_aabb_list(target)
            .into_iter()
            .filter(|block_aabb| {
                let block_position = block_aabb.center().as_ivec3();

                world.get_block(block_position).map_or(false, |b| b.solid)
            })
            .collect()
    }
}
