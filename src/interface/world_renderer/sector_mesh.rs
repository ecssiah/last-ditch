use crate::{
    interface::{
        gpu::gpu_mesh::GpuMesh,
        world_renderer::{sector_face::SectorFace, sector_vertex::SectorVertex, tile_atlas},
    },
    simulation::{
        constants::*,
        overseer::viewer::view::SectorView,
        state::world::{
            grid::{self, axis::Axis, direction_set::DirectionSet, Direction},
            sector::Sector,
        },
    },
};
use ultraviolet::IVec3;

pub struct SectorMesh {
    pub sector_index: usize,
    pub version: u64,
    pub vertex_vec: Vec<SectorVertex>,
    pub index_vec: Vec<u32>,
}

impl SectorMesh {
    pub fn from_sector_view(sector_view: &SectorView) -> Self {
        let sector_mesh = Self::lysenko_optimization(sector_view);

        sector_mesh
    }

    fn lysenko_optimization(sector_view: &SectorView) -> Self {
        let mask_vec = Self::generate_mask_vec(sector_view);
        let sector_mesh = Self::merge_geometry(sector_view, mask_vec);

        sector_mesh
    }

    fn generate_mask_vec(sector_view: &SectorView) -> Vec<Vec<Vec<Option<SectorFace>>>> {
        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;
        let sector_area_in_cells = SECTOR_AREA_IN_CELLS as i32;

        let slice_count = (sector_size_in_cells + 1) as usize;

        let mut mask_vec = vec![
            vec![vec![None; sector_area_in_cells as usize]; slice_count],
            vec![vec![None; sector_area_in_cells as usize]; slice_count],
            vec![vec![None; sector_area_in_cells as usize]; slice_count],
        ];

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell_coordinate = IVec3::new(x, y, z);
                    let cell_index = grid::cell_coordinate_to_cell_index(cell_coordinate);

                    if let Some(block) = Sector::get_block(cell_index, &sector_view.block_vec) {
                        if DirectionSet::has(Direction::North, &block.exposure_set) {
                            let slice_index = (y + sector_radius_in_cells + 1) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_z * (sector_size_in_cells as usize) + local_x;

                            let sector_face = SectorFace {
                                block_kind: block.block_kind.clone(),
                                direction: Direction::North,
                            };

                            mask_vec[Axis::Y as usize][slice_index][mask_index] = Some(sector_face);
                        }

                        if DirectionSet::has(Direction::West, &block.exposure_set) {
                            let slice_index = (x + sector_radius_in_cells) as usize;

                            let local_y = (y + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_z;

                            let sector_face = SectorFace {
                                block_kind: block.block_kind.clone(),
                                direction: Direction::West,
                            };

                            mask_vec[Axis::X as usize][slice_index][mask_index] = Some(sector_face);
                        }

                        if DirectionSet::has(Direction::South, &block.exposure_set) {
                            let slice_index = (y + sector_radius_in_cells) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_z * (sector_size_in_cells as usize) + local_x;

                            let sector_face = SectorFace {
                                block_kind: block.block_kind.clone(),
                                direction: Direction::South,
                            };

                            mask_vec[Axis::Y as usize][slice_index][mask_index] = Some(sector_face);
                        }

                        if DirectionSet::has(Direction::East, &block.exposure_set) {
                            let slice_index = (x + sector_radius_in_cells + 1) as usize;

                            let local_y = (y + sector_radius_in_cells) as usize;
                            let local_z = (z + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_z;

                            let sector_face = SectorFace {
                                block_kind: block.block_kind.clone(),
                                direction: Direction::East,
                            };

                            mask_vec[Axis::X as usize][slice_index][mask_index] = Some(sector_face);
                        }

                        if DirectionSet::has(Direction::Up, &block.exposure_set) {
                            let slice_index = (z + sector_radius_in_cells + 1) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_y = (y + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_x;

                            let sector_face = SectorFace {
                                block_kind: block.block_kind.clone(),
                                direction: Direction::Up,
                            };

                            mask_vec[Axis::Z as usize][slice_index][mask_index] = Some(sector_face);
                        }

                        if DirectionSet::has(Direction::Down, &block.exposure_set) {
                            let slice_index = (z + sector_radius_in_cells) as usize;

                            let local_x = (x + sector_radius_in_cells) as usize;
                            let local_y = (y + sector_radius_in_cells) as usize;

                            let mask_index = local_y * (sector_size_in_cells as usize) + local_x;

                            let sector_face = SectorFace {
                                block_kind: block.block_kind.clone(),
                                direction: Direction::Down,
                            };

                            mask_vec[Axis::Z as usize][slice_index][mask_index] = Some(sector_face);
                        }
                    }
                }
            }
        }

        mask_vec
    }

