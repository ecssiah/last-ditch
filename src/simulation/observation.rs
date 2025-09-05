//! Exposes Simulation data for Interface

pub mod view;

use crate::simulation::{
    consts::JUDGE_VIEW_RADIUS_SQUARED,
    observation::view::{
        AdminView, AgentView, ChunkView, JudgeView, PopulationView, TimeView, View, WorldView,
    },
    state::{world::grid::Grid, State},
};
use glam::Vec3;
use std::collections::HashMap;

pub struct Observation {}

impl Observation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(state: &State, view_input_buffer: &mut triple_buffer::Input<View>) {
        Observation::update_view(state, view_input_buffer);
    }

    pub fn get_view(view_output_buffer: &mut triple_buffer::Output<View>) -> &View {
        view_output_buffer.update();

        let view = view_output_buffer.peek_output_buffer();

        &view
    }

    fn update_view(state: &State, view_input_buffer: &mut triple_buffer::Input<View>) {
        let admin_view = Self::update_admin_view(state);
        let time_view = Self::update_time_view(state);
        let population_view = Self::update_population_view(state);
        let world_view = Self::update_world_view(state);

        let view = view_input_buffer.input_buffer_mut();

        view.entity_id = state.population.judge.info.entity_id;
        view.admin_view = admin_view;
        view.time_view = time_view;
        view.population_view = population_view;
        view.world_view = world_view;

        view_input_buffer.publish();
    }

    fn update_admin_view(state: &State) -> AdminView {
        AdminView {
            mode: state.admin.mode,
            message: state.admin.message.clone(),
        }
    }

    fn update_time_view(state: &State) -> TimeView {
        TimeView {
            instant: state.time.instant,
        }
    }

    fn update_population_view(state: &State) -> PopulationView {
        let judge = &state.population.judge;

        let mut population_view = PopulationView {
            judge_view: JudgeView {
                entity_id: judge.info.entity_id,
                position: Grid::world_to_position(&state.world.grid, judge.spatial.world_position),
                world_position: judge.spatial.world_position,
                chunk_id: judge.info.chunk_id,
                chunk_coordinates: Grid::chunk_id_to_chunk_coordinates(
                    &state.world.grid,
                    judge.info.chunk_id,
                ),
                size: judge.spatial.size,
                quaternion: judge.spatial.quaternion,
                eye: judge.spatial.eye(),
                view_ray_vec: judge.sight.view_ray_vec.clone(),
            },
            agent_view_map: HashMap::new(),
        };

        for agent in state.population.agent_map.values() {
            let distance_to_judge_squared = (agent.spatial.world_position
                - state.population.judge.spatial.world_position)
                .length_squared();

            if distance_to_judge_squared > JUDGE_VIEW_RADIUS_SQUARED {
                continue;
            }

            let agent_view = AgentView {
                id: agent.info.entity_id,
                entity_kind: agent.info.entity_kind,
                nation_kind: agent.info.nation_kind,
                spatial: agent.spatial,
                kinematic: agent.kinematic,
                detection: agent.detection,
            };

            population_view
                .agent_view_map
                .insert(agent.info.entity_id, agent_view);
        }

        population_view
    }

    fn update_world_view(state: &State) -> WorldView {
        let mut world_view = WorldView {
            grid: state.world.grid,
            chunk_view_map: HashMap::new(),
        };

        for chunk_id in &state.population.judge.sight.chunk_id_set {
            if let Some(chunk) = state.world.chunk_vec.get(usize::from(*chunk_id)) {
                let chunk_view = ChunkView {
                    id: chunk.id,
                    world_position: chunk.position.as_vec3(),
                    extent: Vec3::splat(state.world.grid.chunk_extent_units),
                    block_vec: chunk.block_vec.clone(),
                };

                world_view.chunk_view_map.insert(chunk.id, chunk_view);
            }
        }

        world_view
    }
}
