use crate::{
    interface::{gpu::gpu_mesh::GpuMesh, mesh::block_vertex::BlockVertex, world_render::block_render_info::BlockRenderInfo},
    simulation::{
        observation::{face_mask, view::SectorView},
        state::world::{block, grid::Grid, sector},
    },
};
use ultraviolet::IVec3;

pub struct SectorMesh {
    pub sector_id: sector::ID,
    pub version: u64,
    pub vertex_vec: Vec<BlockVertex>,
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

    fn merge_geometry(sector_view: &SectorView, grid: &Grid, mask_vec: Vec<Vec<Vec<i16>>>) -> Self {
        let mut vertex_vec = Vec::new();
        let mut index_vec = Vec::new();

        for dimension in 0..3 {
            let slice_count = mask_vec[dimension].len();

            for slice_index in 0..slice_count {
                let slice = &mask_vec[dimension][slice_index];

                Self::merge_slice(
                    dimension,
                    slice_index,
                    slice,
                    grid,
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
        dimension: usize,
        slice_index: usize,
        slice: &[i16],
        grid: &Grid,
        vertex_vec: &mut Vec<BlockVertex>,
        index_vec: &mut Vec<u32>,
    ) {
        let sector_size_in_cells = grid.sector_size_in_cells as usize;

        let mut consumed = vec![false; slice.len()];

        for y in 0..sector_size_in_cells {
            for x in 0..sector_size_in_cells {
                let mask_index = y * sector_size_in_cells + x;
                let mask_value = slice[mask_index];

                if mask_value == 0 || consumed[mask_index] {
                    continue;
                }

                let mut x_max = x + 1;

                while x_max < sector_size_in_cells {
                    let test_mask_index = y * sector_size_in_cells + x_max;
                    let test_mask_value = slice[test_mask_index];

                    if test_mask_value != mask_value || consumed[test_mask_index] {
                        break;
                    }

                    x_max += 1;
                }

                let mut y_max = y + 1;

                'outer: while y_max < sector_size_in_cells {
                    for xx in x..x_max {
                        let test_mask_index = y_max * sector_size_in_cells + xx;
                        let test_mask_value = slice[test_mask_index];

                        if test_mask_value != mask_value || consumed[test_mask_index] {
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
                    dimension,
                    slice_index,
                    x,
                    y,
                    x_max,
                    y_max,
                    mask_value,
                    grid,
                    vertex_vec,
                    index_vec,
                );
            }
        }
    }

    pub fn emit_triangles(
        dimension: usize,
        slice_index: usize,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        mask_value: i16,
        grid: &Grid,
        vertex_vec: &mut Vec<BlockVertex>,
        index_vec: &mut Vec<u32>,
    ) {
        let sector_radius = grid.sector_radius_in_cells as i32;

        // Signed kind: sign encodes direction, magnitude encodes block kind.
        let kind = mask_value.abs() as u16;
        let is_pos_axis_face = mask_value < 0; // by our convention: < 0 = +axis, > 0 = -axis

        // World coordinate of the slice plane along the primary axis.
        let plane_coord = Self::line_coord(slice_index, sector_radius);

        // Compute world-space bounds along the two in-plane axes.
        // x0..x1, y0..y1 are *cell indices* in the slice plane.
        // We convert them to edge coordinates using line_coord().
        //
        // For each dimension, x/y in the plane map to different world axes:
        //  - dim 0 (X slices): plane axes = (Y=row, Z=col)
        //  - dim 1 (Y slices): plane axes = (Z=row, X=col)
        //  - dim 2 (Z slices): plane axes = (Y=row, X=col)

        let (x_min, x_max, y_min, y_max, z_min, z_max) = match dimension {
            // X slices: (row=y -> world Y, col=x -> world Z), X is constant
            0 => {
                let row0 = y0;
                let row1 = y1;
                let col0 = x0;
                let col1 = x1;

                let y_min = Self::line_coord(row0, sector_radius);
                let y_max = Self::line_coord(row1, sector_radius);
                let z_min = Self::line_coord(col0, sector_radius);
                let z_max = Self::line_coord(col1, sector_radius);

                (plane_coord, plane_coord, y_min, y_max, z_min, z_max)
            }

            // Y slices: (row=y -> world Z, col=x -> world X), Y is constant
            1 => {
                let row0 = y0;
                let row1 = y1;
                let col0 = x0;
                let col1 = x1;

                let z_min = Self::line_coord(row0, sector_radius);
                let z_max = Self::line_coord(row1, sector_radius);
                let x_min = Self::line_coord(col0, sector_radius);
                let x_max = Self::line_coord(col1, sector_radius);

                (x_min, x_max, plane_coord, plane_coord, z_min, z_max)
            }

            // Z slices: (row=y -> world Y, col=x -> world X), Z is constant
            2 => {
                let row0 = y0;
                let row1 = y1;
                let col0 = x0;
                let col1 = x1;

                let y_min = Self::line_coord(row0, sector_radius);
                let y_max = Self::line_coord(row1, sector_radius);
                let x_min = Self::line_coord(col0, sector_radius);
                let x_max = Self::line_coord(col1, sector_radius);

                (x_min, x_max, y_min, y_max, plane_coord, plane_coord)
            }

            _ => unreachable!("dimension must be 0, 1 or 2"),
        };

        // Canonical quad corners in world space.
        //
        // We build them in a consistent order around the rectangle:
        //   v0 = (min, min)
        //   v1 = (max, min)
        //   v2 = (max, max)
        //   v3 = (min, max)
        //
        // Which axes are "min/max" depends on the dimension (see mapping above).
        let (p0, p1, p2, p3) = (
            [x_min, y_min, z_min],
            [x_max, y_min, z_min],
            [x_max, y_max, z_max],
            [x_min, y_max, z_max],
        );

        // Normal from dimension + sign.
        let normal: [f32; 3] = match (dimension, is_pos_axis_face) {
            (0, true) => [ 1.0,  0.0,  0.0], // +X
            (0, false) => [-1.0,  0.0,  0.0], // -X

            (1, true) => [ 0.0,  1.0,  0.0], // +Y
            (1, false) => [ 0.0, -1.0,  0.0], // -Y

            (2, true) => [ 0.0,  0.0,  1.0], // +Z
            (2, false) => [ 0.0,  0.0, -1.0], // -Z

            _ => unreachable!(),
        };

        // For now: simple [0..1] UVs (one tile per merged quad).
        // Because you're using a texture array, the actual layer comes from block_kind in
        // some other attribute/buffer or per-draw uniform. This vertex only cares about UV within the tile.
        let uv0 = [0.0, 0.0];
        let uv1 = [1.0, 0.0];
        let uv2 = [1.0, 1.0];
        let uv3 = [0.0, 1.0];

        let base_index = vertex_vec.len() as u32;

        let tile_coordinates = BlockRenderInfo::get_tile_coordinates(mask_value as u16 as block::Kind)[face_index];
        let layer_index = BlockRenderInfo::tile_to_layer(tile_coordinates);

        vertex_vec.push(BlockVertex { position: p0, normal, uv: uv0, layer: layer_index });
        vertex_vec.push(BlockVertex { position: p1, normal, uv: uv1, layer: layer_index });
        vertex_vec.push(BlockVertex { position: p2, normal, uv: uv2, layer: layer_index });
        vertex_vec.push(BlockVertex { position: p3, normal, uv: uv3, layer: layer_index });

        // Triangle winding:
        //
        // Our canonical vertex order (p0, p1, p2, p3) corresponds to:
        //  - a specific orientation in each plane that yields a particular
        //    normal direction from the cross product of (p1 - p0) x (p2 - p0).
        //
        // Rather than overthink it, we just define:
        //
        //  - "canonical order" (0,1,2, 0,2,3)
        //  - "flipped order"   (0,2,1, 0,3,2)
        //
        // and choose which one to use per dimension/sign so that
        // backface culling lines up with the normal we've assigned.
        //
        let use_canonical = match (dimension, is_pos_axis_face) {
            // dim 0: canonical gives -X; so use canonical for -X, flipped for +X
            (0, false) => true,   // -X
            (0, true)  => false,  // +X

            // dim 1: canonical gives -Y; so use canonical for -Y, flipped for +Y
            (1, false) => true,   // -Y
            (1, true)  => false,  // +Y

            // dim 2: canonical gives +Z; so use canonical for +Z, flipped for -Z
            (2, true)  => true,   // +Z
            (2, false) => false,  // -Z

            _ => unreachable!(),
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

        // `kind` (block_kind) isn't used yet here — you'll likely want to:
        //  - either group quads by kind/material when building the GPU mesh, or
        //  - add a separate vertex attribute/buffer for texture layer index.
    }

    fn line_coord(k: usize, sector_radius: i32) -> f32 {
        // k in [0..=2R+1] → world edge position
        k as f32 - sector_radius as f32 - 0.5
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
