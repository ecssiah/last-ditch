use crate::{
    interface::{
        asset_manager::{AssetManager, block_texture_key::BlockTextureKey},
        gpu::gpu_mesh::GpuMesh, renderer::sector_renderer::{sector_face::SectorFace, sector_vertex::SectorVertexData},
    },
    simulation::{
        constants::*,
        state::world::{
            block::{block_shape::BlockShape, block_state::BlockState},
            grid::{self, Direction, axis::Axis, direction_set::DirectionSet},
            sector::{Sector, sector_index::SectorIndex},
        },
        supervisor::viewer::view::SectorView,
    },
};
use ultraviolet::IVec3;

pub struct SectorModel {
    pub sector_index: SectorIndex,
    pub version: u64,
    pub vertex_vec: Vec<SectorVertexData>,
    pub index_vec: Vec<u32>,
}

impl SectorModel {
    pub fn from_sector_view(sector_view: &SectorView, asset_manager: &AssetManager) -> Self {
        let sector_model = Self::lysenko_optimization(sector_view, asset_manager);

        sector_model
    }

    fn lysenko_optimization(sector_view: &SectorView, asset_manager: &AssetManager) -> Self {
        let sector_face_vec = Self::collect_sector_geometry(sector_view, asset_manager);

        let (sector_vertex_vec, sector_index_vec) = Self::get_sector_face_geometry(sector_face_vec);

        let sector_model = SectorModel {
            sector_index: sector_view.sector_index,
            version: sector_view.version,
            vertex_vec: sector_vertex_vec,
            index_vec: sector_index_vec,
        };

        sector_model
    }

