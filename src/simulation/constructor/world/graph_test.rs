use crate::simulation::state::world::{
    block,
    grid::{self, Grid},
    World,
};
use glam::IVec3;

pub fn construct(world: &mut World) {
    build_center_room(world);

    build_vertical_entrance_room(world);
    build_constricted_entrance_room(world);
    build_expanded_entrance_room(world);
    build_multiple_entrance_room(world);

    // build_floor(world);

    World::update_chunks(&world.grid, &mut world.chunk_vec);
}

fn build_center_room(world: &mut World) {
    let chunk_coordinates = IVec3::new(0, 0, 0);
    let chunk_position = Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates);

    let entrances = Vec::from([
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
    ]);

    build_chunk_room(chunk_position, entrances, block::Kind::Polished1, world);

    let center_position = IVec3::new(0, -4, 0);

    World::set_block_kind(
        center_position + IVec3::Z * 2,
        block::Kind::North,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        center_position - IVec3::Z * 2,
        block::Kind::South,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        center_position + IVec3::X * 2,
        block::Kind::East,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        center_position - IVec3::X * 2,
        block::Kind::West,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );
}

fn build_constricted_entrance_room(world: &mut World) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    let chunk_coordinates = IVec3::new(-1, 0, 0);
    let chunk_position = Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates);

    let entrances = Vec::from([
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ]);

    build_chunk_room(chunk_position, entrances, block::Kind::Polished2, world);

    World::set_cube(
        chunk_position + IVec3::new(chunk_extent_blocks, 0, 0),
        chunk_position + IVec3::new(chunk_extent_blocks, -2, 0),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position + IVec3::new(chunk_extent_blocks + 1, 1, -1),
        chunk_position + IVec3::new(chunk_extent_blocks + 1, -3, 1),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );
}

fn build_expanded_entrance_room(world: &mut World) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    let chunk_coordinates = IVec3::new(1, 0, 0);
    let chunk_position = Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates);

    let entrances = Vec::from([
        grid::Direction::XpYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ]);

    build_chunk_room(chunk_position, entrances, block::Kind::Polished2, world);

    World::set_cube(
        chunk_position + IVec3::new(-chunk_extent_blocks, 1, -2),
        chunk_position + IVec3::new(-chunk_extent_blocks + 2, -4, 2),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position + IVec3::new(-chunk_extent_blocks - 1, 0, -1),
        chunk_position + IVec3::new(-chunk_extent_blocks - 1, -3, 1),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position + IVec3::new(-chunk_extent_blocks, -5, -2),
        chunk_position + IVec3::new(-chunk_extent_blocks + 2, -5, 2),
        block::Kind::Polished2,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );
}

fn build_multiple_entrance_room(world: &mut World) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    let chunk_coordinates = IVec3::new(0, 0, -1);
    let chunk_position = Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates);

    let entrances = vec![
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZn,
    ];

    build_chunk_room(chunk_position, entrances, block::Kind::Polished2, world);

    World::set_cube(
        chunk_position + IVec3::new(-chunk_extent_blocks + 1, 0, chunk_extent_blocks),
        chunk_position + IVec3::new(-chunk_extent_blocks + 2, -3, chunk_extent_blocks + 1),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position + IVec3::new(chunk_extent_blocks - 2, 0, chunk_extent_blocks),
        chunk_position + IVec3::new(chunk_extent_blocks - 1, -3, chunk_extent_blocks + 1),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );
}

