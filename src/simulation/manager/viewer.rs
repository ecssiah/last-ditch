//! Simulation View producer

pub mod face_mask;
pub mod view;

use crate::simulation::{
    constants::*,
    manager::{
        viewer::view::{ManagerView, PersonView, PopulationView, SectorView, View, WorldView},
        Manager,
    },
    state::{
        world::{
            grid::{self},
            sector::Sector,
        },
        State,
    },
};
use std::collections::HashMap;
use ultraviolet::IVec3;

pub struct Viewer {
    pub view_input: triple_buffer::Input<View>,
    pub sector_version_map: HashMap<usize, u64>,
    pub sector_view_cache: HashMap<usize, SectorView>,
}

impl Viewer {
    pub fn new(view_input: triple_buffer::Input<View>) -> Self {
        Self {
            view_input,
            sector_version_map: HashMap::new(),
            sector_view_cache: HashMap::new(),
        }
    }

    pub fn tick(state: &State, manager: &mut Manager) {
        let _span = tracing::info_span!("viewer_tick").entered();

        let manager_view = Self::update_manager_view(manager);
        let population_view = Self::update_population_view(state);

        let world_view = Self::update_world_view(
            state,
            &mut manager.viewer.sector_version_map,
            &mut manager.viewer.sector_view_cache,
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
        sector_view_cache: &mut HashMap<usize, SectorView>,
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

                        if grid::sector_coordinate_is_valid(sector_coordinate) {
                            let sector_index =
                                grid::sector_coordinate_to_sector_index(sector_coordinate);

                            let sector = &state.world.sector_vec[sector_index];

                            let sector_view = Self::get_sector_view(
                                sector,
                                sector_version_map,
                                sector_view_cache,
                            );

                            world_view
                                .sector_view_map
                                .insert(sector.sector_index, sector_view);
                        }
                    }
                }
            }
        }

        world_view
    }

    fn get_sector_view(
        sector: &Sector,
        sector_version_map: &mut HashMap<usize, u64>,
        sector_view_cache: &mut HashMap<usize, SectorView>,
    ) -> SectorView {
        let needs_rebuild = match sector_version_map.get(&sector.sector_index) {
            Some(current_version) => *current_version != sector.version,
            None => true,
        };

        let sector_view = if needs_rebuild {
            SectorView::new_from_sector(sector)
        } else {
            sector_view_cache[&sector.sector_index].clone()
        };

        sector_view
    }
}
