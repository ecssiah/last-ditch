use ultraviolet::IVec3;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    XNYNZN, XOYNZN, XPYNZN,
    XNYOZN, XOYOZN, XPYOZN,
    XNYPZN, XOYPZN, XPYPZN,

    XNYNZO, XOYNZO, XPYNZO,
    XNYOZO, XOYOZO, XPYOZO,
    XNYPZO, XOYPZO, XPYPZO,

    XNYNZP, XOYNZP, XPYNZP,
    XNYOZP, XOYOZP, XPYOZP,
    XNYPZP, XOYPZP, XPYPZP,
}

impl Direction {
    #[rustfmt::skip]
    const ALL_ARRAY: [Direction; 27] = [
        Direction::XNYNZN,
        Direction::XOYNZN,
        Direction::XPYNZN,
        Direction::XNYOZN,
        Direction::XOYOZN,
        Direction::XPYOZN,
        Direction::XNYPZN,
        Direction::XOYPZN,
        Direction::XPYPZN,
        Direction::XNYNZO,
        Direction::XOYNZO,
        Direction::XPYNZO,
        Direction::XNYOZO,
        Direction::XOYOZO,
        Direction::XPYOZO,
        Direction::XNYPZO,
        Direction::XOYPZO,
        Direction::XPYPZO,
        Direction::XNYNZP,
        Direction::XOYNZP,
        Direction::XPYNZP,
        Direction::XNYOZP,
        Direction::XOYOZP,
        Direction::XPYOZP,
        Direction::XNYPZP,
        Direction::XOYPZP,
        Direction::XPYPZP,
    ];

    #[rustfmt::skip]
    const CARDINAL_ARRAY: [Direction; 4] = [
        Direction::XPYOZO,
        Direction::XNYOZO,
        Direction::XOYPZO,
        Direction::XOYNZO,
    ];

    #[rustfmt::skip]
    const NEIGHBOR_ARRAY: [Direction; 26] = [
        Direction::XNYNZN,
        Direction::XOYNZN,
        Direction::XPYNZN,
        Direction::XNYOZN,
        Direction::XOYOZN,
        Direction::XPYOZN,
        Direction::XNYPZN,
        Direction::XOYPZN,
        Direction::XPYPZN,
        Direction::XNYNZO,
        Direction::XOYNZO,
        Direction::XPYNZO,
        Direction::XNYOZO,
        Direction::XPYOZO,
        Direction::XNYPZO,
        Direction::XOYPZO,
        Direction::XPYPZO,
        Direction::XNYNZP,
        Direction::XOYNZP,
        Direction::XPYNZP,
        Direction::XNYOZP,
        Direction::XOYOZP,
        Direction::XPYOZP,
        Direction::XNYPZP,
        Direction::XOYPZP,
        Direction::XPYPZP,
    ];

    #[rustfmt::skip]
    const AXIS_ARRAY: [Direction; 3] = [
        Direction::XPYOZO,
        Direction::XOYPZO,
        Direction::XOYOZP,
    ];

    #[rustfmt::skip]
    const FACE_ARRAY: [Direction; 6] = [
        Direction::XPYOZO,
        Direction::XNYOZO,
        Direction::XOYPZO,
        Direction::XOYNZO,
        Direction::XOYOZP,
        Direction::XOYOZN,
    ];

    #[rustfmt::skip]
    const EDGE_ARRAY: [Direction; 12] = [
        Direction::XOYNZN,
        Direction::XNYOZN,
        Direction::XPYOZN,
        Direction::XOYPZN,
        Direction::XNYNZO,
        Direction::XPYNZO,
        Direction::XNYPZO,
        Direction::XPYPZO,
        Direction::XOYNZP,
        Direction::XNYOZP,
        Direction::XPYOZP,
        Direction::XOYPZP,
    ];

    #[rustfmt::skip]
    const CORNER_ARRAY: [Direction; 8] = [
        Direction::XNYNZN,
        Direction::XPYNZN,
        Direction::XNYPZN,
        Direction::XPYPZN,
        Direction::XNYNZP,
        Direction::XPYNZP,
        Direction::XNYPZP,
        Direction::XPYPZP,
    ];

    #[rustfmt::skip]
    const DIAGONAL_ARRAY: [Direction; 12] = [
        Direction::XNYOZN,
        Direction::XPYOZN,
        Direction::XNYOZP,
        Direction::XPYOZP,
        Direction::XNYNZN,
        Direction::XPYNZN,
        Direction::XNYPZN,
        Direction::XPYPZN,
        Direction::XNYNZP,
        Direction::XPYNZP,
        Direction::XNYPZP,
        Direction::XPYPZP,
    ];

