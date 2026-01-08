#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SectorIndex(usize);

impl SectorIndex {
    pub fn new(sector_index: usize) -> Self {
        Self(sector_index)
    }

    pub fn as_index(sector_index: &Self) -> usize {
        sector_index.0
    }
}

impl From<SectorIndex> for usize {
    fn from(sector_index: SectorIndex) -> Self {
        sector_index.0
    }
}

impl From<SectorIndex> for u32 {
    fn from(sector_index: SectorIndex) -> Self {
        sector_index.0 as u32
    }
}
