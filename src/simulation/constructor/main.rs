use crate::simulation::{
    consts::*,
    state::{
        population::{
            entity::{self, Agent},
            Population,
        },
        world::{block, World},
    },
};
use glam::IVec3;
use glam::Vec3;
use rand::Rng;

pub fn construct_world(world: &mut World) {
    log::info!("Setup Ground");

    build_ground(world);

    log::info!("Setup Structures");

    build_compass(world);

    build_temple(world, 0, 0, 34, entity::Kind::Eagle);
    build_temple(world, -34, 0, 0, entity::Kind::Lion);
    build_temple(world, 0, 0, -34, entity::Kind::Horse);
    build_temple(world, 34, 0, 0, entity::Kind::Wolf);

    build_observation_deck(world);

    world.update_chunks();
}

pub fn construct_population(population: &mut Population, world: &World) {
    setup_judge(population);
    setup_agents(population, world);
}

fn build_ground(world: &mut World) {
    let ground_boundary = (world.grid.world_boundary - world.grid.chunk_size) as isize;

    for x in -ground_boundary..=ground_boundary {
        for y in -1..=0 {
            for z in -ground_boundary..=ground_boundary {
                let position = IVec3::new(x as i32, y, z as i32);

                let chunk_coordinates = world.grid.position_to_chunk_coordinates(position).unwrap();

                let component_sum = chunk_coordinates.x + chunk_coordinates.y + chunk_coordinates.z;

                let kind = if component_sum % 2 == 0 {
                    block::Kind::Polished1
                } else {
                    block::Kind::Polished2
                };

                world.set_block_kind(position, kind);
            }
        }
    }
}

fn build_compass(world: &mut World) {
    world.set_block_kind(IVec3::new(0, 0, 0), block::Kind::TealStone);
    world.set_block_kind(IVec3::new(0, 0, 4), block::Kind::North);
    world.set_block_kind(IVec3::new(-4, 0, 0), block::Kind::West);
    world.set_block_kind(IVec3::new(0, 0, -4), block::Kind::South);
    world.set_block_kind(IVec3::new(4, 0, 0), block::Kind::East);
}

fn build_temple(world: &mut World, x: i32, y: i32, z: i32, kind: entity::Kind) {
    world.flags.insert(kind, IVec3::new(x, y + 3, z));

    world.set_block_kind(IVec3::new(x, y + 6, z), kind.icon());

    world.set_cube(
        IVec3::new(x - 8, y + 1, z - 8),
        IVec3::new(x + 8, y + 1, z + 8),
        block::Kind::Stone1,
    );

    world.set_cube(
        IVec3::new(x - 7, y + 2, z - 7),
        IVec3::new(x + 7, y + 2, z + 7),
        block::Kind::Stone1,
    );

    world.set_cube(
        IVec3::new(x - 6, y + 8, z - 6),
        IVec3::new(x + 6, y + 8, z + 6),
        block::Kind::Stone1,
    );

    world.set_cube(
        IVec3::new(x - 5, y + 9, z - 5),
        IVec3::new(x + 5, y + 9, z + 5),
        block::Kind::Stone1,
    );

    world.set_cube(
        IVec3::new(x - 5, y + 8, z - 5),
        IVec3::new(x + 5, y + 8, z + 5),
        block::Kind::Empty,
    );

    world.set_cube(
        IVec3::new(x + 5, y + 1, z + 5),
        IVec3::new(x + 5, y + 8, z + 5),
        block::Kind::Engraved1,
    );

    world.set_cube(
        IVec3::new(x - 5, y + 1, z + 5),
        IVec3::new(x - 5, y + 8, z + 5),
        block::Kind::Engraved1,
    );

    world.set_cube(
        IVec3::new(x + 5, y + 1, z - 5),
        IVec3::new(x + 5, y + 8, z - 5),
        block::Kind::Engraved1,
    );

    world.set_cube(
        IVec3::new(x - 5, y + 1, z - 5),
        IVec3::new(x - 5, y + 8, z - 5),
        block::Kind::Engraved1,
    );
}

fn build_observation_deck(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;
    let chunk_size = world.grid.chunk_size as i32;

    let height = 16;
    let center = 3 * chunk_size;

    world.set_cube(
        IVec3::new(-center + 1, height, -center + 1),
        IVec3::new(-center - 1, 0, -center - 1),
        block::Kind::Polished2,
    );

    world.set_cube(
        IVec3::new(center + 1, height, -center + 1),
        IVec3::new(center - 1, 0, -center - 1),
        block::Kind::Polished2,
    );

    world.set_cube(
        IVec3::new(-center + 1, height, center + 1),
        IVec3::new(-center - 1, 0, center - 1),
        block::Kind::Polished2,
    );

    world.set_cube(
        IVec3::new(center + 1, height, center + 1),
        IVec3::new(center - 1, 0, center - 1),
        block::Kind::Polished2,
    );

    world.set_cube(
        IVec3::new(-center - chunk_radius, height, -center - chunk_radius),
        IVec3::new(center + chunk_radius, height, center + chunk_radius),
        block::Kind::Polished1,
    );

    world.set_cube(
        IVec3::new(
            -center + chunk_radius + 1,
            height,
            -center + chunk_radius + 1,
        ),
        IVec3::new(center - chunk_radius - 1, height, center - chunk_radius - 1),
        block::Kind::Empty,
    );
}

fn setup_judge(population: &mut Population) {
    let judge = &mut population.judge;

    judge.set_world_position(Vec3::new(0.0, 2.0, 0.0));
    judge.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));
    judge.set_rotation(0.0, 0.0);
}

fn setup_agents(population: &mut Population, world: &World) {
    let mut rng = rand::thread_rng();

    let agent_initial_population = 16;
    let agent_size_bounds = (0.6, 2.2);

    for kind in entity::Kind::all() {
        if let Some(flag_position) = world.get_flag(kind) {
            let flag_position = flag_position.as_vec3();

            for _ in 0..agent_initial_population {
                let offset = Vec3::new(
                    rng.gen_range(-4..=4) as f32,
                    0.0,
                    rng.gen_range(-4..=4) as f32,
                );

                let world_position = flag_position + offset;

                let mut agent = Agent::new();

                agent.kind = kind;
                agent.set_world_position(world_position);
                agent.set_size(Vec3::new(
                    0.6,
                    rng.gen_range(agent_size_bounds.0..=agent_size_bounds.1),
                    0.6,
                ));

                population.agent_map.insert(agent.id, agent);
            }
        }
    }
}
