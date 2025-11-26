use crate::simulation::state::{
    population::nation,
    world::{block, grid::Grid, World},
};
use ultraviolet::IVec3;

pub fn construct(world: &mut World) {
    build_ground(world);
    build_compass(world);
    build_temple(0, 34, 0, nation::Kind::Eagle, world);
    build_temple(-34, 0, 0, nation::Kind::Lion, world);
    build_temple(0, -34, 0, nation::Kind::Horse, world);
    build_temple(34, 0, 0, nation::Kind::Wolf, world);

    build_observation_deck(world);
}

fn build_ground(world: &mut World) {
    let ground_boundary =
        (world.grid.world_radius_in_cells - world.grid.sector_size_in_cells) as isize;

    for z in -1..=0 {
        for y in -ground_boundary..=ground_boundary {
            for x in -ground_boundary..=ground_boundary {
                let position = IVec3::new(x as i32, y as i32, z as i32);

                let sector_coordinates =
                    Grid::position_to_sector_coordinates(position, &world.grid);

                let component_sum =
                    sector_coordinates.x + sector_coordinates.y + sector_coordinates.z;

                let kind = if component_sum % 2 == 0 {
                    block::Kind::Polished1
                } else {
                    block::Kind::Polished2
                };

                World::set_block(
                    position,
                    kind,
                    &world.block_info_map,
                    &world.grid,
                    &mut world.sector_vec,
                );
            }
        }
    }
}

fn build_compass(world: &mut World) {
    World::set_block(
        IVec3::new(0, 0, 0),
        block::Kind::TealStone,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        IVec3::new(0, 4, 6),
        block::Kind::NorthBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        IVec3::new(-4, 0, 6),
        block::Kind::WestBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        IVec3::new(0, -4, 6),
        block::Kind::SouthBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_block(
        IVec3::new(4, 0, 6),
        block::Kind::EastBlock,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );
}

fn build_temple(x: i32, y: i32, z: i32, nation_kind: nation::Kind, world: &mut World) {
    world
        .flag_position_map
        .insert(nation_kind, IVec3::new(x, y, z + 3));

    World::set_block(
        IVec3::new(x, y, z + 6),
        nation_kind.icon_block(),
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 8, y - 8, z + 1),
        IVec3::new(x + 8, y + 8, z + 1),
        block::Kind::Stone1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 7, y - 7, z + 2),
        IVec3::new(x + 7, y + 7, z + 2),
        block::Kind::Stone1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 6, y - 6, z + 8),
        IVec3::new(x + 6, y + 6, z + 8),
        block::Kind::Stone1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y - 5, z + 9),
        IVec3::new(x + 5, y + 5, z + 9),
        block::Kind::Stone1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y - 5, z + 8),
        IVec3::new(x + 5, y + 5, z + 8),
        block::Kind::None,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x + 5, y + 5, z + 1),
        IVec3::new(x + 5, y + 5, z + 8),
        block::Kind::Engraved1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y + 5, z + 1),
        IVec3::new(x - 5, y + 5, z + 8),
        block::Kind::Engraved1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x + 5, y - 5, z + 1),
        IVec3::new(x + 5, y - 5, z + 8),
        block::Kind::Engraved1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(x - 5, y - 5, z + 1),
        IVec3::new(x - 5, y - 5, z + 8),
        block::Kind::Engraved1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );
}

fn build_observation_deck(world: &mut World) {
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let sector_size_in_cells = world.grid.sector_size_in_cells as i32;

    let height = 16;
    let center = 3 * sector_size_in_cells;

    World::set_cube(
        IVec3::new(-center + 1, -center + 1, height),
        IVec3::new(-center - 1, -center - 1, 0),
        block::Kind::Polished2,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(center + 1, -center + 1, height),
        IVec3::new(center - 1, -center - 1, 0),
        block::Kind::Polished2,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(-center + 1, center + 1, height),
        IVec3::new(-center - 1, center - 1, 0),
        block::Kind::Polished2,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(center + 1, center + 1, height),
        IVec3::new(center - 1, center - 1, 0),
        block::Kind::Polished2,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(
            -center - sector_radius_in_cells,
            -center - sector_radius_in_cells,
            height,
        ),
        IVec3::new(
            center + sector_radius_in_cells,
            center + sector_radius_in_cells,
            height,
        ),
        block::Kind::Polished1,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );

    World::set_cube(
        IVec3::new(
            -center + sector_radius_in_cells + 1,
            -center + sector_radius_in_cells + 1,
            height,
        ),
        IVec3::new(
            center - sector_radius_in_cells - 1,
            center - sector_radius_in_cells - 1,
            height,
        ),
        block::Kind::None,
        &world.block_info_map,
        &world.grid,
        &mut world.sector_vec,
    );
}
