use crate::simulation::state::{
    world::{
        block,
        grid::{self, Grid},
    },
    World,
};
use glam::IVec3;

pub fn construct(world: &mut World) {
    build_rooms(world);
    build_central_room(world);
    build_clearance_test(world);

    World::set_block_kind(
        IVec3::new(-9, -2, 0),
        block::Kind::EsayaBlock,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::update_chunks(&world.grid, &mut world.chunk_vec);
}

fn build_rooms(world: &mut World) {
    let world_extent_chunks = world.grid.world_extent_chunks as i32 - 1;
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    for x in -world_extent_chunks..=world_extent_chunks {
        for y in -world_extent_chunks..=world_extent_chunks {
            for z in -world_extent_chunks..=world_extent_chunks {
                let chunk_coordinates = IVec3::new(x, y, z);
                let chunk_position =
                    Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates);

                let component_sum = chunk_coordinates.x + chunk_coordinates.y + chunk_coordinates.z;

                let chunk_kind = if component_sum % 2 == 0 {
                    block::Kind::Polished2
                } else {
                    block::Kind::Polished1
                };

                World::set_cube(
                    chunk_position - chunk_extent_blocks,
                    chunk_position + chunk_extent_blocks,
                    chunk_kind,
                    &world.grid,
                    &world.block_info_map,
                    &mut world.chunk_vec,
                );
            }
        }
    }
}

fn build_central_room(world: &mut World) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    let chunk_coordinates = IVec3::new(0, 0, 0);
    let chunk_position = Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates);

    World::set_cube(
        chunk_position
            + IVec3::new(
                -3 * chunk_extent_blocks - 1,
                -chunk_extent_blocks,
                -3 * chunk_extent_blocks - 1,
            ),
        chunk_position
            + IVec3::new(
                3 * chunk_extent_blocks + 1,
                chunk_extent_blocks,
                3 * chunk_extent_blocks + 1,
            ),
        block::Kind::Empty,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    let center_position = IVec3::new(0, -chunk_extent_blocks - 1, 0);

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

fn build_clearance_test(world: &mut World) {
    let chunk_extent_blocks = world.grid.chunk_extent_blocks as i32;

    let chunk_coordinates = IVec3::new(0, 0, 1);
    let chunk_position = Grid::chunk_coordinates_to_position(&world.grid, chunk_coordinates);

    World::set_block_kind(
        chunk_position + IVec3::new(-4, -4, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(-3, -3, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(-2, -2, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(-1, -1, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(0, 0, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(1, 1, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(2, 2, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(3, 3, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );

    World::set_block_kind(
        chunk_position + IVec3::new(4, 4, chunk_extent_blocks),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.chunk_vec,
    );
}

#[allow(dead_code)]
fn build_chunk_room(
    world: &mut World,
    position: IVec3,
    entrance_vec: Vec<grid::Direction>,
    kind: block::Kind,
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