fn build_vertical_entrance_room(world: &mut World) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    let chunk_coordinates_001 = IVec3::new(0, 0, 1);
    let chunk_position_001 =
        Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates_001);

    let entrance_vec = vec![
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ];

    build_chunk_room(
        chunk_position_001,
        entrance_vec,
        block::Kind::Polished2,
        world,
    );

    let chunk_coordinates_011 = IVec3::new(0, 1, 1);
    let chunk_position_011 =
        Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates_011);

    let entrance_vec = vec![];

    build_chunk_room(
        chunk_position_011,
        entrance_vec,
        block::Kind::Polished1,
        world,
    );

    World::set_box(
        chunk_position_001 + IVec3::new(-chunk_extent_blocks + 1, chunk_extent_blocks, -chunk_extent_blocks + 1),
        chunk_position_001 + IVec3::new(chunk_extent_blocks - 1, chunk_extent_blocks + 1, chunk_extent_blocks - 1),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(2, -3, -2),
        chunk_position_001 + IVec3::new(3, -3, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(2, -2, -1),
        chunk_position_001 + IVec3::new(3, -2, -1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(2, -1, 0),
        chunk_position_001 + IVec3::new(3, -1, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(2, 0, 1),
        chunk_position_001 + IVec3::new(3, 0, 1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(2, 1, 2),
        chunk_position_001 + IVec3::new(3, 1, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(1, 2, 2),
        chunk_position_001 + IVec3::new(1, 2, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(0, 3, 2),
        chunk_position_001 + IVec3::new(0, 3, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-1, 4, 2),
        chunk_position_001 + IVec3::new(-1, 4, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-3, 5, 0),
        chunk_position_001 + IVec3::new(-2, 5, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-2, -3, 2),
        chunk_position_001 + IVec3::new(-3, -3, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-2, -2, 1),
        chunk_position_001 + IVec3::new(-3, -2, 1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-2, -1, 0),
        chunk_position_001 + IVec3::new(-3, -1, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-2, 0, -1),
        chunk_position_001 + IVec3::new(-3, 0, -1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-2, 1, -2),
        chunk_position_001 + IVec3::new(-3, 1, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-1, 2, -2),
        chunk_position_001 + IVec3::new(-1, 2, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(0, 3, -2),
        chunk_position_001 + IVec3::new(0, 3, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(1, 4, -2),
        chunk_position_001 + IVec3::new(1, 4, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(2, 5, -3),
        chunk_position_001 + IVec3::new(3, 5, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_cube(
        chunk_position_001 + IVec3::new(-1, 5, 0),
        chunk_position_001 + IVec3::new(1, 5, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );
}

fn build_chunk_room(
    position: IVec3,
    entrance_vec: Vec<grid::Direction>,
    kind: block::Kind,
    world: &mut World,
) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    World::set_box(
        position - IVec3::splat(chunk_extent_blocks),
        position + IVec3::splat(chunk_extent_blocks),
        kind,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        position - IVec3::Y * chunk_extent_blocks,
        block::Kind::TealStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    if entrance_vec.contains(&grid::Direction::XpYoZo) {
        World::set_cube(
            position + IVec3::new(chunk_extent_blocks, 0, 1),
            position + IVec3::new(chunk_extent_blocks, -3, -1),
            block::Kind::Empty,
            &world.grid,
            &world.block_info_map,
            &mut world.chunk_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XnYoZo) {
        World::set_cube(
            position + IVec3::new(-chunk_extent_blocks, 0, 1),
            position + IVec3::new(-chunk_extent_blocks, -3, -1),
            block::Kind::Empty,
            &world.grid,
            &world.block_info_map,
            &mut world.chunk_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYoZp) {
        World::set_cube(
            position + IVec3::new(1, 0, chunk_extent_blocks),
            position + IVec3::new(-1, -3, chunk_extent_blocks),
            block::Kind::Empty,
            &world.grid,
            &world.block_info_map,
            &mut world.chunk_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYoZn) {
        World::set_cube(
            position + IVec3::new(1, 0, -chunk_extent_blocks),
            position + IVec3::new(-1, -3, -chunk_extent_blocks),
            block::Kind::Empty,
            &world.grid,
            &world.block_info_map,
            &mut world.chunk_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYpZo) {
        World::set_cube(
            position + IVec3::new(-1, chunk_extent_blocks, -1),
            position + IVec3::new(1, chunk_extent_blocks, 1),
            block::Kind::Empty,
            &world.grid,
            &world.block_info_map,
            &mut world.chunk_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYnZo) {
        World::set_cube(
            position + IVec3::new(-1, -chunk_extent_blocks, -1),
            position + IVec3::new(1, -chunk_extent_blocks, 1),
            block::Kind::Empty,
            &world.grid,
            &world.block_info_map,
            &mut world.chunk_vec,
        );
    }
}

fn _build_floor(world: &mut World) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;
    let chunk_size_blocks = world.grid.chunk_size_blocks as i32;

    World::set_cube(
        IVec3::new(-2 * chunk_size_blocks, -chunk_extent_blocks - 1, -2 * chunk_size_blocks),
        IVec3::new(2 * chunk_size_blocks, -chunk_extent_blocks - 1, 2 * chunk_size_blocks),
        block::Kind::Polished1,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        IVec3::new(13, 12, 13),
        block::Kind::EsayaBlock,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );
}
