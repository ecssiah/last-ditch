#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Quadrant {
    NE,
    NW,
    SW,
    SE,
}

impl Quadrant {
    pub const ALL: [Self; 4] = [Self::NE, Self::NW, Self::SW, Self::SE];

    pub fn index(axis: Self) -> usize {
        axis as usize
    }
}
