#[derive(Debug)]
pub struct IDGenerator {
    next_id: u64,
}

impl IDGenerator {
    pub fn new(initial_id: u64) -> Self {
        Self { next_id: initial_id }
    }

    #[inline]
    pub fn allocate(id_generator: &mut Self) -> u64 {
        let id = id_generator.next_id;

        id_generator.next_id += 1;

        id
    }

    #[inline]
    pub fn allocate_many(count: u64, id_generator: &mut Self) -> u64 {
        let start = id_generator.next_id;

        id_generator.next_id += count;
        
        start
    }
}