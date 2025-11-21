//! Conversions between 2d and 3d space to 1d indices

pub use ultraviolet::{IVec2, IVec3};

pub struct Indexing {}

impl Indexing {
    pub fn to_ivec2(index: u32, radius: u32) -> IVec2 {
        let index = index as i32;
        
        let radius = radius as i32;
        let size = 2 * radius + 1;

        let x = index % size;
        let y = index / size;

        IVec2::new(x, y) - IVec2::broadcast(radius as i32)
    }

    pub fn from_ivec2(vector: IVec2, radius: u32) -> u32 {
        let vector_indexable = vector + IVec2::broadcast(radius as i32);

        let radius = radius as i32;
        let size = 2 * radius + 1;

        (vector_indexable.x + vector_indexable.y * size) as u32
    }

    pub fn to_ivec3(index: u32, radius: u32) -> IVec3 {
        let index = index as i32;

        let radius = radius as i32;
        let size = 2 * radius + 1;
        let area = size * size;

        let x = index % size;
        let y = index / size % size;
        let z = index / area;

        IVec3::new(x, y, z) - IVec3::broadcast(radius as i32)
    }

    pub fn from_ivec3(vector: IVec3, radius: u32) -> u32 {
        let vector_indexable = vector + IVec3::broadcast(radius as i32);

        let radius = radius as i32;
        let size = 2 * radius + 1;
        let area = size * size;

        (vector_indexable.x + vector_indexable.y * size + vector_indexable.z * area) as u32
    }
}