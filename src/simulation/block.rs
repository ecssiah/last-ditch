use glam::IVec3;
use serde::Deserialize;

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DirectionBit {
    XPos = 0b00000001,
    XNeg = 0b00000010,
    YPos = 0b00000100,
    YNeg = 0b00001000,
    ZPos = 0b00010000,
    ZNeg = 0b00100000,
}

pub const NEIGHBORS: [(DirectionBit, IVec3); 6] = [
    (DirectionBit::XPos, IVec3::new(1, 0, 0)),
    (DirectionBit::XNeg, IVec3::new(-1, 0, 0)),
    (DirectionBit::YPos, IVec3::new(0, 1, 0)),
    (DirectionBit::YNeg, IVec3::new(0, -1, 0)),
    (DirectionBit::ZPos, IVec3::new(0, 0, 1)),
    (DirectionBit::ZNeg, IVec3::new(0, 0, -1)),
];

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct NeighborMask(u8);

impl NeighborMask {
    pub fn new() -> Self {
        NeighborMask(0)
    }

    pub fn get_value(&self) -> u8 {
        self.0
    }

    pub fn is_solid(&self, direction_bit: DirectionBit) -> bool {
        (self.0 & (direction_bit as u8)) != 0
    }

    pub fn set_solid(&mut self, direction_bit: DirectionBit, solid: bool) {
        if solid {
            self.0 |= direction_bit as u8;
        } else {
            self.0 &= !(direction_bit as u8);
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Meta {
    pub direction: u8,
    pub neighbor_mask: NeighborMask,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub solid: bool,
    pub color: (f32, f32, f32, f32),
}

impl Block {
    pub fn get_bit(target_offset: IVec3) -> Option<DirectionBit> {
        for (direction_bit, offset) in NEIGHBORS {
            if target_offset == offset {
                return Some(direction_bit);
            }
        }

        None 
    }

    pub fn get_offset(target_direction_bit: DirectionBit) -> Option<IVec3> {
        for (direction_bit, offset) in NEIGHBORS {
            if target_direction_bit == direction_bit {
                return Some(offset);
            }
        }

        None 
    }
}
