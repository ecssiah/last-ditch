//! Mesh Optimization

use ultraviolet::IVec3;

/// Mikola Lysenko
/// <https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/>
pub fn lysenko_optimization(volume: Vec<i32>, dimensions: IVec3) {
    let mut vertex_array = Vec::new();
    let mut index_array = Vec::new();

    for dimension_index in 0..3 {
        let local_x_dimension_index: usize = (dimension_index + 1) % 3;
        let local_y_dimension_index: usize = (dimension_index + 2) % 3;

        let slice_cell_count =
            (dimensions[local_x_dimension_index] * dimensions[local_y_dimension_index]) as usize;

        let mut mask: Vec<i32> = Vec::with_capacity(slice_cell_count);
        mask.resize(slice_cell_count, 0);

        let mut cursor_position = IVec3::new(0, 0, 0);

        let cursor_delta = match dimension_index {
            0 => IVec3::new(1, 0, 0),
            1 => IVec3::new(0, 1, 0),
            2 => IVec3::new(0, 0, 1),
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

                        let vertex_count: i32 = vertex_array.len() as i32;

                        vertex_array.push([
                            cursor_position[0],
                            cursor_position[1],
                            cursor_position[2],
                        ]);

                        vertex_array.push([
                            cursor_position[0] + local_x_delta[0],
                            cursor_position[1] + local_x_delta[1],
                            cursor_position[2] + local_x_delta[2],
                        ]);

                        vertex_array.push([
                            cursor_position[0] + local_x_delta[0] + local_y_delta[0],
                            cursor_position[1] + local_x_delta[1] + local_y_delta[1],
                            cursor_position[2] + local_x_delta[2] + local_y_delta[2],
                        ]);

                        vertex_array.push([
                            cursor_position[0] + local_y_delta[0],
                            cursor_position[1] + local_y_delta[1],
                            cursor_position[2] + local_y_delta[2],
                        ]);

                        index_array.push([vertex_count, vertex_count + 1, vertex_count + 2, cell]);

                        index_array.push([vertex_count, vertex_count + 2, vertex_count + 3, cell]);

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
