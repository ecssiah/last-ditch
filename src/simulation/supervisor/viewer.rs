//! Simulation View producer

pub mod face_mask;
pub mod view;

use crate::simulation::{
    state::{
        population::person::person_id::PersonID,
        world::{
            grid::{self},
            sector::Sector,
        },
        State,
    },
    supervisor::{
        viewer::view::{PersonView, PopulationView, SectorView, SupervisorView, View, WorldView},
        Supervisor,
    },
};
use std::collections::HashMap;
use tracing::instrument;
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

    #[instrument(skip_all)]
    pub fn tick(state: &State, supervisor: &mut Supervisor) {
        let supervisor_view = Self::update_supervisor_view(supervisor);

        let population_view = Self::update_population_view(state);

        let world_view = Self::update_world_view(
            state,
            &mut supervisor.viewer.sector_version_map,
            &mut supervisor.viewer.sector_view_cache,
        );

        let view = supervisor.viewer.view_input.input_buffer_mut();

        view.supervisor_view = supervisor_view;
        view.population_view = population_view;
        view.world_view = world_view;

        supervisor.viewer.view_input.publish();
    }

    #[instrument(skip_all)]
    pub fn get_view(view_output: &mut triple_buffer::Output<View>) -> &View {
        view_output.update();

        let view = view_output.peek_output_buffer();

        &view
    }

    #[instrument(skip_all)]
    fn update_supervisor_view(supervisor: &Supervisor) -> SupervisorView {
        let supervisor_view = SupervisorView {
            supervisor_status: supervisor.supervisor_status,
        };

        supervisor_view
    }

    #[instrument(skip_all)]
    fn update_population_view(state: &State) -> PopulationView {
        let mut population_view = PopulationView::default();

        if let Some(judge) = state.population.person_map.get(&PersonID::JUDGE_ID_1) {
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

    #[instrument(skip_all)]
    fn update_world_view(
        state: &State,
        sector_version_map: &mut HashMap<usize, u64>,
        sector_view_cache: &mut HashMap<usize, SectorView>,
    ) -> WorldView {
        let mut world_view = WorldView::default();

        if let Some(judge) = state.population.person_map.get(&PersonID::JUDGE_ID_1) {
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

    #[instrument(skip_all)]
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
