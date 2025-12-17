#[derive(Clone, Copy, Debug)]
pub enum Owner {
    None,
    Block {
        block_id: u64,
    },
    Object {
        object_id: u64,
    },
    Entity {
        entity_id: u64,
    },
}