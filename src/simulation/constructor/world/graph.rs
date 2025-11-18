use crate::simulation::state::world::{
    block,
    grid::{self, Grid},
    World,
};
use ultraviolet::IVec3;

pub fn run(world: &mut World) {
    build_center_room(world);

    build_vertical_entrance_room(world);
    build_constricted_entrance_room(world);
    build_expanded_entrance_room(world);
    build_multiple_entrance_room(world);

    // build_floor(world);

    World::update_sectors(&world.grid, &mut world.sector_vec);
}

fn build_center_room(world: &mut World) {
    let sector_coordinates = IVec3::new(0, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = Vec::from([
        grid::Direction::XPYOZO,
        grid::Direction::XNYOZO,
        grid::Direction::XOYOZP,
    ]);

    build_sector_room(sector_position, entrances, block::Kind::Polished1, world);

    let center_position = IVec3::new(0, 0, -4);

    World::set_block(
        center_position + IVec3::unit_y() * 2,
        block::Kind::North,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        center_position - IVec3::unit_y() * 2,
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

fn build_constricted_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(-1, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = Vec::from([
        grid::Direction::XNYOZO,
        grid::Direction::XOYPZO,
        grid::Direction::XOYNZO,
    ]);

    build_sector_room(sector_position, entrances, block::Kind::Polished2, world);

    World::set_cube(
        sector_position + IVec3::new(sector_radius_in_cells, 0, 0),
        sector_position + IVec3::new(sector_radius_in_cells, 0, -2),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(sector_radius_in_cells + 1, -1, 1),
        sector_position + IVec3::new(sector_radius_in_cells + 1, 1, -3),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );
}

fn build_expanded_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(1, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = Vec::from([
        grid::Direction::XPYOZO,
        grid::Direction::XOYPZO,
        grid::Direction::XOYNZO,
    ]);

    build_sector_room(sector_position, entrances, block::Kind::Polished2, world);

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells, -2, 1),
        sector_position + IVec3::new(-sector_radius_in_cells + 2, 2, -4),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells - 1, -1, 0),
        sector_position + IVec3::new(-sector_radius_in_cells - 1, 1, -3),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells, -2, -5),
        sector_position + IVec3::new(-sector_radius_in_cells + 2, 2, -5),
        block::Kind::Polished2,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );
}

fn build_multiple_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(0, -1, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = vec![
        grid::Direction::XPYOZO,
        grid::Direction::XNYOZO,
        grid::Direction::XOYNZO,
    ];

    build_sector_room(sector_position, entrances, block::Kind::Polished2, world);

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells + 1, 0, sector_radius_in_cells),
        sector_position + IVec3::new(-sector_radius_in_cells + 2, -3, sector_radius_in_cells + 1),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(sector_radius_in_cells - 2, 0, sector_radius_in_cells),
        sector_position + IVec3::new(sector_radius_in_cells - 1, -3, sector_radius_in_cells + 1),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );
}

fn build_vertical_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates_001 = IVec3::new(0, 0, 1);
    let sector_position_001 =
        Grid::sector_coordinates_to_position(&world.grid, sector_coordinates_001);

    let entrance_vec = vec![
        grid::Direction::XPYOZO,
        grid::Direction::XNYOZO,
        grid::Direction::XOYOZP,
        grid::Direction::XOYOZN,
    ];

    build_sector_room(
        sector_position_001,
        entrance_vec,
        block::Kind::Polished2,
        world,
    );

    let sector_coordinates_011 = IVec3::new(0, 1, 1);
    let sector_position_011 =
        Grid::sector_coordinates_to_position(&world.grid, sector_coordinates_011);

    let entrance_vec = vec![];

    build_sector_room(
        sector_position_011,
        entrance_vec,
        block::Kind::Polished1,
        world,
    );

    World::set_box(
        sector_position_001
            + IVec3::new(
                -sector_radius_in_cells + 1,
                sector_radius_in_cells,
                -sector_radius_in_cells + 1,
            ),
        sector_position_001
            + IVec3::new(
                sector_radius_in_cells - 1,
                sector_radius_in_cells + 1,
                sector_radius_in_cells - 1,
            ),
        block::Kind::None,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, -3, -2),
        sector_position_001 + IVec3::new(3, -3, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, -2, -1),
        sector_position_001 + IVec3::new(3, -2, -1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, -1, 0),
        sector_position_001 + IVec3::new(3, -1, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, 0, 1),
        sector_position_001 + IVec3::new(3, 0, 1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, 1, 2),
        sector_position_001 + IVec3::new(3, 1, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(1, 2, 2),
        sector_position_001 + IVec3::new(1, 2, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(0, 3, 2),
        sector_position_001 + IVec3::new(0, 3, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-1, 4, 2),
        sector_position_001 + IVec3::new(-1, 4, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-3, 5, 0),
        sector_position_001 + IVec3::new(-2, 5, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, -3, 2),
        sector_position_001 + IVec3::new(-3, -3, 3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, -2, 1),
        sector_position_001 + IVec3::new(-3, -2, 1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, -1, 0),
        sector_position_001 + IVec3::new(-3, -1, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, 0, -1),
        sector_position_001 + IVec3::new(-3, 0, -1),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, 1, -2),
        sector_position_001 + IVec3::new(-3, 1, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-1, 2, -2),
        sector_position_001 + IVec3::new(-1, 2, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(0, 3, -2),
        sector_position_001 + IVec3::new(0, 3, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(1, 4, -2),
        sector_position_001 + IVec3::new(1, 4, -3),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, 5, -3),
        sector_position_001 + IVec3::new(3, 5, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-1, 5, 0),
        sector_position_001 + IVec3::new(1, 5, 0),
        block::Kind::CrimsonStone,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );
}

fn build_sector_room(
    position: IVec3,
    entrance_vec: Vec<grid::Direction>,
    kind: block::Kind,
    world: &mut World,
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

    World::set_block(
        position - IVec3::unit_y() * sector_radius_in_cells,
        block::Kind::TealStone,
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

fn _build_floor(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let sector_size_in_cells = world.grid.sector_size_in_cells as i32;

    World::set_cube(
        IVec3::new(
            -2 * sector_size_in_cells,
            -sector_radius_in_cells - 1,
            -2 * sector_size_in_cells,
        ),
        IVec3::new(
            2 * sector_size_in_cells,
            -sector_radius_in_cells - 1,
            2 * sector_size_in_cells,
        ),
        block::Kind::Polished1,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );

    World::set_block(
        IVec3::new(13, 12, 13),
        block::Kind::EsayaBlock,
        &world.grid,
        &world.block_info_map,
        &mut world.sector_vec,
    );
}
