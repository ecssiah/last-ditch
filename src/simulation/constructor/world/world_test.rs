use crate::simulation::state::{
    world::{
        cell,
        grid::{self, Grid},
    },
    World,
};
use glam::IVec3;

pub fn construct(world: &mut World) {
    build_rooms(world);
    build_central_room(world);
    build_clearance_test(world);

    World::set_cell_kind(
        IVec3::new(-9, -2, 0),
        cell::Kind::EsayaBlock,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::update_sectors(&world.grid, &mut world.sector_vec);
}

fn build_rooms(world: &mut World) {
    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32 - 1;
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    for x in -world_radius_in_sectors..=world_radius_in_sectors {
        for y in -world_radius_in_sectors..=world_radius_in_sectors {
            for z in -world_radius_in_sectors..=world_radius_in_sectors {
                let sector_coordinates = IVec3::new(x, y, z);
                let sector_position =
                    Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

                let component_sum =
                    sector_coordinates.x + sector_coordinates.y + sector_coordinates.z;

                let sector_kind = if component_sum % 2 == 0 {
                    cell::Kind::Polished2
                } else {
                    cell::Kind::Polished1
                };

                World::set_cube(
                    sector_position - sector_radius_in_cells,
                    sector_position + sector_radius_in_cells,
                    sector_kind,
                    &world.grid,
                    &world.cell_info_map,
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
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    let center_position = IVec3::new(0, -sector_radius_in_cells - 1, 0);

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

fn build_clearance_test(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let sector_coordinates = IVec3::new(0, 0, 1);
    let sector_position = Grid::sector_coordinates_to_position(&world.grid, sector_coordinates);

    World::set_cell_kind(
        sector_position + IVec3::new(-4, -4, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(-3, -3, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(-2, -2, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(-1, -1, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(0, 0, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(1, 1, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(2, 2, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(3, 3, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cell_kind(
        sector_position + IVec3::new(4, 4, sector_radius_in_cells),
        cell::Kind::CrimsonStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

#[allow(dead_code)]
fn build_sector_room(
    world: &mut World,
    position: IVec3,
    entrance_vec: Vec<grid::Direction>,
    kind: cell::Kind,
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
