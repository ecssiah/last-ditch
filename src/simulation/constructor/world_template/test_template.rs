use crate::simulation::state::{
    world::{
        block,
        grid::{self, Grid},
    },
    World,
};
use ultraviolet::IVec3;

pub fn construct(world: &mut World) {
    build_rooms(world);
    build_central_room(world);
    build_clearance_test(world);
}

fn build_rooms(world: &mut World) {
    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32 - 1;
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    for z in -world_radius_in_sectors..=world_radius_in_sectors {
        for y in -world_radius_in_sectors..=world_radius_in_sectors {
            for x in -world_radius_in_sectors..=world_radius_in_sectors {
                let sector_coordinates = IVec3::new(x, y, z);
                let sector_position =
                    Grid::sector_coordinates_to_position(sector_coordinates, &world.grid);

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
                    &world.block_info_map,
                    &world.grid,
                    &mut world.sector_vec,
                );
            }
        }
    }
}

fn build_central_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(0, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(sector_coordinates, &world.grid);

    World::set_cube(
        sector_position
            + IVec3::new(
                -3 * sector_radius_in_cells - 1,
                -3 * sector_radius_in_cells - 1,
                -sector_radius_in_cells,
            ),
        sector_position
            + IVec3::new(
                3 * sector_radius_in_cells + 1,
                3 * sector_radius_in_cells + 1,
                sector_radius_in_cells,
            ),
        block::Kind::None,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    let center_position = IVec3::new(0, 0, -sector_radius_in_cells - 1);

    World::set_block(
        center_position + IVec3::unit_y() * 2,
        block::Kind::NorthBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        center_position - IVec3::unit_y() * 2,
        block::Kind::SouthBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        center_position + IVec3::unit_x() * 2,
        block::Kind::EastBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        center_position - IVec3::unit_x() * 2,
        block::Kind::WestBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );
}

fn build_clearance_test(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(0, 1, 0);
    let sector_position = Grid::sector_coordinates_to_position(sector_coordinates, &world.grid);

    World::set_block(
        sector_position + IVec3::new(-4, sector_radius_in_cells, -4),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(-3, sector_radius_in_cells, -3),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(-2, sector_radius_in_cells, -2),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(-1, sector_radius_in_cells, -1),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(0, sector_radius_in_cells, 0),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(1, sector_radius_in_cells, 1),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(2, sector_radius_in_cells, 2),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(3, sector_radius_in_cells, 3),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        sector_position + IVec3::new(4, sector_radius_in_cells, 4),
        block::Kind::CrimsonStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );
}

fn _build_sector_room(
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
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    if entrance_vec.contains(&grid::Direction::East) {
        World::set_cube(
            position + IVec3::new(sector_radius_in_cells, 0, 1),
            position + IVec3::new(sector_radius_in_cells, -3, -1),
            block::Kind::None,
            &world.block_info_map,
            &world.grid,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::West) {
        World::set_cube(
            position + IVec3::new(-sector_radius_in_cells, 0, 1),
            position + IVec3::new(-sector_radius_in_cells, -3, -1),
            block::Kind::None,
            &world.block_info_map,
            &world.grid,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::North) {
        World::set_cube(
            position + IVec3::new(1, 0, sector_radius_in_cells),
            position + IVec3::new(-1, -3, sector_radius_in_cells),
            block::Kind::None,
            &world.block_info_map,
            &world.grid,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::South) {
        World::set_cube(
            position + IVec3::new(1, 0, -sector_radius_in_cells),
            position + IVec3::new(-1, -3, -sector_radius_in_cells),
            block::Kind::None,
            &world.block_info_map,
            &world.grid,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::Up) {
        World::set_cube(
            position + IVec3::new(-1, sector_radius_in_cells, -1),
            position + IVec3::new(1, sector_radius_in_cells, 1),
            block::Kind::None,
            &world.block_info_map,
            &world.grid,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::Down) {
        World::set_cube(
            position + IVec3::new(-1, -sector_radius_in_cells, -1),
            position + IVec3::new(1, -sector_radius_in_cells, 1),
            block::Kind::None,
            &world.block_info_map,
            &world.grid,
            &mut world.sector_vec,
        );
    }
}
