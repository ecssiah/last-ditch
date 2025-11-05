use crate::simulation::state::{
    population::nation,
    world::{cell, grid::Grid, World},
};
use glam::IVec3;

pub fn construct(world: &mut World) {
    build_ground(world);
    build_compass(world);
    build_temple(0, 0, 34, nation::Kind::Eagle, world);
    build_temple(-34, 0, 0, nation::Kind::Lion, world);
    build_temple(0, 0, -34, nation::Kind::Horse, world);
    build_temple(34, 0, 0, nation::Kind::Wolf, world);

    build_observation_deck(world);

    World::update_sectors(&world.grid, &mut world.sector_vec);
}

fn build_ground(world: &mut World) {
    let ground_boundary =
        (world.grid.world_radius_in_cells - world.grid.sector_size_in_cells) as isize;

    for x in -ground_boundary..=ground_boundary {
        for y in -1..=0 {
            for z in -ground_boundary..=ground_boundary {
                let position = IVec3::new(x as i32, y, z as i32);
                let sector_coordinates =
                    Grid::position_to_sector_coordinates(&world.grid, position);

                let component_sum =
                    sector_coordinates.x + sector_coordinates.y + sector_coordinates.z;

                let kind = if component_sum % 2 == 0 {
                    cell::Kind::Polished1
                } else {
                    cell::Kind::Polished2
                };

                World::set_cell_kind(
                    position,
                    kind,
                    &world.grid,
                    &world.cell_info_map,
                    &mut world.sector_vec,
                );
            }
        }
    }
}

fn build_compass(world: &mut World) {
    World::set_cell_kind(
        IVec3::new(0, 0, 0),
        cell::Kind::TealStone,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
    World::set_cell_kind(
        IVec3::new(0, 0, 4),
        cell::Kind::North,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
    World::set_cell_kind(
        IVec3::new(-4, 0, 0),
        cell::Kind::West,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
    World::set_cell_kind(
        IVec3::new(0, 0, -4),
        cell::Kind::South,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
    World::set_cell_kind(
        IVec3::new(4, 0, 0),
        cell::Kind::East,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

fn build_temple(x: i32, y: i32, z: i32, nation_kind: nation::Kind, world: &mut World) {
    world
        .flag_position_map
        .insert(nation_kind, IVec3::new(x, y + 3, z));

    World::set_cell_kind(
        IVec3::new(x, y + 6, z),
        nation_kind.icon(),
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 8, y + 1, z - 8),
        IVec3::new(x + 8, y + 1, z + 8),
        cell::Kind::Stone1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 7, y + 2, z - 7),
        IVec3::new(x + 7, y + 2, z + 7),
        cell::Kind::Stone1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 6, y + 8, z - 6),
        IVec3::new(x + 6, y + 8, z + 6),
        cell::Kind::Stone1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y + 9, z - 5),
        IVec3::new(x + 5, y + 9, z + 5),
        cell::Kind::Stone1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y + 8, z - 5),
        IVec3::new(x + 5, y + 8, z + 5),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x + 5, y + 1, z + 5),
        IVec3::new(x + 5, y + 8, z + 5),
        cell::Kind::Engraved1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y + 1, z + 5),
        IVec3::new(x - 5, y + 8, z + 5),
        cell::Kind::Engraved1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x + 5, y + 1, z - 5),
        IVec3::new(x + 5, y + 8, z - 5),
        cell::Kind::Engraved1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y + 1, z - 5),
        IVec3::new(x - 5, y + 8, z - 5),
        cell::Kind::Engraved1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}

fn build_observation_deck(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let sector_size_in_cells = world.grid.sector_size_in_cells as i32;

    let height = 16;
    let center = 3 * sector_size_in_cells;

    World::set_cube(
        IVec3::new(-center + 1, height, -center + 1),
        IVec3::new(-center - 1, 0, -center - 1),
        cell::Kind::Polished2,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(center + 1, height, -center + 1),
        IVec3::new(center - 1, 0, -center - 1),
        cell::Kind::Polished2,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(-center + 1, height, center + 1),
        IVec3::new(-center - 1, 0, center - 1),
        cell::Kind::Polished2,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(center + 1, height, center + 1),
        IVec3::new(center - 1, 0, center - 1),
        cell::Kind::Polished2,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(
            -center - sector_radius_in_cells,
            height,
            -center - sector_radius_in_cells,
        ),
        IVec3::new(
            center + sector_radius_in_cells,
            height,
            center + sector_radius_in_cells,
        ),
        cell::Kind::Polished1,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(
            -center + sector_radius_in_cells + 1,
            height,
            -center + sector_radius_in_cells + 1,
        ),
        IVec3::new(
            center - sector_radius_in_cells - 1,
            height,
            center - sector_radius_in_cells - 1,
        ),
        cell::Kind::Empty,
        &world.grid,
        &world.cell_info_map,
        &mut world.sector_vec,
    );
}
