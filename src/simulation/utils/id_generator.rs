use crate::simulation::constants::ID_START;

#[derive(Debug)]
pub struct IDGenerator {
    current_id: u64,
}

impl IDGenerator {
    pub fn new() -> Self {
        Self {
            current_id: ID_START,
        }
    }

    #[inline]
    pub fn allocate(id_generator: &mut Self) -> u64 {
        let id = id_generator.current_id;

        id_generator.current_id += 1;

        id
    }

    #[inline]
    pub fn allocate_many(count: u64, id_generator: &mut Self) -> u64 {
        let start = id_generator.current_id;

        id_generator.current_id += count;

        start
    }
}
