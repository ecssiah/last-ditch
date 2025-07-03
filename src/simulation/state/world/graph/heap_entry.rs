use glam::IVec3;

#[derive(Eq, PartialEq)]
pub struct HeapEntry {
    pub cost: u32,
    pub position: IVec3,
}

impl HeapEntry {
    pub fn new(cost: u32, position: IVec3) -> Self {
        HeapEntry { cost, position }
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse to make BinaryHeap a min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.to_array().cmp(&other.position.to_array()))
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
