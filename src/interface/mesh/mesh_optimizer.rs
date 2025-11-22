//! Mesh Optimization

use crate::{
    interface::mesh::{block_vertex::BlockVertex, sector_mesh::SectorMesh},
    simulation::{
        observation::view::SectorView,
        state::world::{block, grid::Grid},
    },
};
use std::collections::HashMap;
use ultraviolet::IVec3;

pub fn lysenko_optimization(
    sector_view: &SectorView,
    block_tile_coordinates_map: &HashMap<block::Kind, [[u32; 2]; 6]>,
    grid: &Grid,
) -> SectorMesh {
    let mut vertex_vec = Vec::new();
    let mut index_vec = Vec::new();

    let sector_radius_in_cells = grid.sector_radius_in_cells as i32;
    let sector_size_in_cells = grid.sector_size_in_cells as i32;
    let sector_area_in_cells = grid.sector_area_in_cells as i32;

    for dimension_index in 0..3 {
        let local_x_dimension_index: usize = (dimension_index + 1) % 3;
        let local_y_dimension_index: usize = (dimension_index + 2) % 3;

        let mut mask: Vec<i32> = Vec::with_capacity(sector_area_in_cells as usize);
        mask.resize(sector_area_in_cells as usize, 0);

        let mut cursor_position = IVec3::new(0, 0, 0);

        let cursor_delta = match dimension_index {
            0 => IVec3::unit_x(),
            1 => IVec3::unit_y(),
            2 => IVec3::unit_z(),
            _ => panic!("4th dimension!?"),
        };

        cursor_position[dimension_index] = -sector_radius_in_cells - 1;

        while cursor_position[dimension_index] <= sector_radius_in_cells {
            let mut mask_index: i32 = 0;

            cursor_position[local_y_dimension_index] = 0;

            while cursor_position[local_y_dimension_index] <= sector_radius_in_cells {
                cursor_position[local_x_dimension_index] = 0;

                while cursor_position[local_x_dimension_index] <= sector_radius_in_cells {
                    // let near_face_solid = if cursor_position[dimension_index] >= -sector_radius_in_cells {
                    //     get_cell_at(&cursor_position, grid, sector_view).solid as i32
                    // } else {
                    //     0
                    // };

                    // let far_face_solid = if cursor_position[dimension_index] <= sector_size_in_cells {
                    //     get_cell_at(&(cursor_position + cursor_delta), grid, sector_view).solid as i32
                    // } else {
                    //     0
                    // };

                    // mask[mask_index as usize] = if near_face_solid != 0 && far_face_solid != 0 {
                    //     0
                    // } else if near_face_solid != 0 {
                    //     near_face_solid
                    // } else {
                    //     -far_face_solid
                    // };

                    mask_index += 1;

                    cursor_position[local_x_dimension_index] += 1;
                }

                cursor_position[local_y_dimension_index] += 1;
            }

            cursor_position[dimension_index] += 1;

            mask_index = 0;

            for local_y in -sector_radius_in_cells..sector_radius_in_cells {
                let mut local_x: i32 = -sector_radius_in_cells;

                while local_x <= sector_radius_in_cells {
                    let mut cell = mask[mask_index as usize];

                    if cell == 0 {
                        local_x += 1;
                        mask_index += 1;
                    } else {
                        let mut width: i32 = 1;

                        while cell == mask[(mask_index + width) as usize]
                            && local_x + width <= sector_radius_in_cells
                        {
                            width += 1;
                        }

                        let mut height: i32 = 1;

                        'outer: while local_y + height <= sector_radius_in_cells {
                            for quad_x in 0..width {
                                let quad_mask_index =
                                    mask_index as i32 + quad_x + height * sector_radius_in_cells;

                                if cell != mask[quad_mask_index as usize] {
                                    break 'outer;
                                }
                            }

                            height += 1;
                        }

                        cursor_position[local_x_dimension_index] = local_x;
                        cursor_position[local_y_dimension_index] = local_y;

                        let mut local_x_delta = IVec3::new(0, 0, 0);
                        let mut local_y_delta = IVec3::new(0, 0, 0);

                        if cell > 0 {
                            local_x_delta[local_x_dimension_index] = width;
                            local_y_delta[local_y_dimension_index] = height;
                        } else {
                            cell = -cell;

                            local_x_delta[local_y_dimension_index] = width;
                            local_y_delta[local_x_dimension_index] = height;
                        }

                        let vertex_count: i32 = vertex_vec.len() as i32;

                        let normal = if cell > 0 {
                            [
                                -cursor_delta[0] as f32,
                                -cursor_delta[1] as f32,
                                -cursor_delta[2] as f32,
                            ]
                        } else {
                            [
                                cursor_delta[0] as f32,
                                cursor_delta[1] as f32,
                                cursor_delta[2] as f32,
                            ]
                        };

                        let vertex0 = BlockVertex {
                            position: [
                                cursor_position[0] as f32,
                                cursor_position[1] as f32,
                                cursor_position[2] as f32,
                            ],
                            normal,
                            uv: [0.0, 0.0],
                        };

                        let vertex1 = BlockVertex {
                            position: [
                                (cursor_position[0] + local_x_delta[0]) as f32,
                                (cursor_position[1] + local_x_delta[1]) as f32,
                                (cursor_position[2] + local_x_delta[2]) as f32,
                            ],
                            normal,
                            uv: [0.0, 0.0],
                        };

                        let vertex2 = BlockVertex {
                            position: [
                                (cursor_position[0] + local_x_delta[0] + local_y_delta[0]) as f32,
                                (cursor_position[1] + local_x_delta[1] + local_y_delta[1]) as f32,
                                (cursor_position[2] + local_x_delta[2] + local_y_delta[2]) as f32,
                            ],
                            normal,
                            uv: [0.0, 0.0],
                        };

                        let vertex3 = BlockVertex {
                            position: [
                                (cursor_position[0] + local_y_delta[0]) as f32,
                                (cursor_position[1] + local_y_delta[1]) as f32,
                                (cursor_position[2] + local_y_delta[2]) as f32,
                            ],
                            normal,
                            uv: [0.0, 0.0],
                        };

                        vertex_vec.push(vertex0);
                        vertex_vec.push(vertex1);
                        vertex_vec.push(vertex2);
                        vertex_vec.push(vertex3);

                        let index0 = (vertex_count + 0) as u32;
                        let index1 = (vertex_count + 1) as u32;
                        let index2 = (vertex_count + 2) as u32;
                        let index3 = (vertex_count + 3) as u32;

                        index_vec.push(index0);
                        index_vec.push(index1);
                        index_vec.push(index2);

                        index_vec.push(index0);
                        index_vec.push(index2);
                        index_vec.push(index3);

                        for local_y in 0..height {
                            for local_x in 0..width {
                                let quad_mask_index =
                                    mask_index as i32 + local_x + local_y * sector_size_in_cells;

                                mask[quad_mask_index as usize] = 0;
                            }
                        }

                        local_x += 1;
                        mask_index += width;
                    }
                }
            }
        }
    }

    SectorMesh {
        sector_id: sector_view.sector_id,
        version: sector_view.version,
        vertex_vec: vertex_vec,
        index_vec: index_vec,
    }
}