    #[rustfmt::skip]
    const TRAVERSABLE_ARRAY: [Direction; 12] = [
        Direction::XNYPZO,
        Direction::XPYPZO,
        Direction::XOYPZP,
        Direction::XOYPZN,
        Direction::XNYOZO,
        Direction::XPYOZO,
        Direction::XOYOZP,
        Direction::XOYOZN,
        Direction::XNYNZO,
        Direction::XPYNZO,
        Direction::XOYNZP,
        Direction::XOYNZN,
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
            Direction::XNYNZN => IVec3::new(-1, -1, -1),
            Direction::XOYNZN => IVec3::new(0, -1, -1),
            Direction::XPYNZN => IVec3::new(1, -1, -1),
            Direction::XNYOZN => IVec3::new(-1, 0, -1),
            Direction::XOYOZN => IVec3::new(0, 0, -1),
            Direction::XPYOZN => IVec3::new(1, 0, -1),
            Direction::XNYPZN => IVec3::new(-1, 1, -1),
            Direction::XOYPZN => IVec3::new(0, 1, -1),
            Direction::XPYPZN => IVec3::new(1, 1, -1),
            Direction::XNYNZO => IVec3::new(-1, -1, 0),
            Direction::XOYNZO => IVec3::new(0, -1, 0),
            Direction::XPYNZO => IVec3::new(1, -1, 0),
            Direction::XNYOZO => IVec3::new(-1, 0, 0),
            Direction::XOYOZO => IVec3::new(0, 0, 0),
            Direction::XPYOZO => IVec3::new(1, 0, 0),
            Direction::XNYPZO => IVec3::new(-1, 1, 0),
            Direction::XOYPZO => IVec3::new(0, 1, 0),
            Direction::XPYPZO => IVec3::new(1, 1, 0),
            Direction::XNYNZP => IVec3::new(-1, -1, 1),
            Direction::XOYNZP => IVec3::new(0, -1, 1),
            Direction::XPYNZP => IVec3::new(1, -1, 1),
            Direction::XNYOZP => IVec3::new(-1, 0, 1),
            Direction::XOYOZP => IVec3::new(0, 0, 1),
            Direction::XPYOZP => IVec3::new(1, 0, 1),
            Direction::XNYPZP => IVec3::new(-1, 1, 1),
            Direction::XOYPZP => IVec3::new(0, 1, 1),
            Direction::XPYPZP => IVec3::new(1, 1, 1),
        }
    }

    // pub fn cost(&self) -> f32 {
    //     match self {
    //         Direction::XNYNZN => MOVEMENT_COST_CORNER,
    //         Direction::XOYNZN => MOVEMENT_COST_EDGE,
    //         Direction::XPYNZN => MOVEMENT_COST_CORNER,
    //         Direction::XNYOZN => MOVEMENT_COST_EDGE,
    //         Direction::XOYOZN => MOVEMENT_COST_FACE,
    //         Direction::XPYOZN => MOVEMENT_COST_EDGE,
    //         Direction::XNYPZN => MOVEMENT_COST_CORNER,
    //         Direction::XOYPZN => MOVEMENT_COST_EDGE,
    //         Direction::XPYPZN => MOVEMENT_COST_CORNER,
    //         Direction::XNYNZO => MOVEMENT_COST_EDGE,
    //         Direction::XOYNZO => MOVEMENT_COST_FACE,
    //         Direction::XPYNZO => MOVEMENT_COST_EDGE,
    //         Direction::XNYOZO => MOVEMENT_COST_FACE,
    //         Direction::XOYOZO => MOVEMENT_COST_FACE,
    //         Direction::XPYOZO => MOVEMENT_COST_FACE,
    //         Direction::XNYPZO => MOVEMENT_COST_EDGE,
    //         Direction::XOYPZO => MOVEMENT_COST_FACE,
    //         Direction::XPYPZO => MOVEMENT_COST_EDGE,
    //         Direction::XNYNZP => MOVEMENT_COST_CORNER,
    //         Direction::XOYNZP => MOVEMENT_COST_EDGE,
    //         Direction::XPYNZP => MOVEMENT_COST_CORNER,
    //         Direction::XNYOZP => MOVEMENT_COST_EDGE,
    //         Direction::XOYOZP => MOVEMENT_COST_FACE,
    //         Direction::XPYOZP => MOVEMENT_COST_EDGE,
    //         Direction::XNYPZP => MOVEMENT_COST_CORNER,
    //         Direction::XOYPZP => MOVEMENT_COST_EDGE,
    //         Direction::XPYPZP => MOVEMENT_COST_CORNER,
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
