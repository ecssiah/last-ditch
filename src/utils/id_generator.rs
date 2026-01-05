#[derive(Debug)]
pub struct IDGenerator {
    current_id_value: u32,
}

impl IDGenerator {
    pub fn new() -> Self {
        Self {
            current_id_value: 100,
        }
    }

    #[inline]
    pub fn allocate(id_generator: &mut Self) -> u32 {
        let id_value = id_generator.current_id_value;

        id_generator.current_id_value += 1;

        id_value
    }
}