/// Mikola Lysenko
/// <https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/>
pub fn lysenko_optimization_original(volume: Vec<i32>, dimensions: IVec3) {
    let mut vertex_vec = Vec::new();
    let mut index_vec = Vec::new();

    for dimension_index in 0..3 {
        let local_x_dimension_index: usize = (dimension_index + 1) % 3;
        let local_y_dimension_index: usize = (dimension_index + 2) % 3;

        let slice_cell_count =
            (dimensions[local_x_dimension_index] * dimensions[local_y_dimension_index]) as usize;

        let mut mask: Vec<i32> = Vec::with_capacity(slice_cell_count);
        mask.resize(slice_cell_count, 0);

        let mut cursor_position = IVec3::new(0, 0, 0);

        let cursor_delta = match dimension_index {
            0 => IVec3::unit_x(),
            1 => IVec3::unit_y(),
            2 => IVec3::unit_z(),
            _ => panic!("4th dimension!?"),
        };

        cursor_position[dimension_index] = -1;

        while cursor_position[dimension_index] < dimensions[dimension_index] {
            let mut mask_index: i32 = 0;

            cursor_position[local_y_dimension_index] = 0;

            while cursor_position[local_y_dimension_index] < dimensions[local_y_dimension_index] {
                cursor_position[local_x_dimension_index] = 0;

                while cursor_position[local_x_dimension_index] < dimensions[local_x_dimension_index]
                {
                    let near_face_cell = if 0 <= cursor_position[dimension_index] {
                        cell_at(cursor_position, &volume, &dimensions)
                    } else {
                        0
                    };

                    let far_face_cell =
                        if cursor_position[dimension_index] < dimensions[dimension_index] - 1 {
                            cell_at(cursor_position + cursor_delta, &volume, &dimensions)
                        } else {
                            0
                        };

                    mask[mask_index as usize] = if near_face_cell != 0 && far_face_cell != 0 {
                        0
                    } else if near_face_cell != 0 {
                        near_face_cell
                    } else {
                        -far_face_cell
                    };

                    mask_index += 1;

                    cursor_position[local_x_dimension_index] += 1;
                }

                cursor_position[local_y_dimension_index] += 1;
            }

            cursor_position[dimension_index] += 1;

            mask_index = 0;

            for local_y in 0..dimensions[local_y_dimension_index] {
                let mut local_x: i32 = 0;

                while local_x < dimensions[local_x_dimension_index] {
                    let mut cell = mask[mask_index as usize];

                    if cell == 0 {
                        local_x += 1;
                        mask_index += 1;
                    } else {
                        let mut width: i32 = 1;

                        while cell == mask[(mask_index + width) as usize]
                            && local_x + width < dimensions[local_x_dimension_index]
                        {
                            width += 1;
                        }

                        let mut height: i32 = 1;

                        'outer: while local_y + height < dimensions[local_y_dimension_index] {
                            for quad_x in 0..width {
                                let quad_mask_index = mask_index as i32
                                    + quad_x
                                    + height * dimensions[local_x_dimension_index];

                                if cell != mask[quad_mask_index as usize] {
                                    break 'outer;
                                }
                            }

                            height += 1;
                        }

                        cursor_position[local_x_dimension_index] = local_x;
                        cursor_position[local_y_dimension_index] = local_y;

                        let mut local_x_delta = IVec3::new(0, 0, 0);
                        let mut local_y_delta = IVec3::new(0, 0, 0);

                        if cell > 0 {
                            local_x_delta[local_x_dimension_index] = width;
                            local_y_delta[local_y_dimension_index] = height;
                        } else {
                            cell = -cell;

                            local_x_delta[local_y_dimension_index] = width;
                            local_y_delta[local_x_dimension_index] = height;
                        }

                        let vertex_count: i32 = vertex_vec.len() as i32;

                        // index 0 - lower left
                        vertex_vec.push([
                            cursor_position[0],
                            cursor_position[1],
                            cursor_position[2],
                        ]);

                        // index 1 - lower right
                        vertex_vec.push([
                            cursor_position[0] + local_x_delta[0],
                            cursor_position[1] + local_x_delta[1],
                            cursor_position[2] + local_x_delta[2],
                        ]);

                        // index 2 - upper right
                        vertex_vec.push([
                            cursor_position[0] + local_x_delta[0] + local_y_delta[0],
                            cursor_position[1] + local_x_delta[1] + local_y_delta[1],
                            cursor_position[2] + local_x_delta[2] + local_y_delta[2],
                        ]);

                        // index 3 - upper left
                        vertex_vec.push([
                            cursor_position[0] + local_y_delta[0],
                            cursor_position[1] + local_y_delta[1],
                            cursor_position[2] + local_y_delta[2],
                        ]);

                        let index0 = vertex_count + 0;
                        let index1 = vertex_count + 1;
                        let index2 = vertex_count + 2;
                        let index3 = vertex_count + 3;

                        index_vec.push([index0, index1, index2, cell]);
                        index_vec.push([index0, index2, index3, cell]);

                        for local_y in 0..height {
                            for local_x in 0..width {
                                let quad_mask_index = mask_index as i32
                                    + local_x
                                    + local_y * dimensions[local_x_dimension_index];

                                mask[quad_mask_index as usize] = 0;
                            }
                        }

                        local_x += 1;
                        mask_index += width;
                    }
                }
            }
        }
    }
}

fn grid_to_index(position: IVec3, dimensions: &IVec3) -> i32 {
    position.x + dimensions.x * (position.y + dimensions.y * position.z)
}

fn cell_at(position: IVec3, volume: &Vec<i32>, dimensions: &IVec3) -> i32 {
    let index = grid_to_index(position, dimensions);

    volume[index as usize]
}
