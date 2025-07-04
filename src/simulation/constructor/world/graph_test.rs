use crate::simulation::state::world::{block, grid, World};
use glam::IVec3;

pub fn construct(world: &mut World) {
    build_center_room(world);

    build_vertical_entrance_room(world);
    build_constricted_entrance_room(world);
    build_expanded_entrance_room(world);
    build_multiple_entrance_room(world);

    // build_floor(world);

    world.update_chunks();
}

fn build_center_room(world: &mut World) {
    let chunk_coordinates = IVec3::new(0, 0, 0);
    let chunk_position = world.grid.chunk_coordinates_to_position(chunk_coordinates);

    let entrances = Vec::from([
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
    ]);

    build_chunk_room(world, chunk_position, entrances, block::Kind::Polished1);

    let center_position = IVec3::new(0, -4, 0);

    world.set_block_kind(center_position + IVec3::Z * 2, block::Kind::North);
    world.set_block_kind(center_position - IVec3::Z * 2, block::Kind::South);
    world.set_block_kind(center_position + IVec3::X * 2, block::Kind::East);
    world.set_block_kind(center_position - IVec3::X * 2, block::Kind::West);
}

fn build_constricted_entrance_room(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;

    let chunk_coordinates = IVec3::new(-1, 0, 0);
    let chunk_position = world.grid.chunk_coordinates_to_position(chunk_coordinates);

    let entrances = Vec::from([
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ]);

    build_chunk_room(world, chunk_position, entrances, block::Kind::Polished2);

    world.set_cube(
        chunk_position + IVec3::new(chunk_radius, 0, 0),
        chunk_position + IVec3::new(chunk_radius, -2, 0),
        block::Kind::Empty,
    );

    world.set_cube(
        chunk_position + IVec3::new(chunk_radius + 1, 1, -1),
        chunk_position + IVec3::new(chunk_radius + 1, -3, 1),
        block::Kind::Empty,
    );
}

fn build_expanded_entrance_room(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;

    let chunk_coordinates = IVec3::new(1, 0, 0);
    let chunk_position = world.grid.chunk_coordinates_to_position(chunk_coordinates);

    let entrances = Vec::from([
        grid::Direction::XpYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ]);

    build_chunk_room(world, chunk_position, entrances, block::Kind::Polished2);

    world.set_cube(
        chunk_position + IVec3::new(-chunk_radius, 1, -2),
        chunk_position + IVec3::new(-chunk_radius + 2, -4, 2),
        block::Kind::Empty,
    );

    world.set_cube(
        chunk_position + IVec3::new(-chunk_radius - 1, 0, -1),
        chunk_position + IVec3::new(-chunk_radius - 1, -3, 1),
        block::Kind::Empty,
    );

    world.set_cube(
        chunk_position + IVec3::new(-chunk_radius, -5, -2),
        chunk_position + IVec3::new(-chunk_radius + 2, -5, 2),
        block::Kind::Polished2,
    );
}

fn build_multiple_entrance_room(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;

    let chunk_coordinates = IVec3::new(0, 0, -1);
    let chunk_position = world.grid.chunk_coordinates_to_position(chunk_coordinates);

    let entrances = vec![
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZn,
    ];

    build_chunk_room(world, chunk_position, entrances, block::Kind::Polished2);

    world.set_cube(
        chunk_position + IVec3::new(-chunk_radius + 1, 0, chunk_radius),
        chunk_position + IVec3::new(-chunk_radius + 2, -3, chunk_radius + 1),
        block::Kind::Empty,
    );

    world.set_cube(
        chunk_position + IVec3::new(chunk_radius - 2, 0, chunk_radius),
        chunk_position + IVec3::new(chunk_radius - 1, -3, chunk_radius + 1),
        block::Kind::Empty,
    );
}

