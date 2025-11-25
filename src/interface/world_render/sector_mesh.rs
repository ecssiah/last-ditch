use crate::{
    interface::{
        gpu::gpu_mesh::GpuMesh,
        world_render::{face::Face, sector_vertex::SectorVertex, tile_atlas::TileAtlas},
    },
    simulation::{
        constants::CELL_RADIUS,
        observation::{face_mask, view::SectorView},
        state::world::{
            block,
            grid::{self, axis::Axis, Grid},
            sector,
        },
    },
};
use ultraviolet::IVec3;

pub struct SectorMesh {
    pub sector_id: sector::ID,
    pub version: u64,
    pub vertex_vec: Vec<SectorVertex>,
    pub index_vec: Vec<u32>,
}

impl SectorMesh {
    pub fn from_sector_view(sector_view: &SectorView, grid: &Grid) -> Self {
        let sector_mesh = Self::lysenko_optimization(sector_view, grid);

        sector_mesh
    }

    fn lysenko_optimization(sector_view: &SectorView, grid: &Grid) -> Self {
        let mask_vec = Self::generate_mask_vec(sector_view, grid);
        let sector_mesh = Self::merge_geometry(sector_view, grid, mask_vec);

        sector_mesh
    }

    fn generate_mask_vec(sector_view: &SectorView, grid: &Grid) -> Vec<Vec<Vec<Face>>> {
        let sector_radius_in_cells = grid.sector_radius_in_cells as i32;
        let sector_size_in_cells = grid.sector_size_in_cells as i32;
        let sector_area_in_cells = grid.sector_area_in_cells as i32;

        let slice_count = (sector_size_in_cells + 1) as usize;

        let mut mask_vec: Vec<Vec<Vec<Face>>> = vec![
            vec![vec![Face::new(); sector_area_in_cells as usize]; slice_count],
            vec![vec![Face::new(); sector_area_in_cells as usize]; slice_count],
            vec![vec![Face::new(); sector_area_in_cells as usize]; slice_count],
        ];

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell_coordinates = IVec3::new(x, y, z);
                    let cell_id = Grid::cell_coordinates_to_cell_id(cell_coordinates, grid);

                    let block_view_option = &sector_view.block_view_vec[cell_id.to_usize()];

                    if let Some(block_view) = block_view_option {
                        if face_mask::has(face_mask::EAST, &block_view.face_mask) {
                            let slice_index = (x + sector_radius_in_cells + 1) as usize;

                            let local_y = (y + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_z;

                            let face = Face {
                                block_kind: block_view.block_kind,
                                direction: grid::Direction::East,
                            };

                            mask_vec[Axis::X as usize][slice_index][mask_index] = face;
                        }

                        if face_mask::has(face_mask::WEST, &block_view.face_mask) {
                            let slice_index = (x + sector_radius_in_cells) as usize;

                            let local_y = (y + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_z;

                            let face = Face {
                                block_kind: block_view.block_kind,
                                direction: grid::Direction::West,
                            };

                            mask_vec[Axis::X as usize][slice_index][mask_index] = face;
                        }

                        if face_mask::has(face_mask::NORTH, &block_view.face_mask) {
                            let slice_index = (y + sector_radius_in_cells + 1) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_z * (sector_size_in_cells as usize) + local_x;

                            let face = Face {
                                block_kind: block_view.block_kind,
                                direction: grid::Direction::North,
                            };

                            mask_vec[Axis::Y as usize][slice_index][mask_index] = face;
                        }

                        if face_mask::has(face_mask::SOUTH, &block_view.face_mask) {
                            let slice_index = (y + sector_radius_in_cells) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_z * (sector_size_in_cells as usize) + local_x;

                            let face = Face {
                                block_kind: block_view.block_kind,
                                direction: grid::Direction::South,
                            };

                            mask_vec[Axis::Y as usize][slice_index][mask_index] = face;
                        }

                        if face_mask::has(face_mask::UP, &block_view.face_mask) {
                            let slice_index = (z + sector_radius_in_cells + 1) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_y = (y + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_x;

                            let face = Face {
                                block_kind: block_view.block_kind,
                                direction: grid::Direction::Up,
                            };

                            mask_vec[Axis::Z as usize][slice_index][mask_index] = face;
                        }

                        if face_mask::has(face_mask::DOWN, &block_view.face_mask) {
                            let slice_index = (z + sector_radius_in_cells) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_y = (y + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_x;

                            let face = Face {
                                block_kind: block_view.block_kind,
                                direction: grid::Direction::Down,
                            };

                            mask_vec[Axis::Z as usize][slice_index][mask_index] = face;
                        }
                    }
                }
            }
        }

        mask_vec
    }

    fn merge_geometry(
        sector_view: &SectorView,
        grid: &Grid,
        mask_vec: Vec<Vec<Vec<Face>>>,
    ) -> Self {
        let mut vertex_vec = Vec::new();
        let mut index_vec = Vec::new();

        for axis in Axis::all() {
            let axis_vec = &mask_vec[axis as usize];
            let slice_count = axis_vec.len();

            for slice_index in 0..slice_count {
                let slice = &axis_vec[slice_index];

                Self::merge_slice(
                    axis,
                    slice_index,
                    slice,
                    grid,
                    sector_view.world_position.as_array(),
                    &mut vertex_vec,
                    &mut index_vec,
                );
            }
        }

        SectorMesh {
            sector_id: sector_view.sector_id,
            version: sector_view.version,
            vertex_vec,
            index_vec,
        }
    }

    fn merge_slice(
        axis: Axis,
        slice_index: usize,
        slice: &[Face],
        grid: &Grid,
        sector_world_position: &[f32; 3],
        vertex_vec: &mut Vec<SectorVertex>,
        index_vec: &mut Vec<u32>,
    ) {
        let sector_size_in_cells = grid.sector_size_in_cells as usize;

        let mut consumed = vec![false; slice.len()];

        for y in 0..sector_size_in_cells {
            for x in 0..sector_size_in_cells {
                let mask_index = y * sector_size_in_cells + x;
                let face = slice[mask_index];

                if face.block_kind == block::Kind::None || consumed[mask_index] {
                    continue;
                }

                let mut x_max = x + 1;

                while x_max < sector_size_in_cells {
                    let test_mask_index = y * sector_size_in_cells + x_max;
                    let test_face = slice[test_mask_index];

                    if test_face != face || consumed[test_mask_index] {
                        break;
                    }

                    x_max += 1;
                }

                let mut y_max = y + 1;

                'outer: while y_max < sector_size_in_cells {
                    for xx in x..x_max {
                        let test_mask_index = y_max * sector_size_in_cells + xx;
                        let test_face = slice[test_mask_index];

                        if test_face != face || consumed[test_mask_index] {
                            break 'outer;
                        }
                    }

                    y_max += 1;
                }

                for yy in y..y_max {
                    for xx in x..x_max {
                        consumed[yy * sector_size_in_cells + xx] = true;
                    }
                }

                Self::emit_triangles(
                    axis,
                    slice_index,
                    x,
                    y,
                    x_max,
                    y_max,
                    face,
                    grid,
                    sector_world_position,
                    vertex_vec,
                    index_vec,
                );
            }
        }
    }

