use crate::simulation::{
    constants::*,
    state::{
        population::nation,
        work::{
            construct_task::{construct_population_data::ConstructPopulationData, ConstructTask},
            construct_worker::ConstructWorker,
        },
        world::{block, grid},
        State, World,
    },
};
use ultraviolet::IVec3;

#[derive(Clone)]
pub struct ConstructWorldData {
    pub stage: usize,
}

impl ConstructWorldData {
    pub fn new() -> Self {
        let stage = 1;

        Self { stage }
    }

    pub fn cost(construct_world_data: &ConstructWorldData) -> u32 {
        match construct_world_data.stage {
            1 => 100,
            2 => 100,
            3 => 100,
            4 => 100,
            _ => panic!("Requesting an invalid state cost"),
        }
    }

    pub fn step(state: &mut State, construct_world_data: &mut ConstructWorldData) -> bool {
        match construct_world_data.stage {
            1 => {
                ConstructWorldData::build_central_stage(&mut state.world);
                ConstructWorldData::build_ground(&mut state.world);

                construct_world_data.stage += 1;

                false
            }
            2 => {
                ConstructWorldData::build_compass(&mut state.world);

                ConstructWorldData::build_temple(34, 0, 0, nation::Kind::Wolf, &mut state.world);
                ConstructWorldData::build_temple(-34, 0, 0, nation::Kind::Lion, &mut state.world);
                ConstructWorldData::build_temple(0, 34, 0, nation::Kind::Eagle, &mut state.world);
                ConstructWorldData::build_temple(0, -34, 0, nation::Kind::Horse, &mut state.world);

                construct_world_data.stage += 1;

                false
            }
            3 => {
                ConstructWorldData::build_observation_deck(&mut state.world);

                construct_world_data.stage += 1;

                false
            }
            4 => {
                let construct_population_data = ConstructPopulationData::new();
                let construct_task = ConstructTask::ConstructPopulation(construct_population_data);

                ConstructWorker::enqueue(
                    construct_task,
                    &mut state.work.construct_worker.task_deque,
                );

                true
            }
            _ => unreachable!(),
        }
    }

    pub fn build_ground(world: &mut World) {
        let ground_boundary = (WORLD_RADIUS_IN_CELLS - SECTOR_SIZE_IN_CELLS) as i32;

        for z in -1..=0 {
            for y in -ground_boundary..=ground_boundary {
                for x in -ground_boundary..=ground_boundary {
                    let position = IVec3::new(x as i32, y as i32, z as i32);

                    let sector_coordinate = grid::grid_position_to_sector_coordinate(position);

                    let component_sum =
                        sector_coordinate.x + sector_coordinate.y + sector_coordinate.z;

                    let kind = if component_sum % 2 == 0 {
                        block::Kind::Polished1
                    } else {
                        block::Kind::Polished2
                    };

                    World::set_block(position, kind, &world.block_info_map, &mut world.sector_vec);
                }
            }
        }
    }

    pub fn build_compass(world: &mut World) {
        let radius = 4;
        let height = 2;

        World::set_block(
            IVec3::new(0, 0, 0),
            block::Kind::Stone2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(0, radius, height),
            block::Kind::NorthBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(-radius, 0, height),
            block::Kind::WestBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(0, -radius, height),
            block::Kind::SouthBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(radius, 0, height),
            block::Kind::EastBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    pub fn build_temple(x: i32, y: i32, z: i32, nation_kind: nation::Kind, world: &mut World) {
        world
            .flag_position_map
            .insert(nation_kind, IVec3::new(x, y, z + 3));

        World::set_block(
            IVec3::new(x, y, z + 6),
            nation_kind.block(),
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 8, y - 8, z + 1),
            IVec3::new(x + 8, y + 8, z + 1),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 7, y - 7, z + 2),
            IVec3::new(x + 7, y + 7, z + 2),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 6, y - 6, z + 8),
            IVec3::new(x + 6, y + 6, z + 8),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y - 5, z + 9),
            IVec3::new(x + 5, y + 5, z + 9),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y - 5, z + 8),
            IVec3::new(x + 5, y + 5, z + 8),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x + 5, y + 5, z + 1),
            IVec3::new(x + 5, y + 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y + 5, z + 1),
            IVec3::new(x - 5, y + 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x + 5, y - 5, z + 1),
            IVec3::new(x + 5, y - 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y - 5, z + 1),
            IVec3::new(x - 5, y - 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    pub fn build_observation_deck(world: &mut World) {
        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

        let height = 16;
        let center = 3 * sector_size_in_cells;

        World::set_cube(
            IVec3::new(-center + 1, -center + 1, height),
            IVec3::new(-center - 1, -center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(center + 1, -center + 1, height),
            IVec3::new(center - 1, -center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-center + 1, center + 1, height),
            IVec3::new(-center - 1, center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(center + 1, center + 1, height),
            IVec3::new(center - 1, center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(
                -center - sector_radius_in_cells,
                -center - sector_radius_in_cells,
                height,
            ),
            IVec3::new(
                center + sector_radius_in_cells,
                center + sector_radius_in_cells,
                height,
            ),
            block::Kind::Polished1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(
                -center + sector_radius_in_cells + 1,
                -center + sector_radius_in_cells + 1,
                height,
            ),
            IVec3::new(
                center - sector_radius_in_cells - 1,
                center - sector_radius_in_cells - 1,
                height,
            ),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    fn build_central_stage(world: &mut World) {
        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

        World::set_box(
            IVec3::broadcast(-sector_radius_in_cells),
            IVec3::broadcast(sector_radius_in_cells),
            block::Kind::Stone2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // East

        World::set_cube(
            IVec3::new(sector_radius_in_cells, -2, -1),
            IVec3::new(sector_radius_in_cells, 2, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(sector_radius_in_cells, -1, 0),
            IVec3::new(sector_radius_in_cells, 1, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(sector_size_in_cells - 1, -1, 0),
            IVec3::new(sector_size_in_cells + 1, 1, 12),
            block::Kind::WolfStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // West

        World::set_cube(
            IVec3::new(-sector_radius_in_cells, -2, -1),
            IVec3::new(-sector_radius_in_cells, 2, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-sector_radius_in_cells, -1, 0),
            IVec3::new(-sector_radius_in_cells, 1, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-sector_size_in_cells - 1, -1, 0),
            IVec3::new(-sector_size_in_cells + 1, 1, 12),
            block::Kind::LionStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // North

        World::set_cube(
            IVec3::new(-2, sector_radius_in_cells, -1),
            IVec3::new(2, sector_radius_in_cells, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, sector_radius_in_cells, 0),
            IVec3::new(1, sector_radius_in_cells, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, sector_size_in_cells - 1, 0),
            IVec3::new(1, sector_size_in_cells + 1, 12),
            block::Kind::EagleStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // South

        World::set_cube(
            IVec3::new(-2, -sector_radius_in_cells, -1),
            IVec3::new(2, -sector_radius_in_cells, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, -sector_radius_in_cells, 0),
            IVec3::new(1, -sector_radius_in_cells, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, -sector_size_in_cells - 1, 0),
            IVec3::new(1, -sector_size_in_cells + 1, 12),
            block::Kind::HorseStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }
}
