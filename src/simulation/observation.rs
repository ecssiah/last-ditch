//! Generates Views of Simulation data

pub mod view;

use crate::simulation::{
    consts::{JUDGE_VIEW_RADIUS_IN_SECTORS, JUDGE_VIEW_RADIUS_SQUARED},
    observation::view::{
        AdminView, AgentView, FaceView, JudgeView, PopulationView, SectorView, TimeView, View,
        WorldView,
    },
    state::{
        population::entity::Spatial,
        world::{
            block,
            grid::{self, Grid},
            sector::Sector,
        },
        State,
    },
};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct Observation {}

impl Observation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(state: &State, view_buffer_input: &mut triple_buffer::Input<View>) {
        Self::update_view(state, view_buffer_input);
    }

    pub fn get_view(view_buffer_output: &mut triple_buffer::Output<View>) -> &View {
        view_buffer_output.update();

        let view = view_buffer_output.peek_output_buffer();

        &view
    }

    fn update_view(state: &State, view_buffer_input: &mut triple_buffer::Input<View>) {
        let admin_view = Self::update_admin_view(state);
        let time_view = Self::update_time_view(state);
        let population_view = Self::update_population_view(state);
        let world_view = Self::update_world_view(state);

        let view = view_buffer_input.input_buffer_mut();

        view.entity_id = state.population.judge.info.entity_id;
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
        TimeView {
            instant: state.time.instant,
        }
    }

    fn update_population_view(state: &State) -> PopulationView {
        let judge = &state.population.judge;

        let judge_view = JudgeView {
            entity_id: judge.info.entity_id,
            position: Grid::world_to_position(&state.world.grid, judge.spatial.world_position),
            world_position: judge.spatial.world_position,
            sector_id: judge.info.sector_id,
            sector_coordinates: Grid::sector_id_to_sector_coordinates(
                &state.world.grid,
                judge.info.sector_id,
            ),
            size: judge.spatial.size,
            rotor: judge.spatial.rotor,
            eye: Spatial::eye(&judge.spatial),
        };

        let mut population_view = PopulationView {
            judge_view,
            agent_view_map: HashMap::new(),
        };

        for agent in state.population.agent_map.values() {
            let agent_to_judge_mag_sq = (agent.spatial.world_position
                - state.population.judge.spatial.world_position)
                .mag_sq();

            if agent_to_judge_mag_sq > JUDGE_VIEW_RADIUS_SQUARED {
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
            sector_view_map: HashMap::new(),
        };

        let judge_sector_coordinates = Grid::world_to_sector_coordinates(
            &state.world.grid,
            state.population.judge.spatial.world_position,
        );

        for dz in -JUDGE_VIEW_RADIUS_IN_SECTORS..=JUDGE_VIEW_RADIUS_IN_SECTORS {
            for dy in -JUDGE_VIEW_RADIUS_IN_SECTORS..=JUDGE_VIEW_RADIUS_IN_SECTORS {
                for dx in -JUDGE_VIEW_RADIUS_IN_SECTORS..=JUDGE_VIEW_RADIUS_IN_SECTORS {
                    let sector_coordinates = judge_sector_coordinates + IVec3::new(dx, dy, dz);

                    let sector_id = Grid::sector_coordinates_to_sector_id(
                        &state.world.grid,
                        sector_coordinates,
                    );

                    if Grid::sector_id_valid(&state.world.grid, sector_id) {
                        if let Some(sector) = state.world.sector_vec.get(usize::from(sector_id)) {
                            let sector_view = SectorView {
                                sector_id: sector.sector_id,
                                world_position: Vec3::from(sector.position),
                                radius: Vec3::broadcast(state.world.grid.sector_radius_in_meters),
                                face_view_vec: Self::compute_face_view_vec(
                                    &state.world.grid,
                                    sector,
                                ),
                            };

                            world_view
                                .sector_view_map
                                .insert(sector.sector_id, sector_view);
                        }
                    }
                }
            }
        }

        world_view
    }

    fn compute_face_view_vec(grid: &Grid, sector: &Sector) -> Vec<FaceView> {
        let mut face_view_vec = Vec::new();

        let sector_radius_in_cells = grid.sector_radius_in_cells as i32;

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell_id = Grid::cell_coordinates_to_cell_id(grid, IVec3::new(x, y, z));

                    if Grid::cell_id_valid(grid, cell_id) {
                        let cell = &sector.cell_vec[usize::from(cell_id)];
                        let cell_world_position = Vec3::from(cell.position);

                        if cell.block_kind == block::Kind::None {
                            continue;
                        }

                        let xn_cell_id =
                            Grid::cell_coordinates_to_cell_id(grid, IVec3::new(x - 1, y, z));

                        let xn_visible = (x - 1) < -sector_radius_in_cells
                            || !Grid::cell_id_valid(grid, xn_cell_id)
                            || sector.cell_vec[usize::from(xn_cell_id)].block_kind
                                == block::Kind::None;

                        if xn_visible {
                            face_view_vec.push(FaceView {
                                position: cell_world_position,
                                direction: grid::Direction::XNYOZO,
                                block_kind: cell.block_kind,
                            });
                        }

                        let xp_cell_id =
                            Grid::cell_coordinates_to_cell_id(grid, IVec3::new(x + 1, y, z));

                        let xp_visible = (x + 1) > sector_radius_in_cells
                            || !Grid::cell_id_valid(grid, xp_cell_id)
                            || sector.cell_vec[usize::from(xp_cell_id)].block_kind
                                == block::Kind::None;

                        if xp_visible {
                            face_view_vec.push(FaceView {
                                position: cell_world_position,
                                direction: grid::Direction::XPYOZO,
                                block_kind: cell.block_kind,
                            });
                        }

                        let yn_cell_id =
                            Grid::cell_coordinates_to_cell_id(grid, IVec3::new(x, y - 1, z));

                        let yn_visible = (y - 1) < -sector_radius_in_cells
                            || !Grid::cell_id_valid(grid, yn_cell_id)
                            || sector.cell_vec[usize::from(yn_cell_id)].block_kind
                                == block::Kind::None;

                        if yn_visible {
                            face_view_vec.push(FaceView {
                                position: cell_world_position,
                                direction: grid::Direction::XOYNZO,
                                block_kind: cell.block_kind,
                            });
                        }

                        let yp_cell_id =
                            Grid::cell_coordinates_to_cell_id(grid, IVec3::new(x, y + 1, z));

                        let yp_visible = (y + 1) > sector_radius_in_cells
                            || !Grid::cell_id_valid(grid, yp_cell_id)
                            || sector.cell_vec[usize::from(yp_cell_id)].block_kind
                                == block::Kind::None;

                        if yp_visible {
                            face_view_vec.push(FaceView {
                                position: cell_world_position,
                                direction: grid::Direction::XOYPZO,
                                block_kind: cell.block_kind,
                            });
                        }

                        let zn_cell_id =
                            Grid::cell_coordinates_to_cell_id(grid, IVec3::new(x, y, z - 1));

                        let zn_visible = (z - 1) < -sector_radius_in_cells
                            || !Grid::cell_id_valid(grid, zn_cell_id)
                            || sector.cell_vec[usize::from(zn_cell_id)].block_kind
                                == block::Kind::None;

                        if zn_visible {
                            face_view_vec.push(FaceView {
                                position: cell_world_position,
                                direction: grid::Direction::XOYOZN,
                                block_kind: cell.block_kind,
                            });
                        }

                        let zp_cell_id =
                            Grid::cell_coordinates_to_cell_id(grid, IVec3::new(x, y, z + 1));

                        let zp_visible = (z + 1) > sector_radius_in_cells
                            || !Grid::cell_id_valid(grid, zp_cell_id)
                            || sector.cell_vec[usize::from(zp_cell_id)].block_kind
                                == block::Kind::None;

                        if zp_visible {
                            face_view_vec.push(FaceView {
                                position: cell_world_position,
                                direction: grid::Direction::XOYOZP,
                                block_kind: cell.block_kind,
                            });
                        }
                    }
                }
            }
        }

        face_view_vec
    }
}
