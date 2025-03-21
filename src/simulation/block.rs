use crate::simulation::BLOCK_RADIUS;
use bitflags::bitflags;
use glam::IVec3;
use serde::Deserialize;

pub const VERTEX_COUNT: u32 = 36;

pub type BlockID = usize;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    Air,
    Wood,
    Metal,
    Concrete,
    Plastic,
    Brick,
    Light,
    Marker1,
    Marker2,
    Black,
    White,
    Red,
    Blue,
    Gold,
    Beige,
    Green,
    Brown,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
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

    const FACE_INDICES: [usize; 6] = [4, 10, 12, 14, 16, 22];

    const EDGE_INDICES: [usize; 12] = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];

    const CORNER_INDICES: [usize; 8] = [0, 2, 6, 8, 18, 20, 24, 26];

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

    pub fn face_offsets() -> [IVec3; 6] {
        Self::FACE_INDICES.map(|index| Self::OFFSETS[index])
    }

    pub fn edge_offsets() -> [IVec3; 12] {
        Self::EDGE_INDICES.map(|index| Self::OFFSETS[index])
    }

    pub fn corner_offsets() -> [IVec3; 8] {
        Self::CORNER_INDICES.map(|index| Self::OFFSETS[index])
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Face: u8 {
        const XP = 1 << 0;
        const XN = 1 << 1;
        const YP = 1 << 2;
        const YN = 1 << 3;
        const ZP = 1 << 4;
        const ZN = 1 << 5;
    }
}

impl Face {
    const XP_QUAD: [(f32, f32, f32); 4] = [
        (BLOCK_RADIUS, -BLOCK_RADIUS, -BLOCK_RADIUS),
        (BLOCK_RADIUS, BLOCK_RADIUS, -BLOCK_RADIUS),
        (BLOCK_RADIUS, BLOCK_RADIUS, BLOCK_RADIUS),
        (BLOCK_RADIUS, -BLOCK_RADIUS, BLOCK_RADIUS),
    ];

    const XN_QUAD: [(f32, f32, f32); 4] = [
        (-BLOCK_RADIUS, -BLOCK_RADIUS, BLOCK_RADIUS),
        (-BLOCK_RADIUS, BLOCK_RADIUS, BLOCK_RADIUS),
        (-BLOCK_RADIUS, BLOCK_RADIUS, -BLOCK_RADIUS),
        (-BLOCK_RADIUS, -BLOCK_RADIUS, -BLOCK_RADIUS),
    ];

    const YP_QUAD: [(f32, f32, f32); 4] = [
        (-BLOCK_RADIUS, BLOCK_RADIUS, BLOCK_RADIUS),
        (BLOCK_RADIUS, BLOCK_RADIUS, BLOCK_RADIUS),
        (BLOCK_RADIUS, BLOCK_RADIUS, -BLOCK_RADIUS),
        (-BLOCK_RADIUS, BLOCK_RADIUS, -BLOCK_RADIUS),
    ];

    const YN_QUAD: [(f32, f32, f32); 4] = [
        (-BLOCK_RADIUS, -BLOCK_RADIUS, -BLOCK_RADIUS),
        (BLOCK_RADIUS, -BLOCK_RADIUS, -BLOCK_RADIUS),
        (BLOCK_RADIUS, -BLOCK_RADIUS, BLOCK_RADIUS),
        (-BLOCK_RADIUS, -BLOCK_RADIUS, BLOCK_RADIUS),
    ];

    const ZP_QUAD: [(f32, f32, f32); 4] = [
        (-BLOCK_RADIUS, -BLOCK_RADIUS, BLOCK_RADIUS),
        (BLOCK_RADIUS, -BLOCK_RADIUS, BLOCK_RADIUS),
        (BLOCK_RADIUS, BLOCK_RADIUS, BLOCK_RADIUS),
        (-BLOCK_RADIUS, BLOCK_RADIUS, BLOCK_RADIUS),
    ];

    const ZN_QUAD: [(f32, f32, f32); 4] = [
        (BLOCK_RADIUS, -BLOCK_RADIUS, -BLOCK_RADIUS),
        (-BLOCK_RADIUS, -BLOCK_RADIUS, -BLOCK_RADIUS),
        (-BLOCK_RADIUS, BLOCK_RADIUS, -BLOCK_RADIUS),
        (BLOCK_RADIUS, BLOCK_RADIUS, -BLOCK_RADIUS),
    ];

    pub const ALL: [Face; 6] = [Face::XP, Face::XN, Face::YP, Face::YN, Face::ZP, Face::ZN];

    pub fn quad_offsets(self) -> &'static [(f32, f32, f32); 4] {
        match self {
            Face::XP => &Self::XP_QUAD,
            Face::XN => &Self::XN_QUAD,
            Face::YP => &Self::YP_QUAD,
            Face::YN => &Self::YN_QUAD,
            Face::ZP => &Self::ZP_QUAD,
            Face::ZN => &Self::ZN_QUAD,
            _ => panic!("Invalid or multiple Face flags set: {:?}", self),
        }
    }

    pub fn normal(self) -> IVec3 {
        match self {
            Face::XP => IVec3::new(1, 0, 0),
            Face::XN => IVec3::new(-1, 0, 0),
            Face::YP => IVec3::new(0, 1, 0),
            Face::YN => IVec3::new(0, -1, 0),
            Face::ZP => IVec3::new(0, 0, 1),
            Face::ZN => IVec3::new(0, 0, -1),
            _ => panic!("Invalid or multiple Face flags set: {:?}", self),
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
    pub struct Neighbors: u32 {
        const NONE = 0;
    }
}

impl Neighbors {
    pub fn is_solid(&self, direction: Direction) -> bool {
        self.bits() & direction.bits() != 0
    }

    pub fn set_solid(&mut self, direction: Direction, solid: bool) {
        if solid {
            self.insert(Neighbors::from_bits_retain(self.bits() | direction.bits()));
        } else {
            self.remove(Neighbors::from_bits_retain(self.bits() & !direction.bits()));
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Meta {
    pub direction: Direction,
    pub visibility: Face,
    pub neighbors: Neighbors,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct LightLevel {
    pub received: u8,
    pub emitted: u8,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub emittance: u8,
    pub solid: bool,
    pub color: (f32, f32, f32, f32),
}
