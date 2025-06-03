use glam::IVec3;
use serde::{Deserialize, Serialize};

use crate::simulation::{WORLD_CORNER_COST, WORLD_EDGE_COST, WORLD_FACE_COST};

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
    const NEIGHBOR_LIST: [Direction; 26] = [
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
    const AXIS_LIST: [Direction; 3] = [
        Direction::XpYoZo,
        Direction::XoYpZo,
        Direction::XoYoZp,
    ];

    #[rustfmt::skip]
    const FACE_LIST: [Direction; 6] = [
        Direction::XpYoZo,
        Direction::XnYoZo,
        Direction::XoYpZo,
        Direction::XoYnZo,
        Direction::XoYoZp,
        Direction::XoYoZn,
    ];

    #[rustfmt::skip]
    const EDGE_LIST: [Direction; 12] = [
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
    const CORNER_LIST: [Direction; 8] = [
        Direction::XnYnZn,
        Direction::XpYnZn,
        Direction::XnYpZn,
        Direction::XpYpZn,
        Direction::XnYnZp,
        Direction::XpYnZp,
        Direction::XnYpZp,
        Direction::XpYpZp,
    ];

    #[rustfmt::skip]
    const DIAGONAL_LIST: [Direction; 12] = [
        Direction::XnYoZn,
        Direction::XpYoZn,
        Direction::XnYoZp,
        Direction::XpYoZp,
        Direction::XnYnZn,
        Direction::XpYnZn,
        Direction::XnYpZn,
        Direction::XpYpZn,
        Direction::XnYnZp,
        Direction::XpYnZp,
        Direction::XnYpZp,
        Direction::XpYpZp,
    ];

    #[rustfmt::skip]
    const TRAVERSABLE_LIST: [Direction; 12] = [
        Direction::XnYpZo,
        Direction::XpYpZo,
        Direction::XoYpZp,
        Direction::XoYpZn,
        Direction::XnYoZo,
        Direction::XpYoZo,
        Direction::XoYoZp,
        Direction::XoYoZn,
        Direction::XnYnZo,
        Direction::XpYnZo,
        Direction::XoYnZp,
        Direction::XoYnZn,
    ];

    pub fn is_face(&self) -> bool {
        let offset = self.offset();

        [offset.x, offset.y, offset.z]
            .iter()
            .filter(|&&v| v != 0)
            .count()
            == 1
    }

    pub fn is_edge(&self) -> bool {
        let offset = self.offset();

        [offset.x, offset.y, offset.z]
            .iter()
            .filter(|&&v| v != 0)
            .count()
            == 2
    }

    pub fn is_corner(&self) -> bool {
        let offset = self.offset();

        [offset.x, offset.y, offset.z]
            .iter()
            .filter(|&&v| v != 0)
            .count()
            == 3
    }

    pub fn is_diagonal(&self) -> bool {
        let offset = self.offset();

        offset.x != 0 && offset.z != 0
    }

    pub fn all() -> [Direction; 27] {
        Self::ALL
    }

    pub fn neighbor_list() -> [Direction; 26] {
        Self::NEIGHBOR_LIST
    }

    pub fn axis_list() -> [Direction; 3] {
        Self::AXIS_LIST
    }

    pub fn face_list() -> [Direction; 6] {
        Self::FACE_LIST
    }

    pub fn edge_list() -> [Direction; 12] {
        Self::EDGE_LIST
    }

    pub fn corner_list() -> [Direction; 8] {
        Self::CORNER_LIST
    }

    pub fn diagonal_list() -> [Direction; 12] {
        Self::DIAGONAL_LIST
    }

    pub fn traversable_list() -> [Direction; 12] {
        Self::TRAVERSABLE_LIST
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
            Direction::XoYoZn => WORLD_FACE_COST,
            Direction::XpYoZn => WORLD_EDGE_COST,
            Direction::XnYpZn => WORLD_CORNER_COST,
            Direction::XoYpZn => WORLD_EDGE_COST,
            Direction::XpYpZn => WORLD_CORNER_COST,
            Direction::XnYnZo => WORLD_EDGE_COST,
            Direction::XoYnZo => WORLD_FACE_COST,
            Direction::XpYnZo => WORLD_EDGE_COST,
            Direction::XnYoZo => WORLD_FACE_COST,
            Direction::XoYoZo => WORLD_FACE_COST,
            Direction::XpYoZo => WORLD_FACE_COST,
            Direction::XnYpZo => WORLD_EDGE_COST,
            Direction::XoYpZo => WORLD_FACE_COST,
            Direction::XpYpZo => WORLD_EDGE_COST,
            Direction::XnYnZp => WORLD_CORNER_COST,
            Direction::XoYnZp => WORLD_EDGE_COST,
            Direction::XpYnZp => WORLD_CORNER_COST,
            Direction::XnYoZp => WORLD_EDGE_COST,
            Direction::XoYoZp => WORLD_FACE_COST,
            Direction::XpYoZp => WORLD_EDGE_COST,
            Direction::XnYpZp => WORLD_CORNER_COST,
            Direction::XoYpZp => WORLD_EDGE_COST,
            Direction::XpYpZp => WORLD_CORNER_COST,
        }
    }

    pub fn neighbor_offsets() -> [IVec3; 26] {
        Self::NEIGHBOR_LIST.map(|neighbor| neighbor.offset())
    }

    pub fn face_offsets() -> [IVec3; 6] {
        Self::FACE_LIST.map(|face| face.offset())
    }

    pub fn edge_offsets() -> [IVec3; 12] {
        Self::EDGE_LIST.map(|edge| edge.offset())
    }

    pub fn corner_offsets() -> [IVec3; 8] {
        Self::CORNER_LIST.map(|corner| corner.offset())
    }

    pub fn from_components(x: i32, y: i32, z: i32) -> Option<Direction> {
        Self::ALL
            .iter()
            .copied()
            .find(|direction| direction.offset() == IVec3::new(x, y, z))
    }
}
