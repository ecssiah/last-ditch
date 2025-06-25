use crate::simulation::{
    consts::*,
    state::world::{block, grid},
};
use nalgebra::Point3;
use std::collections::HashSet;

#[derive(Clone, Default, Debug)]
pub struct Geometry {
    pub face_list: Vec<block::Face>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            face_list: Vec::new(),
        }
    }

    pub fn vertices_and_indices(&self) -> (Vec<Point3<f32>>, Vec<[u32; 3]>) {
        let mut vertex_list = Vec::new();
        let mut index_list = Vec::new();

        for face in self.face_list.iter() {
            let face_vertex_list = face.vertices();
            let start_index = vertex_list.len() as u32;

            for vertex in &face_vertex_list {
                vertex_list.push(Point3::new(vertex.x, vertex.y, vertex.z));
            }

            index_list.push([start_index, start_index + 1, start_index + 2]);
            index_list.push([start_index + 2, start_index + 3, start_index]);
        }

        (vertex_list, index_list)
    }

    pub fn optimized_vertices_and_indices(&self) -> (Vec<Point3<f32>>, Vec<[u32; 3]>) {
        let mut vertex_list = Vec::new();
        let mut index_list = Vec::new();

        for direction in grid::Direction::face_list() {
            let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();

            let direction_faces = self
                .face_list
                .iter()
                .filter(|face| face.direction == direction);

            for face in direction_faces {
                grid.insert((face.position.x, face.position.y, face.position.z));
            }

            while let Some(&(x, y, z)) = grid.iter().next() {
                let mut width = 1;
                let mut height = 1;

                match direction {
                    grid::Direction::XoYoZp | grid::Direction::XoYoZn => {
                        while grid.contains(&(x + width, y, z)) {
                            width += 1;
                        }

                        'outer: loop {
                            for dx in 0..width {
                                if !grid.contains(&(x + dx, y + height, z)) {
                                    break 'outer;
                                }
                            }
                            height += 1;
                        }

                        for dy in 0..height {
                            for dx in 0..width {
                                grid.remove(&(x + dx, y + dy, z));
                            }
                        }

                        let start_index = vertex_list.len() as u32;

                        let zf = z as f32
                            + match direction {
                                grid::Direction::XoYoZp => BLOCK_RADIUS,
                                grid::Direction::XoYoZn => -BLOCK_RADIUS,
                                _ => 0.0,
                            };

                        let x0 = x as f32 - BLOCK_RADIUS;
                        let y0 = y as f32 - BLOCK_RADIUS;
                        let x1 = x0 + width as f32;
                        let y1 = y0 + height as f32;

                        vertex_list.push(Point3::new(x0, y0, zf));
                        vertex_list.push(Point3::new(x1, y0, zf));
                        vertex_list.push(Point3::new(x1, y1, zf));
                        vertex_list.push(Point3::new(x0, y1, zf));

                        index_list.push([start_index, start_index + 1, start_index + 2]);
                        index_list.push([start_index + 2, start_index + 3, start_index]);
                    }
                    grid::Direction::XpYoZo | grid::Direction::XnYoZo => {
                        while grid.contains(&(x, y + width, z)) {
                            width += 1;
                        }

                        'outer: loop {
                            for dy in 0..width {
                                if !grid.contains(&(x, y + dy, z + height)) {
                                    break 'outer;
                                }
                            }
                            height += 1;
                        }

                        for dz in 0..height {
                            for dy in 0..width {
                                grid.remove(&(x, y + dy, z + dz));
                            }
                        }

                        let start_index = vertex_list.len() as u32;

                        let xf = x as f32
                            + match direction {
                                grid::Direction::XpYoZo => BLOCK_RADIUS,
                                grid::Direction::XnYoZo => -BLOCK_RADIUS,
                                _ => 0.0,
                            };

                        let y0 = y as f32 - BLOCK_RADIUS;
                        let z0 = z as f32 - BLOCK_RADIUS;
                        let y1 = y0 + width as f32;
                        let z1 = z0 + height as f32;

                        vertex_list.push(Point3::new(xf, y0, z0));
                        vertex_list.push(Point3::new(xf, y1, z0));
                        vertex_list.push(Point3::new(xf, y1, z1));
                        vertex_list.push(Point3::new(xf, y0, z1));

                        index_list.push([start_index, start_index + 1, start_index + 2]);
                        index_list.push([start_index, start_index + 2, start_index + 3]);
                    }
                    grid::Direction::XoYpZo | grid::Direction::XoYnZo => {
                        while grid.contains(&(x + width, y, z)) {
                            width += 1;
                        }

                        'outer: loop {
                            for dx in 0..width {
                                if !grid.contains(&(x + dx, y, z + height)) {
                                    break 'outer;
                                }
                            }
                            height += 1;
                        }

                        for dz in 0..height {
                            for dx in 0..width {
                                grid.remove(&(x + dx, y, z + dz));
                            }
                        }

                        let start_index = vertex_list.len() as u32;

                        let yf = y as f32
                            + match direction {
                                grid::Direction::XoYpZo => BLOCK_RADIUS,
                                grid::Direction::XoYnZo => -BLOCK_RADIUS,
                                _ => 0.0,
                            };

                        let x0 = x as f32 - BLOCK_RADIUS;
                        let z0 = z as f32 - BLOCK_RADIUS;
                        let x1 = x0 + width as f32;
                        let z1 = z0 + height as f32;

                        vertex_list.push(Point3::new(x0, yf, z0));
                        vertex_list.push(Point3::new(x1, yf, z0));
                        vertex_list.push(Point3::new(x1, yf, z1));
                        vertex_list.push(Point3::new(x0, yf, z1));

                        index_list.push([start_index, start_index + 1, start_index + 2]);
                        index_list.push([start_index, start_index + 2, start_index + 3]);
                    }
                    _ => {}
                }
            }
        }

        (vertex_list, index_list)
    }
}
