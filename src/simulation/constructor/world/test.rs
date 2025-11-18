use crate::simulation::state::{
    world::{
        block,
        grid::{self, Grid},
    },
    World,
};
use ultraviolet::IVec3;

pub fn run(world: &mut World) {
    // build_rooms(world);
    // build_central_room(world);
    // build_clearance_test(world);

    // World::set_block(
    //     IVec3::new(-9, -2, 0),
    //     block::Kind::EsayaBlock,
    //     &world.grid,
    //     &world.block_info_map,
    //     &mut world.sector_vec,
    // );

    World::set_cube(
        IVec3::new(-3, -3, -3),
        IVec3::new(3, 3, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::update_sectors(&world.grid, &mut world.sector_vec);
}

fn build_rooms(world: &mut World) {
    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32 - 1;
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    for z in -world_radius_in_sectors..=world_radius_in_sectors {
        for y in -world_radius_in_sectors..=world_radius_in_sectors {
            for x in -world_radius_in_sectors..=world_radius_in_sectors {
                let sector_coordinates = IVec3::new(x, y, z);
                let sector_position =
                    Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

                let component_sum =
                    sector_coordinates.x + sector_coordinates.y + sector_coordinates.z;

                let sector_kind = if component_sum % 2 == 0 {
                    block::Kind::Polished2
                } else {
                    block::Kind::Polished1
                };

                World::set_cube(
                    sector_position - IVec3::broadcast(sector_radius_in_cells),
                    sector_position + IVec3::broadcast(sector_radius_in_cells),
                    sector_kind,
                    &world.grid,
                    &world.block_info_map,
                    &mut world.sector_vec,
                );
            }
        }
    }
}

fn build_central_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(0, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    World::set_cube(
        sector_position
            + IVec3::new(
                -3 * sector_radius_in_cells - 1,
                -sector_radius_in_cells,
                -3 * sector_radius_in_cells - 1,
            ),
        sector_position
            + IVec3::new(
                3 * sector_radius_in_cells + 1,
                sector_radius_in_cells,
                3 * sector_radius_in_cells + 1,
            ),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    let center_position = IVec3::new(0, -sector_radius_in_cells - 1, 0);

    World::set_block(
        center_position + IVec3::unit_z() * 2,
        block::Kind::North,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        center_position - IVec3::unit_z() * 2,
        block::Kind::South,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        center_position + IVec3::unit_x() * 2,
        block::Kind::East,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        center_position - IVec3::unit_x() * 2,
        block::Kind::West,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );
}

fn build_clearance_test(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(0, 0, 1);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    World::set_block(
        sector_position + IVec3::new(-4, -4, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(-3, -3, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(-2, -2, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(-1, -1, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(0, 0, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(1, 1, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(2, 2, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(3, 3, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(4, 4, sector_radius_in_cells),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );
}

#[allow(dead_code)]
fn build_sector_room(
    world: &mut World,
    position: IVec3,
    entrance_vec: Vec<grid::Direction>,
    kind: block::Kind,
) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    World::set_box(
        position - IVec3::broadcast(sector_radius_in_cells),
        position + IVec3::broadcast(sector_radius_in_cells),
        kind,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    if entrance_vec.contains(&grid::Direction::XPYOZO) {
        World::set_cube(
            position + IVec3::new(sector_radius_in_cells, 0, 1),
            position + IVec3::new(sector_radius_in_cells, -3, -1),
            block::Kind::None,
            &world.grid,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XNYOZO) {
        World::set_cube(
            position + IVec3::new(-sector_radius_in_cells, 0, 1),
            position + IVec3::new(-sector_radius_in_cells, -3, -1),
            block::Kind::None,
            &world.grid,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XOYOZP) {
        World::set_cube(
            position + IVec3::new(1, 0, sector_radius_in_cells),
            position + IVec3::new(-1, -3, sector_radius_in_cells),
            block::Kind::None,
            &world.grid,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XOYOZN) {
        World::set_cube(
            position + IVec3::new(1, 0, -sector_radius_in_cells),
            position + IVec3::new(-1, -3, -sector_radius_in_cells),
            block::Kind::None,
            &world.grid,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XOYPZO) {
        World::set_cube(
            position + IVec3::new(-1, sector_radius_in_cells, -1),
            position + IVec3::new(1, sector_radius_in_cells, 1),
            block::Kind::None,
            &world.grid,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XOYNZO) {
        World::set_cube(
            position + IVec3::new(-1, -sector_radius_in_cells, -1),
            position + IVec3::new(1, -sector_radius_in_cells, 1),
            block::Kind::None,
            &world.grid,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }
}
