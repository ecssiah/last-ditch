//! Simulation View producer

pub mod face_mask;
pub mod view;

pub use view::AgentView;
pub use view::BlockView;
pub use view::FaceView;
pub use view::JudgeView;
pub use view::PopulationView;
pub use view::SectorView;
pub use view::View;
pub use view::WorldView;

use crate::simulation::constants::SECTOR_RADIUS_IN_CELLS;
use crate::simulation::constants::SECTOR_VOLUME_IN_CELLS;
use crate::simulation::{
    manager::{viewer::view::ManagerView, Manager},
    state::{
        world::{
            block,
            grid::{self},
            sector::Sector,
        },
        State,
    },
};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct Viewer {
    pub view_input: triple_buffer::Input<View>,
    pub sector_version_map: HashMap<usize, u64>,
    pub block_view_cache: HashMap<usize, Vec<Option<BlockView>>>,
}

impl Viewer {
    pub fn new(view_input: triple_buffer::Input<View>) -> Self {
        Self {
            view_input,
            sector_version_map: HashMap::new(),
            block_view_cache: HashMap::new(),
        }
    }

    pub fn tick(manager: &mut Manager, state: &State) {
        let _ = tracing::info_span!("viewer_tick").entered();

        let manager_view = Self::update_manager_view(manager);

        let population_view = Self::update_population_view(state);

        let world_view = Self::update_world_view(
            state,
            &mut manager.viewer.sector_version_map,
            &mut manager.viewer.block_view_cache,
        );

        let view = manager.viewer.view_input.input_buffer_mut();

        view.manager_view = manager_view;
        view.population_view = population_view;
        view.world_view = world_view;

        manager.viewer.view_input.publish();
    }

    pub fn get_view(view_output: &mut triple_buffer::Output<View>) -> &View {
        view_output.update();

        let view = view_output.peek_output_buffer();

        &view
    }

    fn update_manager_view(manager: &Manager) -> ManagerView {
        let manager_view = ManagerView {
            status: manager.status,
        };

        manager_view
    }

    fn update_population_view(state: &State) -> PopulationView {
        let judge = &state.population.judge;

        let judge_view = JudgeView {
            position: grid::world_position_to_grid_position(judge.spatial.world_position),
            world_position: judge.spatial.world_position,
            sector_id: judge.spatial.sector_id,
            sector_coordinate: grid::sector_id_to_sector_coordinate(judge.spatial.sector_id),
            size: judge.spatial.size,
            sight_world_position: judge.sight.world_position,
            sight_rotor: judge.sight.rotor,
            selected_block_kind: judge.selected_block_kind,
        };

        let mut population_view = PopulationView {
            judge_view,
            agent_view_map: HashMap::new(),
        };

        let judge_sight_range_squared = judge.sight.range_in_meters.powi(2);

        for agent in state.population.agent_map.values() {
            let agent_to_judge_mag_sq = (agent.spatial.world_position
                - state.population.judge.spatial.world_position)
                .mag_sq();

            if agent_to_judge_mag_sq > judge_sight_range_squared {
                continue;
            }

            let agent_view = AgentView {
                role: agent.identity.role,
                nation_kind: agent.identity.nation_kind,
                spatial: agent.spatial,
                kinematic: agent.kinematic,
            };

            population_view
                .agent_view_map
                .insert(agent.population_id, agent_view);
        }

        population_view
    }

    fn update_world_view(
        state: &State,
        sector_version_map: &mut HashMap<usize, u64>,
        block_view_cache: &mut HashMap<usize, Vec<Option<BlockView>>>,
    ) -> WorldView {
        let mut world_view = WorldView {
            sector_view_map: HashMap::new(),
        };

        let judge = &state.population.judge;

        let judge_sector_coordinate =
            grid::world_position_to_sector_coordinate(judge.spatial.world_position);

        let sight_range = judge.sight.range_in_sectors;

        for dz in -sight_range..=sight_range {
            for dy in -sight_range..=sight_range {
                for dx in -sight_range..=sight_range {
                    let sector_coordinate = judge_sector_coordinate + IVec3::new(dx, dy, dz);

                    if !grid::is_sector_coordinate_valid(sector_coordinate) {
                        continue;
                    }

                    let sector_id = grid::sector_coordinate_to_sector_id(sector_coordinate);
                    let sector = &state.world.sector_vec[sector_id];

                    let block_view_vec =
                        Self::get_block_view_vec(sector, sector_version_map, block_view_cache);

                    let sector_view = SectorView {
                        sector_id: sector.sector_id,
                        version: sector.version,
                        world_position: Vec3::from(sector.position),
                        block_view_vec,
                    };

                    world_view
                        .sector_view_map
                        .insert(sector.sector_id, sector_view);
                }
            }
        }

        world_view
    }

    fn get_block_view_vec(
        sector: &Sector,
        sector_version_map: &mut HashMap<usize, u64>,
        block_view_cache: &mut HashMap<usize, Vec<Option<BlockView>>>,
    ) -> Vec<Option<BlockView>> {
        let needs_rebuild = match sector_version_map.get(&sector.sector_id) {
            Some(current_version) => *current_version != sector.version,
            None => true,
        };

        let block_view_vec = if needs_rebuild {
            let block_view_vec = Self::build_block_view_vec(sector);

            block_view_cache.insert(sector.sector_id, block_view_vec.clone());
            sector_version_map.insert(sector.sector_id, sector.version);

            block_view_vec
        } else {
            block_view_cache[&sector.sector_id].clone()
        };

        block_view_vec
    }

    fn build_block_view_vec(sector: &Sector) -> Vec<Option<BlockView>> {
        let mut block_view_vec = vec![None; SECTOR_VOLUME_IN_CELLS];

        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell_coordinate = IVec3::new(x, y, z);

                    let cell = Sector::get_cell_at(cell_coordinate, sector);

                    if cell.block_kind == block::Kind::None {
                        continue;
                    }

                    let mut face_mask = face_mask::EMPTY;

                    for direction in grid::Direction::ALL {
                        let neighbor_cell_coordinate = cell_coordinate + direction.to_ivec3();

                        let neighbor_cell_clear =
                            if !grid::is_cell_coordinate_valid(neighbor_cell_coordinate) {
                                true
                            } else {
                                let neighbor_cell_id =
                                    grid::cell_coordinate_to_cell_id(neighbor_cell_coordinate);

                                sector.cell_vec[neighbor_cell_id].block_kind == block::Kind::None
                            };

                        if neighbor_cell_clear {
                            face_mask::set(face_mask::from_direction(direction), &mut face_mask);
                        }
                    }

                    let block_view = BlockView {
                        cell_id: cell.cell_id,
                        block_kind: cell.block_kind,
                        face_mask,
                    };

                    block_view_vec[cell.cell_id] = Some(block_view);
                }
            }
        }

        block_view_vec
    }
}
