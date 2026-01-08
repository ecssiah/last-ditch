#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellIndex(usize);

impl CellIndex {
    pub fn new(cell_index: usize) -> Self {
        Self(cell_index)
    }

    pub fn as_index(cell_index: &Self) -> usize {
        cell_index.0
    }
}

impl From<CellIndex> for usize {
    fn from(cell_index: CellIndex) -> Self {
        cell_index.0
    }
}

impl From<CellIndex> for u32 {
    fn from(cell_index: CellIndex) -> Self {
        cell_index.0 as u32
    }
}