    pub fn emit_triangles(
        axis: Axis,
        slice_index: usize,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        face: Face,
        grid: &Grid,
        sector_world_position: &[f32; 3],
        vertex_vec: &mut Vec<SectorVertex>,
        index_vec: &mut Vec<u32>,
    ) {
        let sector_radius = grid.sector_radius_in_cells as i32;

        let slice_coordinate = Self::get_axis_coordinate(slice_index, sector_radius);

        let (x_min, x_max, y_min, y_max, z_min, z_max) = match axis {
            Axis::X => {
                let row0 = y0;
                let row1 = y1;
                let col0 = x0;
                let col1 = x1;

                let y_min = Self::get_axis_coordinate(row0, sector_radius);
                let y_max = Self::get_axis_coordinate(row1, sector_radius);
                let z_min = Self::get_axis_coordinate(col0, sector_radius);
                let z_max = Self::get_axis_coordinate(col1, sector_radius);

                (
                    slice_coordinate,
                    slice_coordinate,
                    y_min,
                    y_max,
                    z_min,
                    z_max,
                )
            }
            Axis::Y => {
                let row0 = y0;
                let row1 = y1;
                let col0 = x0;
                let col1 = x1;

                let z_min = Self::get_axis_coordinate(row0, sector_radius);
                let z_max = Self::get_axis_coordinate(row1, sector_radius);
                let x_min = Self::get_axis_coordinate(col0, sector_radius);
                let x_max = Self::get_axis_coordinate(col1, sector_radius);

                (
                    x_min,
                    x_max,
                    slice_coordinate,
                    slice_coordinate,
                    z_min,
                    z_max,
                )
            }
            Axis::Z => {
                let row0 = y0;
                let row1 = y1;
                let col0 = x0;
                let col1 = x1;

                let y_min = Self::get_axis_coordinate(row0, sector_radius);
                let y_max = Self::get_axis_coordinate(row1, sector_radius);
                let x_min = Self::get_axis_coordinate(col0, sector_radius);
                let x_max = Self::get_axis_coordinate(col1, sector_radius);

                (
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    slice_coordinate,
                    slice_coordinate,
                )
            }
        };

        let (vertex_position0, vertex_position1, vertex_position2, vertex_position3) =
            if axis == Axis::X {
                (
                    [x_min, y_min, z_min],
                    [x_max, y_max, z_min],
                    [x_max, y_max, z_max],
                    [x_min, y_min, z_max],
                )
            } else {
                (
                    [x_min, y_min, z_min],
                    [x_max, y_min, z_min],
                    [x_max, y_max, z_max],
                    [x_min, y_max, z_max],
                )
            };

        let normal: [f32; 3] = *face.direction.to_vec3().as_array();

        let (width_blocks, height_blocks) = match axis {
            Axis::X => {
                let w = (y1 - y0) as f32;
                let h = (x1 - x0) as f32;
                (w, h)
            }
            Axis::Y => {
                let w = (x1 - x0) as f32;
                let h = (y1 - y0) as f32;
                (w, h)
            }
            Axis::Z => {
                let w = (x1 - x0) as f32;
                let h = (y1 - y0) as f32;
                (w, h)
            }
        };

        let uv0 = [0.0, height_blocks];
        let uv1 = [width_blocks, height_blocks];
        let uv2 = [width_blocks, 0.0];
        let uv3 = [0.0, 0.0];

        let (uv0, uv1, uv2, uv3) = match face.direction {
            grid::Direction::East => (uv0, uv1, uv2, uv3),
            grid::Direction::West => (uv1, uv0, uv3, uv2),
            grid::Direction::North => (uv1, uv0, uv3, uv2),
            grid::Direction::South => (uv0, uv1, uv2, uv3),
            grid::Direction::Up => (uv0, uv1, uv2, uv3),
            grid::Direction::Down => (uv3, uv2, uv1, uv0),
        };

        let base_index = vertex_vec.len() as u32;

        let layer = TileAtlas::get_tile_layer(face.block_kind, face.direction);

        vertex_vec.push(SectorVertex {
            position: [
                vertex_position0[0] + sector_world_position[0],
                vertex_position0[1] + sector_world_position[1],
                vertex_position0[2] + sector_world_position[2],
            ],
            normal,
            uv: uv0,
            layer,
        });

        vertex_vec.push(SectorVertex {
            position: [
                vertex_position1[0] + sector_world_position[0],
                vertex_position1[1] + sector_world_position[1],
                vertex_position1[2] + sector_world_position[2],
            ],
            normal,
            uv: uv1,
            layer,
        });

        vertex_vec.push(SectorVertex {
            position: [
                vertex_position2[0] + sector_world_position[0],
                vertex_position2[1] + sector_world_position[1],
                vertex_position2[2] + sector_world_position[2],
            ],
            normal,
            uv: uv2,
            layer,
        });

        vertex_vec.push(SectorVertex {
            position: [
                vertex_position3[0] + sector_world_position[0],
                vertex_position3[1] + sector_world_position[1],
                vertex_position3[2] + sector_world_position[2],
            ],
            normal,
            uv: uv3,
            layer,
        });

        let use_canonical = match face.direction {
            grid::Direction::East => true,
            grid::Direction::West => false,
            grid::Direction::North => false,
            grid::Direction::South => true,
            grid::Direction::Up => true,
            grid::Direction::Down => false,
        };

        if use_canonical {
            index_vec.push(base_index + 0);
            index_vec.push(base_index + 1);
            index_vec.push(base_index + 2);

            index_vec.push(base_index + 0);
            index_vec.push(base_index + 2);
            index_vec.push(base_index + 3);
        } else {
            index_vec.push(base_index + 0);
            index_vec.push(base_index + 2);
            index_vec.push(base_index + 1);

            index_vec.push(base_index + 0);
            index_vec.push(base_index + 3);
            index_vec.push(base_index + 2);
        }
    }

    fn get_axis_coordinate(slice_index: usize, sector_radius: i32) -> f32 {
        slice_index as f32 - sector_radius as f32 - CELL_RADIUS
    }

    pub fn to_gpu_mesh(sector_mesh: &Self, device: &wgpu::Device) -> GpuMesh {
        assert!(
            !sector_mesh.vertex_vec.is_empty(),
            "Vertex buffer is empty!"
        );

        assert!(!sector_mesh.index_vec.is_empty(), "Index buffer is empty!");

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&sector_mesh.vertex_vec),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&sector_mesh.index_vec),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let index_count = sector_mesh.index_vec.len() as u32;

        let material_id = 0;

        GpuMesh {
            version: sector_mesh.version,
            vertex_buffer,
            index_buffer,
            index_count,
            material_id,
        }
    }
}
