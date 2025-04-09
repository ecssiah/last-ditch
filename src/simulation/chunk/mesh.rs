use crate::simulation::{
    block::{self, Direction},
    BLOCK_RADIUS,
};
use nalgebra::Point3;
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub faces: Vec<block::Face>,
}

impl Mesh {
    pub fn vertices_and_indices(&self) -> (Vec<Point3<f32>>, Vec<[u32; 3]>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for face in self.faces.iter() {
            let face_vertices = face.vertices();

            let start_index = vertices.len() as u32;
            for vertex in &face_vertices {
                vertices.push(Point3::new(vertex.x, vertex.y, vertex.z));
            }

            indices.push([start_index, start_index + 1, start_index + 2]);
            indices.push([start_index, start_index + 2, start_index + 3]);
        }

        (vertices, indices)
    }

    pub fn optimized_vertices_and_indices(&self) -> (Vec<Point3<f32>>, Vec<[u32; 3]>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for direction in block::Direction::faces() {
            let mut face_positions: HashSet<(i32, i32, i32)> = HashSet::new();

            let direction_faces = self.faces.iter().filter(|face| face.direction == direction);

            for face in direction_faces {
                face_positions.insert((face.position.x, face.position.y, face.position.z));
            }

            let mut grid = face_positions.clone();

            while let Some(&(x, y, z)) = grid.iter().next() {
                let mut width = 1;
                let mut height = 1;

                match direction {
                    Direction::ZP | Direction::ZN => {
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

                        let start_index = vertices.len() as u32;

                        let zf = z as f32
                            + match direction {
                                Direction::ZP => BLOCK_RADIUS,
                                Direction::ZN => -BLOCK_RADIUS,
                                _ => 0.0,
                            };

                        let x0 = x as f32 - BLOCK_RADIUS;
                        let y0 = y as f32 - BLOCK_RADIUS;
                        let x1 = x0 + width as f32;
                        let y1 = y0 + height as f32;

                        vertices.push(Point3::new(x0, y0, zf));
                        vertices.push(Point3::new(x1, y0, zf));
                        vertices.push(Point3::new(x1, y1, zf));
                        vertices.push(Point3::new(x0, y1, zf));

                        indices.push([start_index, start_index + 1, start_index + 2]);
                        indices.push([start_index, start_index + 2, start_index + 3]);
                    }
                    Direction::XP | Direction::XN => {
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

                        let start_index = vertices.len() as u32;

                        let xf = x as f32
                            + match direction {
                                Direction::XP => BLOCK_RADIUS,
                                Direction::XN => -BLOCK_RADIUS,
                                _ => 0.0,
                            };

                        let y0 = y as f32 - BLOCK_RADIUS;
                        let z0 = z as f32 - BLOCK_RADIUS;
                        let y1 = y0 + width as f32;
                        let z1 = z0 + height as f32;

                        vertices.push(Point3::new(xf, y0, z0));
                        vertices.push(Point3::new(xf, y1, z0));
                        vertices.push(Point3::new(xf, y1, z1));
                        vertices.push(Point3::new(xf, y0, z1));

                        indices.push([start_index, start_index + 1, start_index + 2]);
                        indices.push([start_index, start_index + 2, start_index + 3]);
                    }
                    Direction::YP | Direction::YN => {
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

                        let start_index = vertices.len() as u32;

                        let yf = y as f32
                            + match direction {
                                Direction::YP => BLOCK_RADIUS,
                                Direction::YN => -BLOCK_RADIUS,
                                _ => 0.0,
                            };

                        let x0 = x as f32 - BLOCK_RADIUS;
                        let z0 = z as f32 - BLOCK_RADIUS;
                        let x1 = x0 + width as f32;
                        let z1 = z0 + height as f32;

                        vertices.push(Point3::new(x0, yf, z0));
                        vertices.push(Point3::new(x1, yf, z0));
                        vertices.push(Point3::new(x1, yf, z1));
                        vertices.push(Point3::new(x0, yf, z1));

                        indices.push([start_index, start_index + 1, start_index + 2]);
                        indices.push([start_index, start_index + 2, start_index + 3]);
                    }
                    _ => {}
                }
            }
        }

        (vertices, indices)
    }
}
