use bitflags::bitflags;
use glam::IVec3;
use serde::Deserialize;

pub const BLOCK_VERTEX_COUNT: u32 = 36;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    Air,
    Wood,
    Metal,
    Concrete,
    Black,
    White,
    Red,
    Blue,
    Gold,
    Skin,
    Green,
    Brown,
    Light,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
    pub struct Neighbors: u32 {
        // South -Z
        const SED = 1 << 0;  // (-X, -Y, -Z) South East Down
        const SCD = 1 << 1;  // ( 0, -Y, -Z) South Center Down
        const SWD = 1 << 2;  // (+X, -Y, -Z) South West Down
        const SEC = 1 << 3;  // (-X,  0, -Z) South East Center
        const SCC = 1 << 4;  // ( 0,  0, -Z) South Center Center
        const SWC = 1 << 5;  // (+X,  0, -Z) South West Center
        const SEU = 1 << 6;  // (-X, +Y, -Z) South East Up
        const SCU = 1 << 7;  // ( 0, +Y, -Z) South Center Up
        const SWU = 1 << 8;  // (+X, +Y, -Z) South West Up

        // Center
        const CED = 1 << 9;  // (-X, -Y,  0) Center East Down
        const CCD = 1 << 10; // ( 0, -Y,  0) Center Center Down
        const CWD = 1 << 11; // (+X, -Y,  0) Center West Down
        const CEC = 1 << 12; // (-X,  0,  0) Center East Center
        const CCC = 1 << 13; // ( 0,  0,  0) Center
        const CWC = 1 << 14; // (+X,  0,  0) Center West Center
        const CEU = 1 << 15; // (-X, +Y,  0) Center East Up
        const CCU = 1 << 16; // ( 0, +Y,  0) Center Center Up
        const CWU = 1 << 17; // (+X, +Y,  0) Center West Up

        // North +Z
        const NED = 1 << 18; // (-X, -Y, +Z) North East Down
        const NCD = 1 << 19; // ( 0, -Y, +Z) North Center Down
        const NWD = 1 << 20; // (+X, -Y, +Z) North West Down
        const NEC = 1 << 21; // (-X,  0, +Z) North East Center
        const NCC = 1 << 22; // ( 0,  0, +Z) North Center Center
        const NWC = 1 << 23; // (+X,  0, +Z) North West Center
        const NEU = 1 << 24; // (-X, +Y, +Z) North East Up
        const NCU = 1 << 25; // ( 0, +Y, +Z) North Center Up
        const NWU = 1 << 26; // (+X, +Y, +Z) North West Up
    }
}

#[rustfmt::skip]
pub const OFFSETS: [IVec3; 27] = [
    // South -Z
    IVec3::new(-1, -1, -1), IVec3::new(0, -1, -1), IVec3::new(1, -1, -1),
    IVec3::new(-1,  0, -1), IVec3::new(0,  0, -1), IVec3::new(1,  0, -1),
    IVec3::new(-1,  1, -1), IVec3::new(0,  1, -1), IVec3::new(1,  1, -1),

    // Center
    IVec3::new(-1, -1,  0), IVec3::new(0, -1,  0), IVec3::new(1, -1,  0),
    IVec3::new(-1,  0,  0), IVec3::new(0,  0,  0), IVec3::new(1,  0,  0),
    IVec3::new(-1,  1,  0), IVec3::new(0,  1,  0), IVec3::new(1,  1,  0),

    // North +Z
    IVec3::new(-1, -1,  1), IVec3::new(0, -1,  1), IVec3::new(1, -1,  1),
    IVec3::new(-1,  0,  1), IVec3::new(0,  0,  1), IVec3::new(1,  0,  1),
    IVec3::new(-1,  1,  1), IVec3::new(0,  1,  1), IVec3::new(1,  1,  1),
];

pub const FACE_OFFSET_INDICES: [usize; 6] = [4, 10, 12, 14, 16, 22];

pub const EDGE_OFFSET_INDICES: [usize; 12] = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];

pub const CORNER_OFFSET_INDICES: [usize; 8] = [0, 2, 6, 8, 18, 20, 24, 26];

impl Neighbors {
    pub fn is_solid(&self, neighbor: Neighbors) -> bool {
        self.contains(neighbor)
    }

    pub fn set_solid(&mut self, neighbor: Neighbors, solid: bool) {
        if solid {
            self.insert(neighbor);
        } else {
            self.remove(neighbor);
        }
    }

    pub fn bit(index: usize) -> Option<Neighbors> {
        Self::from_bits(1 << index)
    }

    pub fn index(self) -> usize {
        self.bits().trailing_zeros() as usize
    }

    pub fn offset(self) -> IVec3 {
        OFFSETS[self.index()]
    }

    pub fn face_offsets() -> [IVec3; 6] {
        FACE_OFFSET_INDICES.map(|i| OFFSETS[i])
    }

    pub fn edge_offsets() -> [IVec3; 12] {
        EDGE_OFFSET_INDICES.map(|i| OFFSETS[i])
    }

    pub fn corner_offsets() -> [IVec3; 8] {
        CORNER_OFFSET_INDICES.map(|i| OFFSETS[i])
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Meta {
    pub direction: u8,
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
