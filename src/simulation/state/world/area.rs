use ultraviolet::IVec3;

#[derive(Clone)]
pub struct Area {
    pub area_id: u64,
    pub grid_position: IVec3,
    pub size: (usize, usize, usize),
}