    fn collect_sector_geometry(
        sector_view: &SectorView,
        asset_manager: &AssetManager,
    ) -> Vec<Vec<Vec<Option<SectorFace>>>> {
        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;
        let sector_area_in_cells = SECTOR_AREA_IN_CELLS as i32;

        let slice_count = (sector_size_in_cells + 1) as usize;

        let mut sector_face_vec = vec![
            vec![vec![None; sector_area_in_cells as usize]; slice_count],
            vec![vec![None; sector_area_in_cells as usize]; slice_count],
            vec![vec![None; sector_area_in_cells as usize]; slice_count],
        ];

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell_coordinate = IVec3::new(x, y, z);
                    let cell_index = grid::cell_coordinate_to_cell_index(cell_coordinate);

                    let block_grid_position =
                        grid::indices_to_grid_position(sector_view.sector_index, cell_index);

                    let block_world_position =
                        grid::grid_position_to_world_position(block_grid_position);

                    if let Some(block) = Sector::get_block(cell_index, &sector_view.block_vec) {
                        if block.block_shape == BlockShape::Block {
                            let BlockState::Block(ref block_data) = block.block_state else {
                                panic!("block should have block data")
                            };

                            let layer_index = AssetManager::get_block_layer_index(
                                &BlockTextureKey::from_block_kind(&block.block_kind),
                                asset_manager,
                            );

                            if DirectionSet::has(&Direction::North, &block_data.exposure_set) {
                                let slice_index = (y + sector_radius_in_cells + 1) as usize;

                                let local_x = (x + sector_radius_in_cells) as usize;
                                let local_z = (z + sector_radius_in_cells) as usize;

                                let mask_index =
                                    local_z * (sector_size_in_cells as usize) + local_x;

                                let direction = Direction::North;
                                let normal = Direction::to_vec3(&direction);
                                let world_position = block_world_position + normal * 0.5;

                                let sector_face =
                                    SectorFace::new(world_position, direction, layer_index);

                                sector_face_vec[Axis::Y as usize][slice_index][mask_index] =
                                    Some(sector_face);
                            }

                            if DirectionSet::has(&Direction::West, &block_data.exposure_set) {
                                let slice_index = (x + sector_radius_in_cells) as usize;

                                let local_y = (y + sector_radius_in_cells) as usize;
                                let local_z = (z + sector_radius_in_cells) as usize;

                                let mask_index =
                                    local_y * (sector_size_in_cells as usize) + local_z;

                                let direction = Direction::West;
                                let normal = Direction::to_vec3(&direction);
                                let world_position = block_world_position + normal * 0.5;

                                let sector_face =
                                    SectorFace::new(world_position, direction, layer_index);

                                sector_face_vec[Axis::X as usize][slice_index][mask_index] =
                                    Some(sector_face);
                            }

                            if DirectionSet::has(&Direction::South, &block_data.exposure_set) {
                                let slice_index = (y + sector_radius_in_cells) as usize;

                                let local_x = (x + sector_radius_in_cells) as usize;
                                let local_z = (z + sector_radius_in_cells) as usize;

                                let mask_index =
                                    local_z * (sector_size_in_cells as usize) + local_x;

                                let direction = Direction::South;
                                let normal = Direction::to_vec3(&direction);
                                let world_position = block_world_position + normal * 0.5;

                                let sector_face =
                                    SectorFace::new(world_position, direction, layer_index);

                                sector_face_vec[Axis::Y as usize][slice_index][mask_index] =
                                    Some(sector_face);
                            }

                            if DirectionSet::has(&Direction::East, &block_data.exposure_set) {
                                let slice_index = (x + sector_radius_in_cells + 1) as usize;

                                let local_y = (y + sector_radius_in_cells) as usize;
                                let local_z = (z + sector_radius_in_cells) as usize;

                                let mask_index =
                                    local_y * (sector_size_in_cells as usize) + local_z;

                                let direction = Direction::East;
                                let normal = Direction::to_vec3(&direction);
                                let world_position = block_world_position + normal * 0.5;

                                let sector_face =
                                    SectorFace::new(world_position, direction, layer_index);

                                sector_face_vec[Axis::X as usize][slice_index][mask_index] =
                                    Some(sector_face);
                            }

                            if DirectionSet::has(&Direction::Up, &block_data.exposure_set) {
                                let slice_index = (z + sector_radius_in_cells + 1) as usize;

                                let local_x = (x + sector_radius_in_cells) as usize;
                                let local_y = (y + sector_radius_in_cells) as usize;

                                let mask_index =
                                    local_y * (sector_size_in_cells as usize) + local_x;

                                let direction = Direction::Up;
                                let normal = Direction::to_vec3(&direction);
                                let world_position = block_world_position + normal * 0.5;

                                let sector_face =
                                    SectorFace::new(world_position, direction, layer_index);

                                sector_face_vec[Axis::Z as usize][slice_index][mask_index] =
                                    Some(sector_face);
                            }

                            if DirectionSet::has(&Direction::Down, &block_data.exposure_set) {
                                let slice_index = (z + sector_radius_in_cells) as usize;

                                let local_x = (x + sector_radius_in_cells) as usize;
                                let local_y = (y + sector_radius_in_cells) as usize;

                                let mask_index =
                                    local_y * (sector_size_in_cells as usize) + local_x;

                                let direction = Direction::Down;
                                let normal = Direction::to_vec3(&direction);
                                let world_position = block_world_position + normal * 0.5;

                                let sector_face =
                                    SectorFace::new(world_position, direction, layer_index);

                                sector_face_vec[Axis::Z as usize][slice_index][mask_index] =
                                    Some(sector_face);
                            }
                        }
                    }
                }
            }
        }

        sector_face_vec
    }

    fn get_sector_face_geometry(
        sector_face_vec: Vec<Vec<Vec<Option<SectorFace>>>>,
    ) -> (Vec<SectorVertexData>, Vec<u32>) {
        let mut vertex_vec = Vec::new();
        let mut index_vec = Vec::new();

        for axis in Axis::ALL {
            let axis_vec = &sector_face_vec[axis as usize];
            let slice_count = axis_vec.len();

            for slice_index in 0..slice_count {
                let sector_face_slice = &axis_vec[slice_index];

                Self::merge_slice(
                    axis,
                    slice_index,
                    sector_face_slice,
                    &mut vertex_vec,
                    &mut index_vec,
                );
            }
        }

        (vertex_vec, index_vec)
    }

    fn merge_slice(
        axis: Axis,
        slice_index: usize,
        sector_face_slice: &[Option<SectorFace>],
        vertex_vec: &mut Vec<SectorVertexData>,
        index_vec: &mut Vec<u32>,
    ) {
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as usize;

        let mut visited = vec![false; sector_face_slice.len()];

        for y in 0..sector_size_in_cells {
            for x in 0..sector_size_in_cells {
                let mask_index = y * sector_size_in_cells + x;

                let sector_face = match sector_face_slice[mask_index].clone() {
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

                    if test_face != sector_face || visited[test_mask_index] {
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

                        if test_face != sector_face || visited[test_mask_index] {
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
                    x,
                    y,
                    x_max,
                    y_max,
                    sector_face,
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
        sector_face: SectorFace,
        vertex_vec: &mut Vec<SectorVertexData>,
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

        let normal_array: [f32; 3] = Direction::to_array(&sector_face.direction);

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

        let initial_index = vertex_vec.len() as u32;

        vertex_vec.push(SectorVertexData {
            position_array: vertex_position0,
            normal_array,
            uv_array: uv0,
            layer_index: sector_face.layer_index.into(),
        });

        vertex_vec.push(SectorVertexData {
            position_array: vertex_position1,
            normal_array,
            uv_array: uv1,
            layer_index: sector_face.layer_index.into(),
        });

        vertex_vec.push(SectorVertexData {
            position_array: vertex_position2,
            normal_array,
            uv_array: uv2,
            layer_index: sector_face.layer_index.into(),
        });

        vertex_vec.push(SectorVertexData {
            position_array: vertex_position3,
            normal_array,
            uv_array: uv3,
            layer_index: sector_face.layer_index.into(),
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
            index_vec.push(initial_index + 0);
            index_vec.push(initial_index + 1);
            index_vec.push(initial_index + 2);

            index_vec.push(initial_index + 0);
            index_vec.push(initial_index + 2);
            index_vec.push(initial_index + 3);
        } else {
            index_vec.push(initial_index + 0);
            index_vec.push(initial_index + 2);
            index_vec.push(initial_index + 1);

            index_vec.push(initial_index + 0);
            index_vec.push(initial_index + 3);
            index_vec.push(initial_index + 2);
        }
    }

    fn get_axis_coordinate(slice_index: usize, sector_radius: i32) -> f32 {
        slice_index as f32 - sector_radius as f32 - CELL_RADIUS_IN_METERS
    }

    pub fn to_gpu_mesh(sector_model: &Self, device: &wgpu::Device) -> GpuMesh {
        assert!(
            !sector_model.vertex_vec.is_empty(),
            "Vertex buffer is empty!"
        );

        assert!(!sector_model.index_vec.is_empty(), "Index buffer is empty!");

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&sector_model.vertex_vec),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&sector_model.index_vec),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let index_count = sector_model.index_vec.len() as u32;

        let material_id = 0;

        GpuMesh {
            version: sector_model.version,
            vertex_buffer,
            index_buffer,
            index_count,
            material_id,
        }
    }
}
