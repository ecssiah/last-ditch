//! Conversions between 2d and 3d space to 1d indices

pub use ultraviolet::{IVec2, IVec3};

#[inline]
pub fn to_ivec2(index: usize, radius: usize) -> IVec2 {
    let index = index as i32;

    let radius = radius as i32;
    let size = 2 * radius + 1;

    let x = index % size;
    let y = index / size;

    IVec2::new(x, y) - IVec2::broadcast(radius as i32)
}

#[inline]
pub fn from_ivec2(vector: IVec2, radius: usize) -> usize {
    let vector_indexable = vector + IVec2::broadcast(radius as i32);

    let radius = radius as i32;
    let size = 2 * radius + 1;

    (vector_indexable.x + vector_indexable.y * size) as usize
}

#[inline]
pub fn to_ivec3(index: usize, radius: usize) -> IVec3 {
    let index = index as i32;

    let radius = radius as i32;
    let size = 2 * radius + 1;
    let area = size * size;

    let x = index % size;
    let y = index / size % size;
    let z = index / area;

    IVec3::new(x, y, z) - IVec3::broadcast(radius as i32)
}

#[inline]
pub fn from_ivec3(vector: IVec3, radius: usize) -> usize {
    let vector_indexable = vector + IVec3::broadcast(radius as i32);

    let radius = radius as i32;
    let size = 2 * radius + 1;
    let area = size * size;

    (vector_indexable.x + vector_indexable.y * size + vector_indexable.z * area) as usize
}
