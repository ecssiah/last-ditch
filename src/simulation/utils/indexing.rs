use glam::IVec3;

pub fn in_bounds(vector: IVec3, extent: u32) -> bool {
    let min = -(extent as i32);
    let max = extent as i32;

    (min..=max).contains(&vector.x)
        && (min..=max).contains(&vector.y)
        && (min..=max).contains(&vector.z)
}

pub fn indexable_vector(vector: IVec3, extent: u32) -> IVec3 {
    if in_bounds(vector, extent) {
        vector + IVec3::splat(extent as i32)
    } else {
        IVec3::MAX
    }
}

pub fn index_to_vector(index: u32, extent: u32) -> IVec3 {
    let index = index as i32;

    let extent = extent as i32;
    let size = 2 * extent + 1;
    let area = size * size;

    let x = (index % size) - extent;
    let y = (index / size % size) - extent;
    let z = (index / area) - extent;

    IVec3::new(x, y, z)
}

pub fn vector_to_index(vector: IVec3, extent: u32) -> u32 {
    let vector = vector.as_uvec3();

    let size = 2 * extent + 1;
    let area = size * size;

    vector.x + vector.y * size + vector.z * area
}
