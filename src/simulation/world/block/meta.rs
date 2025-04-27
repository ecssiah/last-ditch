use crate::simulation::world::grid;

#[derive(Clone, Debug)]
pub struct Meta {
    pub direction: grid::Direction,
    pub visibility_direction_list: Vec<grid::Direction>,
    pub neighbor_direction_list: Vec<grid::Direction>,
}

impl Meta {
    pub fn new() -> Meta {
        let meta = Meta {
            direction: grid::Direction::XoYoZo,
            visibility_direction_list: Vec::new(),
            neighbor_direction_list: Vec::new(),
        };

        meta
    }
}
