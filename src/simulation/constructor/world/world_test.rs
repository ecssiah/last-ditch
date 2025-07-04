use crate::simulation::state::{
    world::{block, grid},
    World,
};
use glam::IVec3;

pub fn construct(world: &mut World) {
    build_rooms(world);
    build_central_room(world);
    build_clearance_test(world);

    world.set_block_kind(IVec3::new(-9, -2, 0), block::Kind::EsayaBlock);

    world.update_chunks();
}

fn build_rooms(world: &mut World) {
    let chunk_extent = world.grid.world_radius as i32 - 1;
    let chunk_radius = world.grid.chunk_radius as i32;

    for x in -chunk_extent..=chunk_extent {
        for y in -chunk_extent..=chunk_extent {
            for z in -chunk_extent..=chunk_extent {
                let chunk_coordinates = IVec3::new(x, y, z);
                let chunk_position = world.grid.chunk_coordinates_to_position(chunk_coordinates);

                let component_sum = chunk_coordinates.x + chunk_coordinates.y + chunk_coordinates.z;

                let chunk_kind = if component_sum % 2 == 0 {
                    block::Kind::Polished2
                } else {
                    block::Kind::Polished1
                };

                world.set_cube(
                    chunk_position - chunk_radius,
                    chunk_position + chunk_radius,
                    chunk_kind,
                );
            }
        }
    }
}

fn build_central_room(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;

    let chunk_coordinates = IVec3::new(0, 0, 0);
    let chunk_position = world.grid.chunk_coordinates_to_position(chunk_coordinates);

    world.set_cube(
        chunk_position + IVec3::new(-3 * chunk_radius - 1, -chunk_radius, -3 * chunk_radius - 1),
        chunk_position + IVec3::new(3 * chunk_radius + 1, chunk_radius, 3 * chunk_radius + 1),
        block::Kind::Empty,
    );

    let center_position = IVec3::new(0, -chunk_radius - 1, 0);

    world.set_block_kind(center_position + IVec3::Z * 2, block::Kind::North);
    world.set_block_kind(center_position - IVec3::Z * 2, block::Kind::South);
    world.set_block_kind(center_position + IVec3::X * 2, block::Kind::East);
    world.set_block_kind(center_position - IVec3::X * 2, block::Kind::West);
}

fn build_clearance_test(world: &mut World) {
    let chunk_radius = world.grid.chunk_radius as i32;

    let chunk_coordinates = IVec3::new(0, 0, 1);
    let chunk_position = world.grid.chunk_coordinates_to_position(chunk_coordinates);

    world.set_block_kind(
        chunk_position + IVec3::new(-4, -4, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(-3, -3, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(-2, -2, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(-1, -1, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(0, 0, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(1, 1, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(2, 2, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(3, 3, chunk_radius),
        block::Kind::CrimsonStone,
    );

    world.set_block_kind(
        chunk_position + IVec3::new(4, 4, chunk_radius),
        block::Kind::CrimsonStone,
    );
}

#[allow(dead_code)]
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
