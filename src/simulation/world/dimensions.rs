pub struct Dimensions {
    pub radius: i32,
    pub size: i32,
    pub area: i32,
    pub volume: i32,
    pub chunk_radius: i32,
    pub chunk_size: i32,
    pub chunk_area: i32,
    pub chunk_volume: i32,
}

impl Dimensions {
    pub fn new(radius: i32, chunk_radius: i32) -> Dimensions {
        let size = 2 * radius + 1;
        let area = size * size;
        let volume = size * size * size;

        let chunk_size = 2 * chunk_radius + 1;
        let chunk_area = chunk_size * chunk_size;
        let chunk_volume = chunk_size * chunk_size * chunk_size;

        let dimensions = Dimensions {
            radius,
            size,
            area,
            volume,
            chunk_radius,
            chunk_size,
            chunk_area,
            chunk_volume,
        };

        dimensions
    }
}