fn build_vertical_entrance_room(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;

    let chunk_coordinates_001 = IVec3::new(0, 0, 1);
    let chunk_position_001 = world
        .grid
        .chunk_coordinates_to_position(chunk_coordinates_001);

    let entrance_vec = vec![
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ];

    build_chunk_room(
        world,
        chunk_position_001,
        entrance_vec,
        block::Kind::Polished2,
    );

    let chunk_coordinates_011 = IVec3::new(0, 1, 1);
    let chunk_position_011 = world
        .grid
        .chunk_coordinates_to_position(chunk_coordinates_011);

    let entrance_vec = vec![];

    build_chunk_room(
        world,
        chunk_position_011,
        entrance_vec,
        block::Kind::Polished1,
    );

    world.set_box(
        chunk_position_001 + IVec3::new(-chunk_radius + 1, chunk_radius, -chunk_radius + 1),
        chunk_position_001 + IVec3::new(chunk_radius - 1, chunk_radius + 1, chunk_radius - 1),
        block::Kind::Empty,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(2, -3, -2),
        chunk_position_001 + IVec3::new(3, -3, -3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(2, -2, -1),
        chunk_position_001 + IVec3::new(3, -2, -1),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(2, -1, 0),
        chunk_position_001 + IVec3::new(3, -1, 0),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(2, 0, 1),
        chunk_position_001 + IVec3::new(3, 0, 1),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(2, 1, 2),
        chunk_position_001 + IVec3::new(3, 1, 3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(1, 2, 2),
        chunk_position_001 + IVec3::new(1, 2, 3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(0, 3, 2),
        chunk_position_001 + IVec3::new(0, 3, 3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-1, 4, 2),
        chunk_position_001 + IVec3::new(-1, 4, 3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-3, 5, 0),
        chunk_position_001 + IVec3::new(-2, 5, 3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-2, -3, 2),
        chunk_position_001 + IVec3::new(-3, -3, 3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-2, -2, 1),
        chunk_position_001 + IVec3::new(-3, -2, 1),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-2, -1, 0),
        chunk_position_001 + IVec3::new(-3, -1, 0),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-2, 0, -1),
        chunk_position_001 + IVec3::new(-3, 0, -1),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-2, 1, -2),
        chunk_position_001 + IVec3::new(-3, 1, -3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-1, 2, -2),
        chunk_position_001 + IVec3::new(-1, 2, -3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(0, 3, -2),
        chunk_position_001 + IVec3::new(0, 3, -3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(1, 4, -2),
        chunk_position_001 + IVec3::new(1, 4, -3),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(2, 5, -3),
        chunk_position_001 + IVec3::new(3, 5, 0),
        block::Kind::CrimsonStone,
    );

    world.set_cube(
        chunk_position_001 + IVec3::new(-1, 5, 0),
        chunk_position_001 + IVec3::new(1, 5, 0),
        block::Kind::CrimsonStone,
    );
}

fn build_chunk_room(
    world: &mut World,
    position: IVec3,
    entrance_vec: Vec<grid::Direction>,
    kind: block::Kind,
) {
    let chunk_radius = world.grid.chunk_radius as i32;

    world.set_box(
        position - IVec3::splat(chunk_radius),
        position + IVec3::splat(chunk_radius),
        kind,
    );

    world.set_block_kind(position - IVec3::Y * chunk_radius, block::Kind::TealStone);

    if entrance_vec.contains(&grid::Direction::XpYoZo) {
        world.set_cube(
            position + IVec3::new(chunk_radius, 0, 1),
            position + IVec3::new(chunk_radius, -3, -1),
            block::Kind::Empty,
        );
    }

    if entrance_vec.contains(&grid::Direction::XnYoZo) {
        world.set_cube(
            position + IVec3::new(-chunk_radius, 0, 1),
            position + IVec3::new(-chunk_radius, -3, -1),
            block::Kind::Empty,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYoZp) {
        world.set_cube(
            position + IVec3::new(1, 0, chunk_radius),
            position + IVec3::new(-1, -3, chunk_radius),
            block::Kind::Empty,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYoZn) {
        world.set_cube(
            position + IVec3::new(1, 0, -chunk_radius),
            position + IVec3::new(-1, -3, -chunk_radius),
            block::Kind::Empty,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYpZo) {
        world.set_cube(
            position + IVec3::new(-1, chunk_radius, -1),
            position + IVec3::new(1, chunk_radius, 1),
            block::Kind::Empty,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYnZo) {
        world.set_cube(
            position + IVec3::new(-1, -chunk_radius, -1),
            position + IVec3::new(1, -chunk_radius, 1),
            block::Kind::Empty,
        );
    }
}

fn _build_floor(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;
    let chunk_size = world.grid.chunk_size as i32;

    world.set_cube(
        IVec3::new(-2 * chunk_size, -chunk_radius - 1, -2 * chunk_size),
        IVec3::new(2 * chunk_size, -chunk_radius - 1, 2 * chunk_size),
        block::Kind::Polished1,
    );

    world.set_block_kind(IVec3::new(13, 12, 13), block::Kind::EsayaBlock);
}
