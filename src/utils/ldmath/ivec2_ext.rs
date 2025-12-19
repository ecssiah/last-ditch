use ultraviolet::IVec2;

#[inline]
pub fn index_to_ivec2(index: usize, radius: usize) -> IVec2 {
    let index = index as i32;

    let radius = radius as i32;
    let size = 2 * radius + 1;

    let x = index % size;
    let y = index / size;

    IVec2::new(x, y) - IVec2::broadcast(radius as i32)
}

#[inline]
pub fn ivec2_to_index(ivec2: IVec2, radius: usize) -> usize {
    let ivec2_indexable = ivec2 + IVec2::broadcast(radius as i32);

    let radius = radius as i32;
    let size = 2 * radius + 1;

    (ivec2_indexable.x + ivec2_indexable.y * size) as usize
}

