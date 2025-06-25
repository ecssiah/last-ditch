use glam::IVec3;

pub fn in_bounds(vector: IVec3, radius: u32) -> bool {
    let min = -(radius as i32);
    let max = radius as i32;

    (min..=max).contains(&vector.x)
        && (min..=max).contains(&vector.y)
        && (min..=max).contains(&vector.z)
}

pub fn indexable_vector(vector: IVec3, radius: u32) -> Option<IVec3> {
    if in_bounds(vector, radius) {
        Some(vector + IVec3::splat(radius as i32))
    } else {
        None
    }
}

pub fn index_to_vector(index: u32, radius: u32) -> IVec3 {
    let index = index as i32;

    let radius = radius as i32;
    let size = 2 * radius + 1;
    let area = size * size;

    let x = (index % size) - radius;
    let y = (index / size % size) - radius;
    let z = (index / area) - radius;

    IVec3::new(x, y, z)
}

pub fn vector_to_index(vector: IVec3, radius: u32) -> u32 {
    let vector = vector.as_uvec3();

    let size = 2 * radius + 1;
    let area = size * size;

    vector.x + vector.y * size + vector.z * area
}
