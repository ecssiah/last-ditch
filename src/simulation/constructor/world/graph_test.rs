use crate::simulation::state::world::{
    cell,
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

    World::update_sectors(&world.grid, &mut world.sector_vec);
}

fn build_center_room(world: &mut World) {
    let sector_coordinates = IVec3::new(0, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = Vec::from([
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
    ]);

    build_sector_room(sector_position, entrances, cell::Kind::Polished1, world);

    let center_position = IVec3::new(0, -4, 0);

    World::set_cell_kind(
        center_position + IVec3::Z * 2,
        cell::Kind::North,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        center_position - IVec3::Z * 2,
        cell::Kind::South,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        center_position + IVec3::X * 2,
        cell::Kind::East,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        center_position - IVec3::X * 2,
        cell::Kind::West,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

fn build_constricted_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(-1, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = Vec::from([
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ]);

    build_sector_room(sector_position, entrances, cell::Kind::Polished2, world);

    World::set_cube(
        sector_position + IVec3::new(sector_radius_in_cells, 0, 0),
        sector_position + IVec3::new(sector_radius_in_cells, -2, 0),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(sector_radius_in_cells + 1, 1, -1),
        sector_position + IVec3::new(sector_radius_in_cells + 1, -3, 1),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

fn build_expanded_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(1, 0, 0);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = Vec::from([
        grid::Direction::XpYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ]);

    build_sector_room(sector_position, entrances, cell::Kind::Polished2, world);

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells, 1, -2),
        sector_position + IVec3::new(-sector_radius_in_cells + 2, -4, 2),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells - 1, 0, -1),
        sector_position + IVec3::new(-sector_radius_in_cells - 1, -3, 1),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells, -5, -2),
        sector_position + IVec3::new(-sector_radius_in_cells + 2, -5, 2),
        cell::Kind::Polished2,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

fn build_multiple_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(0, 0, -1);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    let entrances = vec![
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZn,
    ];

    build_sector_room(sector_position, entrances, cell::Kind::Polished2, world);

    World::set_cube(
        sector_position + IVec3::new(-sector_radius_in_cells + 1, 0, sector_radius_in_cells),
        sector_position + IVec3::new(-sector_radius_in_cells + 2, -3, sector_radius_in_cells + 1),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position + IVec3::new(sector_radius_in_cells - 2, 0, sector_radius_in_cells),
        sector_position + IVec3::new(sector_radius_in_cells - 1, -3, sector_radius_in_cells + 1),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

fn build_vertical_entrance_room(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates_001 = IVec3::new(0, 0, 1);
    let sector_position_001 =
        Grid::sector_coordinates_to_position(&world.grid, sector_coordinates_001);

    let entrance_vec = vec![
        grid::Direction::XpYoZo,
        grid::Direction::XnYoZo,
        grid::Direction::XoYoZp,
        grid::Direction::XoYoZn,
    ];

    build_sector_room(
        sector_position_001,
        entrance_vec,
        cell::Kind::Polished2,
        world,
    );

    let sector_coordinates_011 = IVec3::new(0, 1, 1);
    let sector_position_011 =
        Grid::sector_coordinates_to_position(&world.grid, sector_coordinates_011);

    let entrance_vec = vec![];

    build_sector_room(
        sector_position_011,
        entrance_vec,
        cell::Kind::Polished1,
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
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, -3, -2),
        sector_position_001 + IVec3::new(3, -3, -3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, -2, -1),
        sector_position_001 + IVec3::new(3, -2, -1),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, -1, 0),
        sector_position_001 + IVec3::new(3, -1, 0),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, 0, 1),
        sector_position_001 + IVec3::new(3, 0, 1),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, 1, 2),
        sector_position_001 + IVec3::new(3, 1, 3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(1, 2, 2),
        sector_position_001 + IVec3::new(1, 2, 3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(0, 3, 2),
        sector_position_001 + IVec3::new(0, 3, 3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-1, 4, 2),
        sector_position_001 + IVec3::new(-1, 4, 3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-3, 5, 0),
        sector_position_001 + IVec3::new(-2, 5, 3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, -3, 2),
        sector_position_001 + IVec3::new(-3, -3, 3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, -2, 1),
        sector_position_001 + IVec3::new(-3, -2, 1),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, -1, 0),
        sector_position_001 + IVec3::new(-3, -1, 0),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, 0, -1),
        sector_position_001 + IVec3::new(-3, 0, -1),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-2, 1, -2),
        sector_position_001 + IVec3::new(-3, 1, -3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-1, 2, -2),
        sector_position_001 + IVec3::new(-1, 2, -3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(0, 3, -2),
        sector_position_001 + IVec3::new(0, 3, -3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(1, 4, -2),
        sector_position_001 + IVec3::new(1, 4, -3),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(2, 5, -3),
        sector_position_001 + IVec3::new(3, 5, 0),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        sector_position_001 + IVec3::new(-1, 5, 0),
        sector_position_001 + IVec3::new(1, 5, 0),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

fn build_sector_room(
    position: IVec3,
    entrance_vec: Vec<grid::Direction>,
    kind: cell::Kind,
    world: &mut World,
) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    World::set_box(
        position - IVec3::splat(sector_radius_in_cells),
        position + IVec3::splat(sector_radius_in_cells),
        kind,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        position - IVec3::Y * sector_radius_in_cells,
        cell::Kind::TealStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    if entrance_vec.contains(&grid::Direction::XpYoZo) {
        World::set_cube(
            position + IVec3::new(sector_radius_in_cells, 0, 1),
            position + IVec3::new(sector_radius_in_cells, -3, -1),
            cell::Kind::Empty,
            &world.grid,
            &world.cell_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XnYoZo) {
        World::set_cube(
            position + IVec3::new(-sector_radius_in_cells, 0, 1),
            position + IVec3::new(-sector_radius_in_cells, -3, -1),
            cell::Kind::Empty,
            &world.grid,
            &world.cell_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYoZp) {
        World::set_cube(
            position + IVec3::new(1, 0, sector_radius_in_cells),
            position + IVec3::new(-1, -3, sector_radius_in_cells),
            cell::Kind::Empty,
            &world.grid,
            &world.cell_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYoZn) {
        World::set_cube(
            position + IVec3::new(1, 0, -sector_radius_in_cells),
            position + IVec3::new(-1, -3, -sector_radius_in_cells),
            cell::Kind::Empty,
            &world.grid,
            &world.cell_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYpZo) {
        World::set_cube(
            position + IVec3::new(-1, sector_radius_in_cells, -1),
            position + IVec3::new(1, sector_radius_in_cells, 1),
            cell::Kind::Empty,
            &world.grid,
            &world.cell_info_map,
            &mut world.sector_vec,
        );
    }

    if entrance_vec.contains(&grid::Direction::XoYnZo) {
        World::set_cube(
            position + IVec3::new(-1, -sector_radius_in_cells, -1),
            position + IVec3::new(1, -sector_radius_in_cells, 1),
            cell::Kind::Empty,
            &world.grid,
            &world.cell_info_map,
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
        cell::Kind::Polished1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        IVec3::new(13, 12, 13),
        cell::Kind::EsayaBlock,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}
