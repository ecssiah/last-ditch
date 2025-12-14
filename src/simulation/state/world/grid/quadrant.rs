#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Quadrant {
    NW,
    SW,
    SE,
    NE,
}

impl Quadrant {
    pub const ALL: [Self; 4] = [Self::NW, Self::SW, Self::SE, Self::NE];

    pub fn index(axis: Self) -> usize {
        axis as usize
    }
}
