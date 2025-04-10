use crate::simulation::block;

#[derive(Clone, Debug)]
pub struct Meta {
    pub direction: block::Direction,
    pub visibility: Vec<block::Direction>,
    pub neighbors: Vec<block::Direction>,
}

impl Meta {
    pub fn new() -> Meta {
        let meta = Meta {
            direction: block::Direction::XoYoZo,
            visibility: Vec::new(),
            neighbors: Vec::new(),
        };

        meta
    }
}
