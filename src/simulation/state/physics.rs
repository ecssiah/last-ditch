//! Physics responses

pub mod axis_resolution;
pub mod body;
pub mod collider;
pub mod collision_shape;
pub mod hit;
pub mod integration_result;
pub mod resolution_result;

use crate::{
    simulation::{
        constants::*,
        state::{
            physics::{
                axis_resolution::AxisResolution,
                body::{body_label::BodyLabel, contact_set::ContactSet, Body},
                collider::ColliderKind,
                hit::Hit,
                integration_result::IntegrationResult,
                resolution_result::ResolutionResult,
            },
            population::{
                motion,
                person::{person_id::PersonID, Person},
                Population,
            },
            world::{
                block::{block_kind::BlockKind, block_state::block_type::BlockType},
                grid::{self, axis::Axis},
            },
            World,
        },
    },
    utils::ldmath::{float_ext, FloatBox},
};
use std::f32;
use tracing::instrument;
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

    #[instrument(skip_all)]
    pub fn tick(world: &World, population: &mut Population, physics: &mut Self) {
        if let Some(judge) = population.person_map.get_mut(&PersonID::JUDGE_ID_1) {
            let integration_result = Self::integrate_person(physics, judge);

            let resolution_result =
                Self::compute_resolution_person(&integration_result.delta_intent, world, judge);

            let world_position = judge.transform.world_position + resolution_result.delta_resolved;
            let velocity = integration_result.velocity_intent * resolution_result.velocity_mask;

            Person::set_world_position(world_position, judge);
            Person::set_velocity(velocity, judge);

            Self::update_contact(world, judge);
        }
    }

    fn integrate_person(physics: &Self, judge: &mut Person) -> IntegrationResult {
        let mut integration_result = IntegrationResult::new();

        let initial_velocity = judge.motion.velocity;

        let acceleration = if judge.motion.mode == motion::Mode::Ground {
            physics.gravity
        } else {
            Vec3::zero()
        };

        integration_result.velocity_intent =
            initial_velocity + acceleration * SIMULATION_TICK_IN_SECONDS;

        integration_result.delta_intent = initial_velocity * SIMULATION_TICK_IN_SECONDS
            + 0.5 * acceleration * SIMULATION_TICK_IN_SECONDS_SQUARED;

        integration_result
    }

    fn compute_resolution_person(
        delta_intent: &Vec3,
        world: &World,
        person: &mut Person,
    ) -> ResolutionResult {
        ContactSet::clear(&mut person.body.contact_set);

        let mut core_float_box = Body::get_collider(&BodyLabel::Core, &person.body)
            .expect("Body is missing core")
            .clone()
            .float_box;

        let mut resolution_result = ResolutionResult::new();

        for delta_axis in [Axis::Z, Axis::X, Axis::Y] {
            let axis_index = Axis::index(delta_axis);

            let axis_resolution = Self::compute_axis_resolution(
                &core_float_box,
                delta_axis,
                delta_intent[axis_index],
                world,
            );

            resolution_result.delta_resolved[axis_index] = axis_resolution.delta_resolved;
            resolution_result.velocity_mask[axis_index] = axis_resolution.velocity_mask;

            core_float_box = FloatBox::translated(
                Axis::unit(delta_axis) * resolution_result.delta_resolved,
                &core_float_box,
            );
        }

        resolution_result
    }

    fn compute_axis_resolution(
        float_box: &FloatBox,
        delta_axis: Axis,
        delta_intent: f32,
        world: &World,
    ) -> AxisResolution {
        let mut axis_resolution = AxisResolution::new();

        let mut t_min = 0.0;
        let mut t_max = 1.0;

        let delta_axis_unit = Axis::unit(delta_axis);
        let delta_intent_sign = delta_intent.signum();

        // let normal = delta_axis_unit * -delta_intent_sign;

        for _ in 0..COLLISION_RESOLVE_ITERATIONS {
            let t_mid = 0.5 * (t_min + t_max);

            let delta_translated = delta_intent * t_mid;

            let float_box_translated =
                FloatBox::translated(delta_axis_unit * delta_translated, float_box);

            let hit_vec = Self::get_hit_vec(&float_box_translated, world);

            if hit_vec
                .iter()
                .any(|hit| hit.collider_kind == ColliderKind::Solid)
            {
                t_max = t_mid;
            } else {
                t_min = t_mid;
                axis_resolution.delta_resolved = delta_translated;
            }
        }

        let collision_occurred = float_ext::not_equal(
            axis_resolution.delta_resolved,
            delta_intent,
            COLLISION_EPSILON,
        );

        if collision_occurred {
            let separation_bias = COLLISION_EPSILON * delta_intent_sign;

            axis_resolution.delta_resolved -= separation_bias;
            axis_resolution.velocity_mask = 0.0;
        } else {
            axis_resolution.velocity_mask = 1.0;
        };

        axis_resolution
    }

    fn update_contact(world: &World, judge: &mut Person) {
        let ground_float_box = Body::get_collider(&BodyLabel::Ground, &judge.body)
            .expect("Body is missing ground")
            .clone()
            .float_box;

        let ground_hit_vec = Self::get_hit_vec(&ground_float_box, world);

        if ground_hit_vec
            .iter()
            .any(|hit| hit.collider_kind == ColliderKind::Solid)
        {
            ContactSet::add(body::Contact::Ground, &mut judge.body.contact_set);
        }

        if judge.motion.mode == motion::Mode::Climb
            && !ContactSet::has(body::Contact::Ladder, &judge.body.contact_set)
        {
            judge.motion.mode = motion::Mode::Ground;
        }
    }

    fn get_hit_vec(float_box: &FloatBox, world: &World) -> Vec<Hit> {
        let overlap_grid_positions = grid::get_float_box_grid_overlap_vec(float_box);

        let mut hit_vec = Vec::with_capacity(overlap_grid_positions.len());

        for grid_position in overlap_grid_positions {
            if let Some(block) = World::get_block(grid_position, &world.sector_vec) {
                match block.block_state.block_type {
                    BlockType::Block => {
                        let contact_point = Vec3::from(grid_position);

                        let hit = Hit {
                            collider_kind: ColliderKind::Solid,
                            contact_point,
                        };

                        hit_vec.push(hit);
                    }
                    _ => {
                        let block_float_box_array = BlockKind::get_collider_shape_array(
                            &block.block_state,
                            &block.block_kind,
                        );

                        for block_float_box in block_float_box_array {
                            if FloatBox::overlap(float_box, block_float_box) {
                                let contact_point = Vec3::from(grid_position);

                                let hit = Hit {
                                    collider_kind: ColliderKind::Solid,
                                    contact_point,
                                };

                                hit_vec.push(hit);
                            }
                        }
                    }
                }
            }
        }

        hit_vec
    }
}
