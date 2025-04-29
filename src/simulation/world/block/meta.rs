use crate::simulation::world::grid;

#[derive(Clone, Debug)]
pub struct Meta {
    pub direction: grid::Direction,
    pub visibility_list: Vec<grid::Direction>,
    pub neighbor_list: Vec<grid::Direction>,
}

impl Meta {
    pub fn new() -> Meta {
        let meta = Meta {
            direction: grid::Direction::XoYoZo,
            visibility_list: Vec::new(),
            neighbor_list: Vec::new(),
        };

        meta
    }
}
