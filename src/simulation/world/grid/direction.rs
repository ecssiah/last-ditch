use glam::IVec3;
use serde::{Deserialize, Serialize};

use crate::simulation::{WORLD_CARDINAL_COST, WORLD_CORNER_COST, WORLD_EDGE_COST};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Direction {
    XnYnZn, XoYnZn, XpYnZn,
    XnYoZn, XoYoZn, XpYoZn,
    XnYpZn, XoYpZn, XpYpZn,

    XnYnZo, XoYnZo, XpYnZo,
    XnYoZo, XoYoZo, XpYoZo,
    XnYpZo, XoYpZo, XpYpZo,

    XnYnZp, XoYnZp, XpYnZp,
    XnYoZp, XoYoZp, XpYoZp,
    XnYpZp, XoYpZp, XpYpZp,
}

impl Direction {
    #[rustfmt::skip]
    const ALL: [Direction; 27] = [
        Direction::XnYnZn,
        Direction::XoYnZn,
        Direction::XpYnZn,
        Direction::XnYoZn,
        Direction::XoYoZn,
        Direction::XpYoZn,
        Direction::XnYpZn,
        Direction::XoYpZn,
        Direction::XpYpZn,
        Direction::XnYnZo,
        Direction::XoYnZo,
        Direction::XpYnZo,
        Direction::XnYoZo,
        Direction::XoYoZo,
        Direction::XpYoZo,
        Direction::XnYpZo,
        Direction::XoYpZo,
        Direction::XpYpZo,
        Direction::XnYnZp,
        Direction::XoYnZp,
        Direction::XpYnZp,
        Direction::XnYoZp,
        Direction::XoYoZp,
        Direction::XpYoZp,
        Direction::XnYpZp,
        Direction::XoYpZp,
        Direction::XpYpZp,
    ];

    #[rustfmt::skip]
    const NEIGHBORS: [Direction; 26] = [
        Direction::XnYnZn,
        Direction::XoYnZn,
        Direction::XpYnZn,
        Direction::XnYoZn,
        Direction::XoYoZn,
        Direction::XpYoZn,
        Direction::XnYpZn,
        Direction::XoYpZn,
        Direction::XpYpZn,
        Direction::XnYnZo,
        Direction::XoYnZo,
        Direction::XpYnZo,
        Direction::XnYoZo,
        Direction::XpYoZo,
        Direction::XnYpZo,
        Direction::XoYpZo,
        Direction::XpYpZo,
        Direction::XnYnZp,
        Direction::XoYnZp,
        Direction::XpYnZp,
        Direction::XnYoZp,
        Direction::XoYoZp,
        Direction::XpYoZp,
        Direction::XnYpZp,
        Direction::XoYpZp,
        Direction::XpYpZp,
    ];

    #[rustfmt::skip]
    const AXES: [Direction; 3] = [
        Direction::XpYoZo,
        Direction::XoYpZo,
        Direction::XoYoZp,
    ];

    #[rustfmt::skip]
    const CARDINAL: [Direction; 4] = [
        Direction::XpYoZo,
        Direction::XnYoZo,
        Direction::XoYoZp,
        Direction::XoYoZn,
    ];

    #[rustfmt::skip]
    const FACES: [Direction; 6] = [
        Direction::XpYoZo,
        Direction::XnYoZo,
        Direction::XoYpZo,
        Direction::XoYnZo,
        Direction::XoYoZp,
        Direction::XoYoZn,
    ];

    #[rustfmt::skip]
    const EDGES: [Direction; 12] = [
        Direction::XoYnZn,
        Direction::XnYoZn,
        Direction::XpYoZn,
        Direction::XoYpZn,
        Direction::XnYnZo,
        Direction::XpYnZo,
        Direction::XnYpZo,
        Direction::XpYpZo,
        Direction::XoYnZp,
        Direction::XnYoZp,
        Direction::XpYoZp,
        Direction::XoYpZp,
    ];

    #[rustfmt::skip]
    const CORNERS: [Direction; 8] = [
        Direction::XnYnZn,
        Direction::XpYnZn,
        Direction::XnYpZn,
        Direction::XpYpZn,
        Direction::XnYnZp,
        Direction::XpYnZp,
        Direction::XnYpZp,
        Direction::XpYpZp,
    ];

    pub fn all() -> [Direction; 27] {
        Self::ALL
    }

    pub fn neighbors() -> [Direction; 26] {
        Self::NEIGHBORS
    }

    pub fn axes() -> [Direction; 3] {
        Self::AXES
    }

    pub fn cardinal() -> [Direction; 4] {
        Self::CARDINAL
    }

