//! Physics responses

pub mod body;
pub mod collider;

use crate::{
    simulation::{
        constants::*,
        state::{
            physics::body::{contact_set::ContactSet, Body},
            population::{motion, person::Person, Population},
            world::grid::{self, axis::Axis},
            World,
        },
    },
    utils::ldmath::FloatBox,
};
use std::f32;
use ultraviolet::Vec3;

#[derive(Default)]
pub struct Physics {
    pub active: bool,
    pub gravity: Vec3,
}

impl Physics {
    pub fn new() -> Self {
        let active = false;
        let gravity = Vec3::new(0.0, 0.0, -GRAVITY_ACCELERATION);

        Self { active, gravity }
    }

    pub fn tick(world: &World, population: &mut Population, physics: &mut Self) {
        let _span = tracing::info_span!("physics_tick").entered();

        if let Some(judge) = population.person_map.get_mut(&ID_JUDGE_1) {
            let (velocity, delta_position_intent) = Self::integrate_person(physics, judge);

            let (delta_position_resolved, velocity_mask) =
                Self::compute_resolution_person(&delta_position_intent, world, judge);

            let world_position = judge.transform.world_position + delta_position_resolved;
            let velocity = velocity_mask * velocity;

            Person::set_world_position(world_position, judge);
            Person::set_velocity(velocity, judge);

            let ground_collider = Body::get_collider(collider::Label::Ground, &judge.body)
                .expect("Body is missing ground");

            let ground_hit_vec =
                Self::get_hit_vec(&ground_collider.float_box, Vec3::new(0.0, 0.0, 1.0), world);

            if ground_hit_vec
                .iter()
                .any(|hit| hit.collider_kind == collider::Kind::Solid)
            {
                ContactSet::insert(body::Contact::Ground, &mut judge.body.contact_set);
            }

            let base_collider = Body::get_collider(collider::Label::Base, &judge.body)
                .expect("Base is missing ground");

            let base_hit_vec =
                Self::get_hit_vec(&base_collider.float_box, Vec3::new(0.0, 0.0, 1.0), world);

            if base_hit_vec
                .iter()
                .any(|hit| hit.collider_kind == collider::Kind::StairsNorth)
            {
                ContactSet::insert(body::Contact::StairsNorth, &mut judge.body.contact_set);
            } else if base_hit_vec
                .iter()
                .any(|hit| hit.collider_kind == collider::Kind::StairsWest)
            {
                ContactSet::insert(body::Contact::StairsWest, &mut judge.body.contact_set);
            } else if base_hit_vec
                .iter()
                .any(|hit| hit.collider_kind == collider::Kind::StairsSouth)
            {
                ContactSet::insert(body::Contact::StairsSouth, &mut judge.body.contact_set);
            } else if base_hit_vec
                .iter()
                .any(|hit| hit.collider_kind == collider::Kind::StairsEast)
            {
                ContactSet::insert(body::Contact::StairsEast, &mut judge.body.contact_set);
            }

            if judge.motion.mode == motion::Mode::Climb
                && !ContactSet::contains(body::Contact::Ladder, &judge.body.contact_set)
            {
                judge.motion.mode = motion::Mode::Ground;
            }
        }
    }

    fn integrate_person(physics: &Self, judge: &mut Person) -> (Vec3, Vec3) {
        let initial_velocity = judge.motion.velocity;

        let acceleration = if judge.motion.mode == motion::Mode::Ground {
            physics.gravity
        } else {
            Vec3::zero()
        };

        let velocity = initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        let delta_position_intent = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        (velocity, delta_position_intent)
    }

    fn compute_resolution_person(
        delta_position_intent: &Vec3,
        world: &World,
        person: &mut Person,
    ) -> (Vec3, Vec3) {
        ContactSet::clear(&mut person.body.contact_set);

        let core_collider =
            Body::get_collider(collider::Label::Core, &person.body).expect("Body is missing core");

        let mut core_float_box = core_collider.float_box.clone();

        let mut delta_position_resolved = Vec3::zero();
        let mut velocity_mask = Vec3::one();

        for delta_axis in [Axis::Z, Axis::X, Axis::Y] {
            let axis_index = Axis::index(delta_axis);

            let (delta_position_resolved_axis, velocity_mask_axis) =
                Self::compute_resolution_along_axis(
                    &core_float_box,
                    delta_axis,
                    delta_position_intent[axis_index],
                    world,
                    person,
                );

            delta_position_resolved[axis_index] = delta_position_resolved_axis;
            velocity_mask[axis_index] = velocity_mask_axis;

            core_float_box = FloatBox::translated(
                Axis::unit(delta_axis) * delta_position_resolved,
                &core_float_box,
            );
        }

        (delta_position_resolved, velocity_mask)
    }

    fn compute_resolution_along_axis(
        float_box: &FloatBox,
        delta_axis: Axis,
        delta_position_intent: f32,
        world: &World,
        person: &mut Person,
    ) -> (f32, f32) {
        let mut delta_position_resolved = 0.0;

        let mut t_min = 0.0;
        let mut t_max = 1.0;

        let delta_axis_unit = Axis::unit(delta_axis);
        let delta_position_intent_sign = delta_position_intent.signum();

        let normal = delta_axis_unit * -delta_position_intent_sign;

        for _ in 0..COLLISION_RESOLVE_ITERATIONS {
            let t_mid = (t_min + t_max) * 0.5;

            let delta_position_test = delta_position_intent * t_mid;

            let float_box_test =
                FloatBox::translated(delta_axis_unit * delta_position_test, float_box);

            let hit_vec = Self::get_hit_vec(&float_box_test, normal, world);

            if hit_vec
                .iter()
                .any(|hit| hit.collider_kind == collider::Kind::Solid)
            {
                t_max = t_mid;
            } else {
                t_min = t_mid;
                delta_position_resolved = delta_position_test;
            }

            if hit_vec
                .iter()
                .any(|hit| hit.collider_kind == collider::Kind::Ladder)
            {
                ContactSet::insert(body::Contact::Ladder, &mut person.body.contact_set);
            }
        }

        let collision_occurred =
            (delta_position_resolved - delta_position_intent).abs() > COLLISION_EPSILON;

        let velocity_mask = if collision_occurred {
            let separation_bias = COLLISION_EPSILON * delta_position_intent_sign;

            delta_position_resolved -= separation_bias;

            0.0
        } else {
            1.0
        };

        (delta_position_resolved, velocity_mask)
    }

    fn get_hit_vec(float_box: &FloatBox, normal: Vec3, world: &World) -> Vec<collider::Hit> {
        let overlap_grid_positions = grid::get_grid_overlap_vec(float_box);

        let mut hit_vec = Vec::with_capacity(overlap_grid_positions.len());

        for grid_position in overlap_grid_positions {
            let cell = World::get_cell_at(grid_position, &world.sector_vec);

            if let Some(block) = &cell.block {
                if block.solid {
                    let contact_point = Vec3::from(grid_position);

                    let cell_hit = collider::Hit {
                        collider_kind: collider::Kind::Solid,
                        contact_point,
                        normal,
                    };

                    hit_vec.push(cell_hit);
                }
            }

            if let Some(object) = &cell.object {
                if object.collider.active {
                    let contact_point = Vec3::from(grid_position);

                    let object_hit = collider::Hit {
                        collider_kind: object.collider.collider_kind,
                        contact_point,
                        normal,
                    };

                    hit_vec.push(object_hit);
                }
            }
        }

        hit_vec
    }
}
