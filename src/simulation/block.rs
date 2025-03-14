use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialOrd, Ord, Eq, PartialEq)]
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

pub enum Cardinal {
    NN = 0,
    NE = 1,
    EE = 2,
    SE = 3,
    SS = 4,
    SW = 5,
    WW = 6,
    NW = 7,
}

#[derive(Clone, Copy, Debug)]
struct Direction(u8);

impl Direction {
    fn get_cardinal(self) -> Cardinal {
        match self.0 & 0b11 {
            0 => Cardinal::NN,
            1 => Cardinal::NE,
            2 => Cardinal::EE,
            3 => Cardinal::SE,
            4 => Cardinal::SS,
            5 => Cardinal::SW,
            6 => Cardinal::WW,
            7 => Cardinal::NW,
            _ => unreachable!(),
        }
    }

    fn rotate_cw(self) -> Direction {
        Direction((self.0 + 1) & 0b111)
    }

    fn rotate_ccw(self) -> Direction {
        Direction((self.0.wrapping_sub(1)) & 0b111)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct NeighborMask(u8);

impl NeighborMask {
    pub fn new(value: u8) -> Self {
        NeighborMask(value)
    }

    pub fn is_solid(&self, dir: usize) -> bool {
        (self.0 & (1 << dir)) != 0
    }

    pub fn set_solid(&mut self, dir: usize, solid: bool) {
        if solid {
            self.0 |= 1 << dir;
        } else {
            self.0 &= !(1 << dir);
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct Meta {
    pub direction: u8,
    pub neighbor_masks: [NeighborMask; 8],
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub solid: bool,
    pub color: (f32, f32, f32, f32),
}
