use ultraviolet::IVec3;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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
    const ALL_ARRAY: [Direction; 27] = [
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
    const CARDINAL_ARRAY: [Direction; 4] = [
        Direction::XpYoZo,
        Direction::XnYoZo,
        Direction::XoYoZp,
        Direction::XoYoZn,
    ];

    #[rustfmt::skip]
    const NEIGHBOR_ARRAY: [Direction; 26] = [
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
    const AXIS_ARRAY: [Direction; 3] = [
        Direction::XpYoZo,
        Direction::XoYpZo,
        Direction::XoYoZp,
    ];

    #[rustfmt::skip]
    const FACE_ARRAY: [Direction; 6] = [
        Direction::XpYoZo,
        Direction::XnYoZo,
        Direction::XoYpZo,
        Direction::XoYnZo,
        Direction::XoYoZp,
        Direction::XoYoZn,
    ];

    #[rustfmt::skip]
    const EDGE_ARRAY: [Direction; 12] = [
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
    const CORNER_ARRAY: [Direction; 8] = [
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
    const DIAGONAL_ARRAY: [Direction; 12] = [
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
    const TRAVERSABLE_ARRAY: [Direction; 12] = [
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

    pub fn all_array() -> [Direction; 27] {
        Self::ALL_ARRAY
    }

    pub fn cardinal_array() -> [Direction; 4] {
        Self::CARDINAL_ARRAY
    }

    pub fn neighbor_array() -> [Direction; 26] {
        Self::NEIGHBOR_ARRAY
    }

    pub fn axis_array() -> [Direction; 3] {
        Self::AXIS_ARRAY
    }

    pub fn face_array() -> [Direction; 6] {
        Self::FACE_ARRAY
    }

    pub fn edge_array() -> [Direction; 12] {
        Self::EDGE_ARRAY
    }

    pub fn corner_array() -> [Direction; 8] {
        Self::CORNER_ARRAY
    }

    pub fn diagonal_array() -> [Direction; 12] {
        Self::DIAGONAL_ARRAY
    }

    pub fn traversable_array() -> [Direction; 12] {
        Self::TRAVERSABLE_ARRAY
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

    // pub fn cost(&self) -> f32 {
    //     match self {
    //         Direction::XnYnZn => MOVEMENT_COST_CORNER,
    //         Direction::XoYnZn => MOVEMENT_COST_EDGE,
    //         Direction::XpYnZn => MOVEMENT_COST_CORNER,
    //         Direction::XnYoZn => MOVEMENT_COST_EDGE,
    //         Direction::XoYoZn => MOVEMENT_COST_FACE,
    //         Direction::XpYoZn => MOVEMENT_COST_EDGE,
    //         Direction::XnYpZn => MOVEMENT_COST_CORNER,
    //         Direction::XoYpZn => MOVEMENT_COST_EDGE,
    //         Direction::XpYpZn => MOVEMENT_COST_CORNER,
    //         Direction::XnYnZo => MOVEMENT_COST_EDGE,
    //         Direction::XoYnZo => MOVEMENT_COST_FACE,
    //         Direction::XpYnZo => MOVEMENT_COST_EDGE,
    //         Direction::XnYoZo => MOVEMENT_COST_FACE,
    //         Direction::XoYoZo => MOVEMENT_COST_FACE,
    //         Direction::XpYoZo => MOVEMENT_COST_FACE,
    //         Direction::XnYpZo => MOVEMENT_COST_EDGE,
    //         Direction::XoYpZo => MOVEMENT_COST_FACE,
    //         Direction::XpYpZo => MOVEMENT_COST_EDGE,
    //         Direction::XnYnZp => MOVEMENT_COST_CORNER,
    //         Direction::XoYnZp => MOVEMENT_COST_EDGE,
    //         Direction::XpYnZp => MOVEMENT_COST_CORNER,
    //         Direction::XnYoZp => MOVEMENT_COST_EDGE,
    //         Direction::XoYoZp => MOVEMENT_COST_FACE,
    //         Direction::XpYoZp => MOVEMENT_COST_EDGE,
    //         Direction::XnYpZp => MOVEMENT_COST_CORNER,
    //         Direction::XoYpZp => MOVEMENT_COST_EDGE,
    //         Direction::XpYpZp => MOVEMENT_COST_CORNER,
    //     }
    // }

    pub fn axis_offset_array() -> [IVec3; 3] {
        Self::AXIS_ARRAY.map(|axis| axis.offset())
    }

    pub fn cardinal_offset_array() -> [IVec3; 4] {
        Self::CARDINAL_ARRAY.map(|cardinal| cardinal.offset())
    }

    pub fn neighbor_offset_array() -> [IVec3; 26] {
        Self::NEIGHBOR_ARRAY.map(|neighbor| neighbor.offset())
    }

    pub fn face_offset_array() -> [IVec3; 6] {
        Self::FACE_ARRAY.map(|face| face.offset())
    }

    pub fn edge_offset_array() -> [IVec3; 12] {
        Self::EDGE_ARRAY.map(|edge| edge.offset())
    }

    pub fn corner_offset_array() -> [IVec3; 8] {
        Self::CORNER_ARRAY.map(|corner| corner.offset())
    }

    pub fn from_components(x: i32, y: i32, z: i32) -> Option<Direction> {
        Self::ALL_ARRAY
            .iter()
            .copied()
            .find(|direction| direction.offset() == IVec3::new(x, y, z))
    }
}