    fn merge_geometry(
        sector_view: &SectorView,
        mask_vec: Vec<Vec<Vec<Option<SectorFace>>>>,
    ) -> Self {
        let mut vertex_vec = Vec::new();
        let mut index_vec = Vec::new();

        for axis in Axis::ALL {
            let axis_vec = &mask_vec[axis as usize];
            let slice_count = axis_vec.len();

            for slice_index in 0..slice_count {
                let sector_face_slice = &axis_vec[slice_index];

                Self::merge_slice(
                    axis,
                    slice_index,
                    sector_face_slice,
                    sector_view.world_position.as_array(),
                    &mut vertex_vec,
                    &mut index_vec,
                );
            }
        }

        Self {
            sector_index: sector_view.sector_index,
            version: sector_view.version,
            vertex_vec,
            index_vec,
        }
    }

    fn merge_slice(
        axis: Axis,
        slice_index: usize,
        sector_face_slice: &[Option<SectorFace>],
        sector_world_position: &[f32; 3],
        vertex_vec: &mut Vec<SectorVertex>,
        index_vec: &mut Vec<u32>,
    ) {
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as usize;

        let mut visited = vec![false; sector_face_slice.len()];

        for y in 0..sector_size_in_cells {
            for x in 0..sector_size_in_cells {
                let mask_index = y * sector_size_in_cells + x;

                let face = match sector_face_slice[mask_index].clone() {
                    Some(face) => face,
                    None => continue,
                };

                if visited[mask_index] {
                    continue;
                }

                let mut x_max = x + 1;

                while x_max < sector_size_in_cells {
                    let test_mask_index = y * sector_size_in_cells + x_max;

                    let test_face = match sector_face_slice[test_mask_index].clone() {
                        Some(face) => face,
                        None => break,
                    };

                    if test_face != face || visited[test_mask_index] {
                        break;
                    }

                    x_max += 1;
                }

                let mut y_max = y + 1;

                'outer: while y_max < sector_size_in_cells {
                    for xx in x..x_max {
                        let test_mask_index = y_max * sector_size_in_cells + xx;

                        let test_face = match sector_face_slice[test_mask_index].clone() {
                            Some(face) => face,
                            None => break 'outer,
                        };

                        if test_face != face || visited[test_mask_index] {
                            break 'outer;
                        }
                    }

                    y_max += 1;
                }

                for yy in y..y_max {
                    for xx in x..x_max {
                        visited[yy * sector_size_in_cells + xx] = true;
                    }
                }

                Self::emit_triangles(
                    axis,
                    slice_index,
                    sector_world_position,
                    x,
                    y,
                    x_max,
                    y_max,
                    face,
                    vertex_vec,
                    index_vec,
                );
            }
        }
    }

    pub fn emit_triangles(
        axis: Axis,
        slice_index: usize,
        sector_world_position: &[f32; 3],
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        sector_face: SectorFace,
        vertex_vec: &mut Vec<SectorVertex>,
        index_vec: &mut Vec<u32>,
    ) {
        let sector_radius = SECTOR_RADIUS_IN_CELLS as i32;

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

        let normal: [f32; 3] = *Direction::to_vec3(&sector_face.direction).as_array();

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

        let (uv0, uv1, uv2, uv3) = match sector_face.direction {
            Direction::North => (uv1, uv0, uv3, uv2),
            Direction::West => (uv1, uv0, uv3, uv2),
            Direction::South => (uv0, uv1, uv2, uv3),
            Direction::East => (uv0, uv1, uv2, uv3),
            Direction::Up => (uv0, uv1, uv2, uv3),
            Direction::Down => (uv3, uv2, uv1, uv0),
        };

        let base_index = vertex_vec.len() as u32;

        let layer = tile_atlas::get_tile_layer(sector_face.block_kind, &sector_face.direction);

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

        let use_canonical = match sector_face.direction {
            Direction::North => false,
            Direction::West => false,
            Direction::South => true,
            Direction::East => true,
            Direction::Up => true,
            Direction::Down => false,
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
        slice_index as f32 - sector_radius as f32 - CELL_RADIUS_IN_METERS
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