    pub fn faces() -> [Direction; 6] {
        Self::FACES
    }

    pub fn edges() -> [Direction; 12] {
        Self::EDGES
    }

    pub fn corners() -> [Direction; 8] {
        Self::CORNERS
    }

    pub fn offset(&self) -> IVec3 {
        match self {
            Direction::XnYnZn => IVec3::new(-1, -1, -1),
            Direction::XoYnZn => IVec3::new(0, -1, -1),
            Direction::XpYnZn => IVec3::new(1, -1, -1),
            Direction::XnYoZn => IVec3::new(-1, 0, -1),
            Direction::XoYoZn => IVec3::new(0, 0, -1),
            Direction::XpYoZn => IVec3::new(1, 0, -1),
            Direction::XnYpZn => IVec3::new(-1, 1, -1),
            Direction::XoYpZn => IVec3::new(0, 1, -1),
            Direction::XpYpZn => IVec3::new(1, 1, -1),
            Direction::XnYnZo => IVec3::new(-1, -1, 0),
            Direction::XoYnZo => IVec3::new(0, -1, 0),
            Direction::XpYnZo => IVec3::new(1, -1, 0),
            Direction::XnYoZo => IVec3::new(-1, 0, 0),
            Direction::XoYoZo => IVec3::new(0, 0, 0),
            Direction::XpYoZo => IVec3::new(1, 0, 0),
            Direction::XnYpZo => IVec3::new(-1, 1, 0),
            Direction::XoYpZo => IVec3::new(0, 1, 0),
            Direction::XpYpZo => IVec3::new(1, 1, 0),
            Direction::XnYnZp => IVec3::new(-1, -1, 1),
            Direction::XoYnZp => IVec3::new(0, -1, 1),
            Direction::XpYnZp => IVec3::new(1, -1, 1),
            Direction::XnYoZp => IVec3::new(-1, 0, 1),
            Direction::XoYoZp => IVec3::new(0, 0, 1),
            Direction::XpYoZp => IVec3::new(1, 0, 1),
            Direction::XnYpZp => IVec3::new(-1, 1, 1),
            Direction::XoYpZp => IVec3::new(0, 1, 1),
            Direction::XpYpZp => IVec3::new(1, 1, 1),
        }
    }

    pub fn cost(&self) -> f32 {
        match self {
            Direction::XnYnZn => WORLD_CORNER_COST,
            Direction::XoYnZn => WORLD_EDGE_COST,
            Direction::XpYnZn => WORLD_CORNER_COST,
            Direction::XnYoZn => WORLD_EDGE_COST,
            Direction::XoYoZn => WORLD_CARDINAL_COST,
            Direction::XpYoZn => WORLD_EDGE_COST,
            Direction::XnYpZn => WORLD_CORNER_COST,
            Direction::XoYpZn => WORLD_EDGE_COST,
            Direction::XpYpZn => WORLD_CORNER_COST,
            Direction::XnYnZo => WORLD_EDGE_COST,
            Direction::XoYnZo => WORLD_CARDINAL_COST,
            Direction::XpYnZo => WORLD_EDGE_COST,
            Direction::XnYoZo => WORLD_CARDINAL_COST,
            Direction::XoYoZo => WORLD_CARDINAL_COST,
            Direction::XpYoZo => WORLD_CARDINAL_COST,
            Direction::XnYpZo => WORLD_EDGE_COST,
            Direction::XoYpZo => WORLD_CARDINAL_COST,
            Direction::XpYpZo => WORLD_EDGE_COST,
            Direction::XnYnZp => WORLD_CORNER_COST,
            Direction::XoYnZp => WORLD_EDGE_COST,
            Direction::XpYnZp => WORLD_CORNER_COST,
            Direction::XnYoZp => WORLD_EDGE_COST,
            Direction::XoYoZp => WORLD_CARDINAL_COST,
            Direction::XpYoZp => WORLD_EDGE_COST,
            Direction::XnYpZp => WORLD_CORNER_COST,
            Direction::XoYpZp => WORLD_EDGE_COST,
            Direction::XpYpZp => WORLD_CORNER_COST,
        }
    }

    pub fn neighbor_offsets() -> [IVec3; 26] {
        Self::NEIGHBORS.map(|neighbor| neighbor.offset())
    }

    pub fn face_offsets() -> [IVec3; 6] {
        Self::FACES.map(|face| face.offset())
    }

    pub fn edge_offsets() -> [IVec3; 12] {
        Self::EDGES.map(|edge| edge.offset())
    }

    pub fn corner_offsets() -> [IVec3; 8] {
        Self::CORNERS.map(|corner| corner.offset())
    }
}
