use crate::simulation::block;

#[derive(Clone, Debug)]
pub struct Meta {
    pub direction: block::Direction,
    pub visibility_direction_list: Vec<block::Direction>,
    pub neighbor_direction_list: Vec<block::Direction>,
}

impl Meta {
    pub fn new() -> Meta {
        let meta = Meta {
            direction: block::Direction::XoYoZo,
            visibility_direction_list: Vec::new(),
            neighbor_direction_list: Vec::new(),
        };

        meta
    }
}
