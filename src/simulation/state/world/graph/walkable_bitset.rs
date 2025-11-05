#[derive(Clone)]
pub struct WalkableBitset {
    pub extent: usize,
    pub size: usize,
    pub bit_vec: Vec<u64>,
}

impl WalkableBitset {
    pub fn new(extent: usize, size: usize) -> Self {
        Self {
            extent,
            size,
            bit_vec: Vec::from_iter(std::iter::repeat(0_u64).take(size * size * size)),
        }
    }

    pub fn set_walkable(
        x: i32,
        y: i32,
        z: i32,
        is_walkable: bool,
        walkable_bitset: &mut WalkableBitset,
    ) {
        let extent = walkable_bitset.extent as i32;
        let size = walkable_bitset.size as i32;

        let index = (x + extent) + size * (y + extent) + size * size * (z + extent);

        let word_index = (index / 64) as usize;
        let bit_index = (index % 64) as u32;

        if is_walkable {
            walkable_bitset.bit_vec[word_index] |= 1u64 << bit_index;
        } else {
            walkable_bitset.bit_vec[word_index] &= !(1u64 << bit_index);
        }
    }

    pub fn is_walkable(x: i32, y: i32, z: i32, walkable_bitset: &WalkableBitset) -> bool {
        let extent = walkable_bitset.extent as i32;
        let size = walkable_bitset.size as i32;

        let index = (x + extent) + size * (y + extent) + size * size * (z + extent);

        let word_index = (index / 64) as usize;
        let bit_index = (index % 64) as u32;

        let word = walkable_bitset.bit_vec[word_index];
        (word & (1u64 << bit_index)) != 0
    }
}
