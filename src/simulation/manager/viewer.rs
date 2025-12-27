//! Simulation View producer

pub mod face_mask;
pub mod view;

use crate::simulation::{
    constants::*,
    manager::{
        Manager, viewer::view::{
            BlockView, CellView, ManagerView, ObjectView, PersonView, PopulationView, SectorView,
            View, WorldView,
        }
    },
    state::{
        State, World, world::{
            grid::{self, Direction}, sector::Sector,
        }
    },
};
use std::collections::HashMap;
use ultraviolet::IVec3;

pub struct Viewer {
    pub view_input: triple_buffer::Input<View>,
    pub sector_version_map: HashMap<usize, u64>,
    pub cell_view_cache: HashMap<usize, Vec<CellView>>,
}

impl Viewer {
    pub fn new(view_input: triple_buffer::Input<View>) -> Self {
        Self {
            view_input,
            sector_version_map: HashMap::new(),
            cell_view_cache: HashMap::new(),
        }
    }

    pub fn tick(state: &State, manager: &mut Manager) {
        let _span = tracing::info_span!("viewer_tick").entered();

        let manager_view = Self::update_manager_view(manager);

        let population_view = Self::update_population_view(state);

        let world_view = Self::update_world_view(
            state,
            &mut manager.viewer.sector_version_map,
            &mut manager.viewer.cell_view_cache,
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
        let mut population_view = PopulationView::new();

        if let Some(judge) = state.population.person_map.get(&ID_JUDGE_1) {
            let judge_sight_range_squared = judge.sight.range_in_meters.powi(2);

            for person in state.population.person_map.values() {
                let person_to_judge_distance_squared =
                    (person.transform.world_position - judge.transform.world_position).mag_sq();

                if person_to_judge_distance_squared <= judge_sight_range_squared {
                    let person_view = PersonView {
                        identity: person.identity.clone(),
                        transform: person.transform.clone(),
                        motion: person.motion.clone(),
                        body: person.body.clone(),
                        sight: person.sight.clone(),
                        selected_block_kind: person.selected_block_kind.clone(),
                    };

                    population_view
                        .person_view_map
                        .insert(person.person_id, person_view);
                }
            }
        }

        population_view
    }

    fn update_world_view(
        state: &State,
        sector_version_map: &mut HashMap<usize, u64>,
        cell_view_cache: &mut HashMap<usize, Vec<CellView>>,
    ) -> WorldView {
        let mut world_view = WorldView::new();

        if let Some(judge) = state.population.person_map.get(&ID_JUDGE_1) {
            let judge_sector_coordinate =
                grid::world_position_to_sector_coordinate(judge.transform.world_position);

            let sight_range = judge.sight.range_in_sectors;

            for dz in -sight_range..=sight_range {
                for dy in -sight_range..=sight_range {
                    for dx in -sight_range..=sight_range {
                        let sector_coordinate = judge_sector_coordinate + IVec3::new(dx, dy, dz);

                        if !grid::is_sector_coordinate_valid(sector_coordinate) {
                            continue;
                        }

                        let sector_index = grid::sector_coordinate_to_sector_index(sector_coordinate);
                        let sector = &state.world.sector_vec[sector_index];
                        let world_position =
                            grid::grid_position_to_world_position(sector.grid_position);

                        let cell_view_vec = Self::get_cell_view_vec(
                            sector,
                            &state.world,
                            sector_version_map,
                            cell_view_cache,
                        );

                        let sector_view = SectorView {
                            sector_index: sector.sector_index,
                            version: sector.version,
                            world_position,
                            cell_view_vec,
                        };

                        world_view
                            .sector_view_map
                            .insert(sector.sector_index, sector_view);
                    }
                }
            }
        }

        world_view
    }

    fn get_cell_view_vec(
        sector: &Sector,
        world: &World,
        sector_version_map: &mut HashMap<usize, u64>,
        cell_view_cache: &mut HashMap<usize, Vec<CellView>>,
    ) -> Vec<CellView> {
        let needs_rebuild = match sector_version_map.get(&sector.sector_index) {
            Some(current_version) => *current_version != sector.version,
            None => true,
        };

        let cell_view_vec = if needs_rebuild {
            let cell_view_vec = Self::build_cell_view_vec(sector, world);

            cell_view_cache.insert(sector.sector_index, cell_view_vec.clone());
            sector_version_map.insert(sector.sector_index, sector.version);

            cell_view_vec
        } else {
            cell_view_cache[&sector.sector_index].clone()
        };

        cell_view_vec
    }

    fn build_cell_view_vec(sector: &Sector, world: &World) -> Vec<CellView> {
        let mut cell_view_vec = vec![CellView::default(); SECTOR_VOLUME_IN_CELLS];

        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell_coordinate = IVec3::new(x, y, z);

                    let cell_index = grid::cell_coordinate_to_cell_index(cell_coordinate);
                    let grid_position = grid::ids_to_grid_position(sector.sector_index, cell_index);

                    let cell = World::get_cell_at(grid_position, &world.sector_vec);

                    let block_view = if let Some(block) = cell.block.as_ref() {
                        let mut face_mask = face_mask::EMPTY;

                        for direction in grid::Direction::ALL {
                            let neighbor_grid_position =
                                grid_position + Direction::to_ivec3(direction);

                            if !World::is_block_solid_at(neighbor_grid_position, world) {
                                face_mask::set(
                                    face_mask::from_direction(direction),
                                    &mut face_mask,
                                );
                            }
                        }

                        let block_view = BlockView {
                            block_kind: block.block_kind,
                            face_mask,
                        };

                        Some(block_view)
                    } else {
                        None
                    };

                    let object_view = if let Some(object) = cell.object.as_ref() {
                        let object_view = ObjectView {
                            object_kind: object.object_kind,
                            direction: object.direction,
                        };

                        Some(object_view)
                    } else {
                        None
                    };

                    let cell_view = CellView {
                        cell_index,
                        grid_position,
                        block_view,
                        object_view,
                    };

                    cell_view_vec[cell.cell_index] = cell_view;
                }
            }
        }

        cell_view_vec
    }
}
