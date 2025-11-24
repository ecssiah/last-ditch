//! Mesh Optimization

use crate::{
    interface::mesh::{block_vertex::BlockVertex, sector_mesh::SectorMesh},
    simulation::{
        observation::{face_mask, view::SectorView},
        state::world::grid::Grid,
    },
};
use ultraviolet::IVec3;

pub fn lysenko_optimization(sector_view: &SectorView, grid: &Grid) -> SectorMesh {
    let mask_vec = generate_mask_vec(sector_view, grid);
    let sector_mesh = merge_geometry(sector_view, mask_vec);

    sector_mesh
}

fn generate_mask_vec(sector_view: &SectorView, grid: &Grid) -> Vec<Vec<Vec<i16>>> {
    let sector_radius_in_cells = grid.sector_radius_in_cells as i32;
    let sector_size_in_cells = grid.sector_size_in_cells as i32;
    let sector_area_in_cells = grid.sector_area_in_cells as i32;

    let mut mask_vec: Vec<Vec<Vec<i16>>> = vec![
        vec![vec![0; sector_area_in_cells as usize]; (sector_size_in_cells + 1) as usize],
        vec![vec![0; sector_area_in_cells as usize]; (sector_size_in_cells + 1) as usize],
        vec![vec![0; sector_area_in_cells as usize]; (sector_size_in_cells + 1) as usize],
    ];

    for z in -sector_radius_in_cells..=sector_radius_in_cells {
        for y in -sector_radius_in_cells..=sector_radius_in_cells {
            for x in -sector_radius_in_cells..=sector_radius_in_cells {
                let cell_coordinates = IVec3::new(x, y, z);
                let cell_id = Grid::cell_coordinates_to_cell_id(cell_coordinates, grid);

                let block_view_option = &sector_view.block_view_vec[usize::from(cell_id)];

                if let Some(block_view) = block_view_option {
                    if face_mask::has(face_mask::EAST, &block_view.face_mask) {
                        let slice_index = (x + sector_radius_in_cells + 1) as usize;

                        let local_y = (y + sector_radius_in_cells) as usize;
                        let local_z = (z + sector_radius_in_cells) as usize;

                        let mask_index = local_y * (sector_size_in_cells as usize) + local_z;

                        mask_vec[0][slice_index][mask_index] = block_view.block_kind as i16;
                    }

                    if face_mask::has(face_mask::WEST, &block_view.face_mask) {
                        let slice_index = (x + sector_radius_in_cells) as usize;

                        let local_y = (y + sector_radius_in_cells) as usize;
                        let local_z = (z + sector_radius_in_cells) as usize;

                        let mask_index = local_y * (sector_size_in_cells as usize) + local_z;

                        mask_vec[0][slice_index][mask_index] = -(block_view.block_kind as i16);
                    }

                    if face_mask::has(face_mask::NORTH, &block_view.face_mask) {
                        let slice_index = (y + sector_radius_in_cells + 1) as usize;

                        let local_x = (x + sector_radius_in_cells) as usize;
                        let local_z = (z + sector_radius_in_cells) as usize;

                        let mask_index = local_z * (sector_size_in_cells as usize) + local_x;

                        mask_vec[1][slice_index][mask_index] = block_view.block_kind as i16;
                    }

                    if face_mask::has(face_mask::SOUTH, &block_view.face_mask) {
                        let slice_index = (y + sector_radius_in_cells) as usize;

                        let local_x = (x + sector_radius_in_cells) as usize;
                        let local_z = (z + sector_radius_in_cells) as usize;

                        let mask_index = local_z * (sector_size_in_cells as usize) + local_x;

                        mask_vec[1][slice_index][mask_index] = -(block_view.block_kind as i16);
                    }

                    if face_mask::has(face_mask::UP, &block_view.face_mask) {
                        let slice_index = (z + sector_radius_in_cells + 1) as usize;

                        let local_x = (x + sector_radius_in_cells) as usize;
                        let local_y = (y + sector_radius_in_cells) as usize;

                        let mask_index = local_y * (sector_size_in_cells as usize) + local_x;

                        mask_vec[2][slice_index][mask_index] = block_view.block_kind as i16;
                    }

                    if face_mask::has(face_mask::DOWN, &block_view.face_mask) {
                        let slice_index = (z + sector_radius_in_cells) as usize;

                        let local_x = (x + sector_radius_in_cells) as usize;
                        let local_y = (y + sector_radius_in_cells) as usize;

                        let mask_index = local_y * (sector_size_in_cells as usize) + local_x;

                        mask_vec[2][slice_index][mask_index] = -(block_view.block_kind as i16);
                    }
                }
            }
        }
    }

    mask_vec
}

fn merge_geometry(sector_view: &SectorView, mask_vec: Vec<Vec<Vec<i16>>>) -> SectorMesh {
    let mut vertex_vec = Vec::new();
    let mut index_vec = Vec::new();

    for dimension_vec in mask_vec {
        for slice_vec in dimension_vec {
            
        }
    }

    SectorMesh {
        sector_id: sector_view.sector_id,
        version: sector_view.version,
        vertex_vec,
        index_vec,
    }
}
