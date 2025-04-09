use bitflags::bitflags;
use glam::IVec3;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq)]
    pub struct Direction: u32 {
        const XN_YN_ZN = 1 << 00; const X0_YN_ZN = 1 << 01; const XP_YN_ZN = 1 << 02;
        const XN_Y0_ZN = 1 << 03; const X0_Y0_ZN = 1 << 04; const XP_Y0_ZN = 1 << 05;
        const XN_YP_ZN = 1 << 06; const X0_YP_ZN = 1 << 07; const XP_YP_ZN = 1 << 08;

        const XN_YN_Z0 = 1 << 09; const X0_YN_Z0 = 1 << 10; const XP_YN_Z0 = 1 << 11;
        const XN_Y0_Z0 = 1 << 12; const X0_Y0_Z0 = 1 << 13; const XP_Y0_Z0 = 1 << 14;
        const XN_YP_Z0 = 1 << 15; const X0_YP_Z0 = 1 << 16; const XP_YP_Z0 = 1 << 17;

        const XN_YN_ZP = 1 << 18; const X0_YN_ZP = 1 << 19; const XP_YN_ZP = 1 << 20;
        const XN_Y0_ZP = 1 << 21; const X0_Y0_ZP = 1 << 22; const XP_Y0_ZP = 1 << 23;
        const XN_YP_ZP = 1 << 24; const X0_YP_ZP = 1 << 25; const XP_YP_ZP = 1 << 26;

        const XP = Self::XP_Y0_Z0.bits();
        const XN = Self::XN_Y0_Z0.bits();
        const YP = Self::X0_YP_Z0.bits();
        const YN = Self::X0_YN_Z0.bits();
        const ZP = Self::X0_Y0_ZP.bits();
        const ZN = Self::X0_Y0_ZN.bits();

        const ORIGIN = Self::X0_Y0_Z0.bits();

        const EAST = Self::XP_Y0_Z0.bits();
        const WEST = Self::XN_Y0_Z0.bits();
        const UP = Self::X0_YP_Z0.bits();
        const DOWN = Self::X0_YN_Z0.bits();
        const NORTH = Self::X0_Y0_ZP.bits();
        const SOUTH = Self::X0_Y0_ZN.bits();
    }
}

impl Direction {
    #[rustfmt::skip]
    const OFFSETS: [IVec3; 27] = [
        IVec3::new(-1, -1, -1), IVec3::new(0, -1, -1), IVec3::new(1, -1, -1),
        IVec3::new(-1,  0, -1), IVec3::new(0,  0, -1), IVec3::new(1,  0, -1),
        IVec3::new(-1,  1, -1), IVec3::new(0,  1, -1), IVec3::new(1,  1, -1),

        IVec3::new(-1, -1,  0), IVec3::new(0, -1,  0), IVec3::new(1, -1,  0),
        IVec3::new(-1,  0,  0), IVec3::new(0,  0,  0), IVec3::new(1,  0,  0),
        IVec3::new(-1,  1,  0), IVec3::new(0,  1,  0), IVec3::new(1,  1,  0),

        IVec3::new(-1, -1,  1), IVec3::new(0, -1,  1), IVec3::new(1, -1,  1),
        IVec3::new(-1,  0,  1), IVec3::new(0,  0,  1), IVec3::new(1,  0,  1),
        IVec3::new(-1,  1,  1), IVec3::new(0,  1,  1), IVec3::new(1,  1,  1),
    ];

    const FACES: [Direction; 6] = [
        Direction::XP_Y0_Z0,
        Direction::XN_Y0_Z0,
        Direction::X0_YP_Z0,
        Direction::X0_YN_Z0,
        Direction::X0_Y0_ZP,
        Direction::X0_Y0_ZN,
    ];

    const EDGES: [Direction; 12] = [
        Direction::X0_YN_ZN,
        Direction::XN_Y0_ZN,
        Direction::XP_Y0_ZN,
        Direction::X0_YP_ZN,
        Direction::XN_YN_Z0,
        Direction::XP_YN_Z0,
        Direction::XN_YP_Z0,
        Direction::XP_YP_Z0,
        Direction::X0_YN_ZP,
        Direction::XN_Y0_ZP,
        Direction::XP_Y0_ZP,
        Direction::X0_YP_ZP,
    ];

    const CORNERS: [Direction; 8] = [
        Direction::XN_YN_ZN,
        Direction::XP_YN_ZN,
        Direction::XN_YP_ZN,
        Direction::XP_YP_ZN,
        Direction::XN_YN_ZP,
        Direction::XP_YN_ZP,
        Direction::XN_YP_ZP,
        Direction::XP_YP_ZP,
    ];

    pub fn bit(index: usize) -> Option<Self> {
        Self::from_bits(1 << index)
    }

    pub fn index(self) -> usize {
        self.bits().trailing_zeros() as usize
    }

    pub fn offset(self) -> IVec3 {
        Self::OFFSETS[self.index()]
    }

    pub fn get_offset(index: usize) -> Option<IVec3> {
        Self::OFFSETS.get(index).copied()
    }

    pub fn offsets() -> [IVec3; 27] {
        Self::OFFSETS
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
