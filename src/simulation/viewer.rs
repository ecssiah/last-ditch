//! Simulation View producer

pub mod face_mask;
pub mod view;

pub use view::AdminView;
pub use view::AgentView;
pub use view::BlockView;
pub use view::FaceView;
pub use view::JudgeView;
pub use view::PopulationView;
pub use view::SectorView;
pub use view::TimeView;
pub use view::View;
pub use view::WorldView;

use crate::simulation::state::{
    admin::{self},
    world::{
        block,
        grid::{self, Grid},
        sector::{self, Sector},
    },
    State,
};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct Viewer {
    pub sector_version_map: HashMap<sector::ID, u64>,
    pub block_view_cache: HashMap<sector::ID, Vec<Option<BlockView>>>,
}

impl Viewer {
    pub fn new() -> Self {
        Self {
            sector_version_map: HashMap::new(),
            block_view_cache: HashMap::new(),
        }
    }

    pub fn tick(
        state: &State,
        view_buffer_input: &mut triple_buffer::Input<View>,
        viewer: &mut Viewer,
    ) {
        let _ = tracing::info_span!("observation_tick").entered();

        Self::update_view(state, view_buffer_input, viewer);
    }

    pub fn get_view(view_buffer_output: &mut triple_buffer::Output<View>) -> &View {
        view_buffer_output.update();

        let view = view_buffer_output.peek_output_buffer();

        &view
    }

    fn update_view(
        state: &State,
        view_buffer_input: &mut triple_buffer::Input<View>,
        viewer: &mut Viewer,
    ) {
        let admin_view = Self::update_admin_view(state);
        let time_view = Self::update_time_view(state);

        let population_view = Self::update_population_view(state);

        let world_view = Self::update_world_view(
            state,
            &mut viewer.sector_version_map,
            &mut viewer.block_view_cache,
        );

        let view = view_buffer_input.input_buffer_mut();

        view.admin_view = admin_view;
        view.time_view = time_view;
        view.population_view = population_view;
        view.world_view = world_view;

        view_buffer_input.publish();
    }

    fn update_admin_view(state: &State) -> AdminView {
        AdminView {
            mode: state.admin.mode,
            message: state.admin.message.clone(),
            debug_active: state.admin.debug_active,
        }
    }

    fn update_time_view(state: &State) -> TimeView {
        if state.admin.mode == admin::Mode::Menu
            || state.admin.mode == admin::Mode::Loading
            || state.admin.mode == admin::Mode::Shutdown
        {
            return TimeView::new();
        }

        TimeView {
            instant: state.time.instant,
        }
    }

    fn update_population_view(state: &State) -> PopulationView {
        if state.admin.mode == admin::Mode::Menu
            || state.admin.mode == admin::Mode::Loading
            || state.admin.mode == admin::Mode::Shutdown
        {
            return PopulationView::new();
        }

        let judge = &state.population.judge;

        let judge_view = JudgeView {
            position: Grid::world_position_to_position(judge.spatial.world_position),
            world_position: judge.spatial.world_position,
            sector_id: judge.spatial.sector_id,
            sector_coordinates: Grid::sector_id_to_sector_coordinates(
                judge.spatial.sector_id,
                &state.world.grid,
            ),
            size: judge.spatial.size,
            sight_world_position: judge.sight.world_position,
            sight_rotor: judge.sight.rotor,
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

            population_view.agent_view_map.insert(agent.id, agent_view);
        }

        population_view
    }

    fn update_world_view(
        state: &State,
        sector_version_map: &mut HashMap<sector::ID, u64>,
        block_view_cache: &mut HashMap<sector::ID, Vec<Option<BlockView>>>,
    ) -> WorldView {
        if state.admin.mode == admin::Mode::Menu
            || state.admin.mode == admin::Mode::Loading
            || state.admin.mode == admin::Mode::Shutdown
        {
            return WorldView::new();
        }

        let mut world_view = WorldView {
            grid: state.world.grid,
            sector_view_map: HashMap::new(),
        };

        let judge = &state.population.judge;

        let judge_sector_coordinates = Grid::world_position_to_sector_coordinates(
            judge.spatial.world_position,
            &state.world.grid,
        );

        let sight_range = judge.sight.range_in_sectors;

        for dz in -sight_range..=sight_range {
            for dy in -sight_range..=sight_range {
                for dx in -sight_range..=sight_range {
                    let sector_coordinates = judge_sector_coordinates + IVec3::new(dx, dy, dz);

                    if !Grid::sector_coordinates_valid(sector_coordinates, &state.world.grid) {
                        continue;
                    }

                    let sector_id = Grid::sector_coordinates_to_sector_id(
                        sector_coordinates,
                        &state.world.grid,
                    );

                    let sector = &state.world.sector_vec[sector_id.to_usize()];

                    let block_view_vec = Self::get_block_view_vec(
                        sector,
                        &state.world.grid,
                        sector_version_map,
                        block_view_cache,
                    );

                    let sector_view = SectorView {
                        sector_id: sector.sector_id,
                        version: sector.version,
                        world_position: Vec3::from(sector.position),
                        radius: state.world.grid.sector_radius_in_meters,
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
        grid: &Grid,
        sector_version_map: &mut HashMap<sector::ID, u64>,
        block_view_cache: &mut HashMap<sector::ID, Vec<Option<BlockView>>>,
    ) -> Vec<Option<BlockView>> {
        let needs_rebuild = match sector_version_map.get(&sector.sector_id) {
            Some(current_version) => *current_version != sector.version,
            None => true,
        };

        let block_view_vec = if needs_rebuild {
            let block_view_vec = Self::build_block_view_vec(sector, grid);

            block_view_cache.insert(sector.sector_id, block_view_vec.clone());
            sector_version_map.insert(sector.sector_id, sector.version);

            block_view_vec
        } else {
            block_view_cache[&sector.sector_id].clone()
        };

        block_view_vec
    }

    fn build_block_view_vec(sector: &Sector, grid: &Grid) -> Vec<Option<BlockView>> {
        let mut block_view_vec = vec![None; grid.sector_volume_in_cells as usize];

        let sector_radius_in_cells = grid.sector_radius_in_cells as i32;

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell_coordinates = IVec3::new(x, y, z);

                    let cell = Sector::get_cell_at(cell_coordinates, grid, sector);

                    if cell.block_kind == block::Kind::None {
                        continue;
                    }

                    let mut face_mask = face_mask::EMPTY;

                    for direction in grid::Direction::get_direction_array() {
                        let neighbor_cell_coordinates = cell_coordinates + direction.to_ivec3();

                        let neighbor_cell_clear =
                            if !Grid::cell_coordinates_valid(neighbor_cell_coordinates, grid) {
                                true
                            } else {
                                let neighbor_cell_id = Grid::cell_coordinates_to_cell_id(
                                    neighbor_cell_coordinates,
                                    grid,
                                );

                                sector.cell_vec[neighbor_cell_id.to_usize()].block_kind
                                    == block::Kind::None
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

                    block_view_vec[cell.cell_id.to_usize()] = Some(block_view);
                }
            }
        }

        block_view_vec
    }
}
